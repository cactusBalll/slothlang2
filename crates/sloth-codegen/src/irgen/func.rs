//! Pass 2 core: emit a single function body.

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use sloth_frontend::ast::*;
#[allow(unused_imports)]
use sloth_frontend::lexer::{Pos, StrPart};
#[allow(unused_imports)]
use sloth_frontend::ty::{Diag, FnTy, LamMeta, Reg, Ty, TyId};
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};

impl ModEmitter {
    /// emit one function; returns mangled symbol name
    pub(crate) fn emit_func(
        &mut self,
        name: &str,
        cls: Option<&str>,
        f: &FuncDef,
        variadic: Option<&Variadic>,
        entry: bool,
    ) -> String {
        let plan = self.plan_func(name, cls, f, variadic);
        if self.emitted_names.contains(&plan.mangled) {
            return plan.mangled;
        }
        // extern func: body-less declaration kept under its raw C-ABI name
        if f.is_extern {
            self.stat_extdecls += 1;
            self.emitted_names.push(name.to_string());
            self.out.push_str(&format!(
                "  func.func private @{}({}) -> {}\n",
                name,
                plan.params
                    .iter()
                    .map(|p| if self.is_float(p.1) { "f64" } else { "i64" })
                    .collect::<Vec<&str>>()
                    .join(", "),
                mlir_ret_ty(self, plan.ret),
            ));
            return plan.mangled;
        }
        // methods addressable by vtable slots are emitted as llvm.func
        let is_ll = cls
            .map(|c| {
                self.llvm_method
                    .contains(&(c.to_string(), name.to_string()))
            })
            .unwrap_or(false);
        self.emitted_names.push(plan.mangled.clone());
        let retf = self.is_float(plan.ret);
        let mut fw = FnWalk {
            cur: String::new(),
            vcount: 1000,
            scopes: vec![HashMap::new()],
            imms: vec![HashMap::new()],
            scope_decls: vec![HashMap::new()],
            dangling: Vec::new(),
            loops: Vec::new(),
            loopvars: Vec::new(),
            xfer: Vec::new(),
            ret: plan.ret,
            ret_alloca: String::new(),
            ret_flag: String::new(),
            bb: 0,
            term: false,
            end_label: "^end".to_string(),
            cur_cls: None,
        };
        fw.cur_cls = cls.map(|c| c.to_string());
        let rf = fw.v();
        fw.op(&format!("    {} = memref.alloca() : memref<1xi64>", rf));
        fw.ret_flag = rf;
        if !self.is_unit(plan.ret) {
            let ra = fw.v();
            let rty = if retf {
                "memref<1xf64>"
            } else {
                "memref<1xi64>"
            };
            fw.ret_alloca = ra.clone();
            fw.op(&format!("    {} = memref.alloca() : {}", ra, rty));
        }
        // function params: %pN fed through allocas; variadic packs later
        let mut argtxts: Vec<String> = Vec::new();
        for (i, (pn, pt, fl)) in plan.params.iter().enumerate() {
            let src = format!("%p{}", i);
            argtxts.push(if *fl {
                "f64".to_string()
            } else {
                "i64".to_string()
            });
            let mty = if *fl {
                "memref<1xf64>"
            } else {
                "memref<1xi64>"
            };
            let a = fw.v();
            let zi = fw.v();
            fw.op(&format!("    {} = arith.constant 0 : i64", zi));
            fw.op(&format!("    {} = memref.alloca() : {}", a, mty));
            fw.op(&format!(
                "    memref.store {}, {}[{}] : {}",
                src, a, zi, mty
            ));
            fw.scopes.last_mut().unwrap().insert(pn.clone(), (a, *pt));
        }
        // emit body statements into the entry block
        self.walk_body(&mut fw, &f.body);
        fw.jump(&"^end");
        let entry_text = fw.cur.clone();
        fw.cur = String::new();
        fw.term = false;
        fw.label(&"^end");
        let mut retval = String::new();
        if !self.is_unit(plan.ret) {
            let zi = fw.v();
            let v = fw.v();
            let rty = if retf {
                "memref<1xf64>"
            } else {
                "memref<1xi64>"
            };
            fw.op(&format!("    {} = arith.constant 0 : index", zi));
            fw.op(&format!(
                "    {} = memref.load {}[{}] : {}",
                v, fw.ret_alloca, zi, rty
            ));
            retval = format!(" {}", v);
        }
        if is_ll {
            if self.is_unit(plan.ret) {
                fw.op("    llvm.return");
            } else {
                let rt = if retf { "f64" } else { "i64" };
                fw.op(&format!("    llvm.return {} : {}", retval.trim_start(), rt));
            }
        } else if self.is_unit(plan.ret) {
            fw.op("    return");
        } else {
            let rt = if retf { "f64" } else { "i64" };
            fw.op(&format!("    return {} : {}", retval.trim_start(), rt));
        }
        let ret_text = fw.cur.clone();
        let sigtxt: Vec<String> = argtxts
            .iter()
            .enumerate()
            .map(|(i, t)| format!("%p{}: {}", i, t))
            .collect();
        let sigtxt = sigtxt.join(", ");
        // entry (declared main or script): imported modules' var inits first
        let entry_text = if entry && !self.init_mods.is_empty() {
            let mut pre = String::new();
            for m in self.init_mods.clone() {
                pre.push_str(&format!("    call @sloth_{}__ginit() : () -> ()\n", m));
            }
            format!("{}{}", pre, entry_text)
        } else {
            entry_text
        };
        if entry {
            self.out.push_str(&format!(
                "  func.func @sloth_main({}) -> {} attributes {{llvm.emit_c_interface}} {{\n",
                sigtxt,
                mlir_ret_ty(self, plan.ret),
            ));
        } else if is_ll {
            self.out.push_str(&format!(
                "  llvm.func @{}({}) -> {} {{\n",
                plan.mangled,
                sigtxt,
                mlir_ret_ty(self, plan.ret)
            ));
        } else {
            self.out.push_str(&format!(
                "  func.func @{}({}) -> {} {{\n",
                plan.mangled,
                sigtxt,
                mlir_ret_ty(self, plan.ret)
            ));
        }
        // bare `call` is func-dialect sugar valid only in func.func regions;
        // inside llvm.func bodies it must be spelled func.call
        let entry_text = if is_ll {
            rename_plain_calls(&entry_text)
        } else {
            entry_text
        };
        let ret_text = if is_ll {
            rename_plain_calls(&ret_text)
        } else {
            ret_text
        };
        self.out.push_str(&entry_text);
        self.out.push_str(&ret_text);
        self.out.push_str("  }\n");
        plan.mangled
    }
}
