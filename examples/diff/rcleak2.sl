// Localize the reference-return leak: binding vs temporary, param vs return.
class Box {
    var v: int;
    func __init__(v: int) { this.v = v; }
}
func f_str_lit(): str { return "abc"; }
func f_arr(): Array<int> { var a: Array<int> = []; a.push(1); return a; }
func f_arr2(): Array<int> { var a: Array<int> = [1, 2, 3]; return a; }
func f_box(): Box { return Box(1); }
func consume(s: str): int { return len(s); }
func consume_arr(a: Array<int>): int { return a.len(); }
func delta(name: str, before: int): int {
    let after = sloth_rc_live();
    print(name + " " + "${after - before}");
    return after;
}
func main(): unit {
    var sink = 0;
    var i = 0;
    var b = sloth_rc_live();

    i = 0; while i < 1000 { let s = f_str_lit(); sink = sink + len(s); i = i + 1; }
    b = delta("bind_str_ret", b);

    i = 0; while i < 1000 { sink = sink + len(f_str_lit()); i = i + 1; }
    b = delta("temp_str_ret", b);

    i = 0; while i < 1000 { sink = sink + len("abc"); i = i + 1; }
    b = delta("lit_only", b);

    i = 0; while i < 1000 { sink = sink + consume("abc"); i = i + 1; }
    b = delta("param_str_lit", b);

    i = 0; while i < 1000 { let s = f_str_lit(); sink = sink + consume(s); i = i + 1; }
    b = delta("param_str_var", b);

    i = 0; while i < 1000 { let a = f_arr(); sink = sink + a.len(); i = i + 1; }
    b = delta("bind_arr_ret", b);

    i = 0; while i < 1000 { sink = sink + f_arr().len(); i = i + 1; }
    b = delta("temp_arr_ret", b);

    i = 0; while i < 1000 { let a = f_arr(); sink = sink + consume_arr(a); i = i + 1; }
    b = delta("param_arr_var", b);

    i = 0; while i < 1000 { sink = sink + f_arr2()[0]; i = i + 1; }
    b = delta("temp_arr_idx", b);

    i = 0; while i < 1000 { let x = f_box(); sink = sink + x.v; i = i + 1; }
    b = delta("bind_box_ret", b);

    i = 0; while i < 1000 { sink = sink + f_box().v; i = i + 1; }
    b = delta("temp_box_ret", b);

    print("sink=${sink}");
}
