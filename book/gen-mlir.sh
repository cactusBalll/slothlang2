#!/usr/bin/env bash
# 从 book/src/examples/*.sl 生成对应的 .mlir。
#
# 逐文件运行 `slothc ir`，并剥离：
#   1. "固定运行时前导声明"（每个模块都会原样发射的 `func.func private
#      @sloth_*`），集合由空程序计算得到，因此用户自己的 `extern func`
#      声明会保留；
#   2. 自举容器实现（`lib/prelude/containers.slt` 注入后发射的
#      `@sloth_arr_*` / `@sloth_map_*` 函数体），它约 1700 行且与示例无关。
set -euo pipefail
cd "$(dirname "$0")/.." # workspace 根

SLOTHC=./target/debug/slothc
EX=book/src/examples
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT

if [ ! -x "$SLOTHC" ]; then
  echo "building slothc ..." >&2
  cargo build -p slothc >/dev/null 2>&1
fi

# 去掉一段顶层容器函数（函数头以 `@...sloth_(arr|map)_` 开头，函数体以
# 两空格缩进的 `}` 结束）
cat > "$TMP/strip.awk" <<'AWK'
/^  (func[.]func|llvm[.]func) @.*sloth_(arr|map)_/ { skip = 1 }
skip == 1 && /^  }$/ { skip = 0; next }
skip == 1 { next }
{ print }
AWK

printf 'func main() {\n}\n' > "$TMP/base.sl"
"$SLOTHC" ir "$TMP/base.sl" | grep '^  func.func private @sloth_' | sort -u > "$TMP/prelude.txt"

n=0
for f in "$EX"/*.sl; do
  b=$(basename "$f" .sl)
  case "$b" in
    modules_lib) continue ;; # 被导入的模块，无独立入口
  esac
  "$SLOTHC" ir "$f" | awk -f "$TMP/strip.awk" > "$TMP/$b.raw"
  grep -vxF -f "$TMP/prelude.txt" "$TMP/$b.raw" > "$EX/$b.mlir"
  n=$((n + 1))
done

# 附录 B 用：保留固定运行时前导（但容器实现同样剥离，否则示例过长）
"$SLOTHC" ir "$EX/hello.sl" | awk -f "$TMP/strip.awk" > "$EX/hello_full.mlir"
echo "generated $((n + 1)) mlir file(s) in $EX"
