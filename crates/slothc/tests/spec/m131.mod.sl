// spec: imported module using str byte subscript + chars() (not run directly)
pub func codepoint_sum(s: str): int {
    var n = 0;
    for c in s.chars() {
        n = n + c;
    }
    return n;
}

pub func second_byte(s: str): int {
    return s[1];
}
