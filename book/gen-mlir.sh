#!/usr/bin/env bash
# 从 book/src/examples/*.sl 生成对应的 .mlir。
#
# 逐文件运行 `slothc ir`，并剥离"固定运行时前导声明"（每个模块都会原样
# 发射的 `func.func private @sloth_*`）。前导集合由空程序计算得到，因此
# 用户自己的 `extern func` 声明会保留下来。
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

printf 'func main() {\n}\n' > "$TMP/base.sl"
"$SLOTHC" ir "$TMP/base.sl" | grep '^  func.func private @sloth_' | sort -u > "$TMP/prelude.txt"

n=0
for f in "$EX"/*.sl; do
  b=$(basename "$f" .sl)
  case "$b" in
    modules_lib) continue ;; # 被导入的模块，无独立入口
  esac
  "$SLOTHC" ir "$f" > "$TMP/$b.raw"
  grep -vxF -f "$TMP/prelude.txt" "$TMP/$b.raw" > "$EX/$b.mlir"
  n=$((n + 1))
done

# 附录 B 用：一个**未剥离**前导的完整模块
"$SLOTHC" ir "$EX/hello.sl" > "$EX/hello_full.mlir"
echo "generated $((n + 1)) mlir file(s) in $EX"
