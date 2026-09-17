// Isolate ARC leaks per language construct via sloth_rc_live deltas.
trait Node { func tag(): int; }
class NNull impl Node { func tag(): int { return 0; } }
class NInt impl Node {
    var v: int;
    func __init__(v: int) { this.v = v; }
    func tag(): int { return this.v; }
}
class Box {
    var v: int;
    func __init__(v: int) { this.v = v; }
}
func f_int(): int { return 42; }
func f_str_lit(): str { return "abc"; }
func f_str_interp(x: int): str { return "v${x}"; }
func f_str_cat(x: int): str { return "a" + "b" + "${x}"; }
func f_arr(): Array<int> { var a: Array<int> = []; a.push(1); a.push(2); return a; }
func f_arr_ref(): Array<dyn Node> { var a: Array<dyn Node> = []; a.push(NNull()); a.push(NInt(3)); return a; }
func f_dyn(c: int): dyn Node { if c == 0 { return NNull(); } return NInt(c); }
func f_nested(): Array<Array<int>> { var a: Array<Array<int>> = []; var r: Array<int> = [5]; a.push(r); return a; }

func delta(name: str, before: int): int {
    let after = sloth_rc_live();
    print(name + " " + "${after - before}");
    return after;
}
func main(): unit {
    var sink = 0;
    var i = 0;
    var b = sloth_rc_live();

    i = 0; while i < 1000 { sink = sink + f_int(); i = i + 1; }
    b = delta("f_int", b);

    i = 0; while i < 1000 { sink = sink + len(f_str_lit()); i = i + 1; }
    b = delta("f_str_lit", b);

    i = 0; while i < 1000 { sink = sink + len(f_str_interp(i)); i = i + 1; }
    b = delta("f_str_interp", b);

    i = 0; while i < 1000 { sink = sink + len(f_str_cat(i)); i = i + 1; }
    b = delta("f_str_cat", b);

    i = 0; while i < 1000 { sink = sink + f_arr().len(); i = i + 1; }
    b = delta("f_arr", b);

    i = 0; while i < 1000 { sink = sink + f_arr_ref().len(); i = i + 1; }
    b = delta("f_arr_ref", b);

    i = 0; while i < 1000 { let o = Box(i); sink = sink + o.v; i = i + 1; }
    b = delta("class_new", b);

    i = 0; while i < 1000 { let x: dyn Node = f_dyn(i); sink = sink + x.tag(); i = i + 1; }
    b = delta("dyn_ret", b);

    i = 0; while i < 1000 { var a: Array<dyn Node> = []; a.push(NNull()); sink = sink + a.len(); i = i + 1; }
    b = delta("arr_push_ref", b);

    i = 0; while i < 1000 { sink = sink + f_nested()[0].len(); i = i + 1; }
    b = delta("f_nested", b);

    i = 0; while i < 1000 { var s = ""; var j = 0; while j < 5 { s = s + "x"; j = j + 1; } sink = sink + len(s); i = i + 1; }
    b = delta("str_build_loop", b);

    print("sink=${sink}");
}
