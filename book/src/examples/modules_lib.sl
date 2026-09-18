// 被导入模块：只有 pub 声明对外可见
pub func double(x: int): int {
    return x * 2;
}

pub var counter = 5;

func hidden(): int {          // 非 pub：模块外不可见
    return 0;
}
