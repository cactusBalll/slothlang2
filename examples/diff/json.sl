// JSON recursive-descent parser + canonical serializer, differential vs C.
// Scope: structural JSON, escapes \" \\ \/ \n \t \r (full) and \uXXXX for
// printable ASCII (others error — slothlang cannot build arbitrary chars).
// Numbers: JInt (no . / e) or JFloat; floats canonicalized to 6 decimals.
var seed = 987654321;
var DIGITS: Array<str> = [];
var HEXS: Array<str> = [];
var ASCIIP: Array<str> = [];

func rnd(): int {
    seed = (seed * 1103515245 + 12345) % 2147483648;
    return seed;
}
func explode(s: str): Array<str> {
    var a: Array<str> = [];
    for ch in s { a.push(ch); }
    return a;
}
func setup(): unit {
    DIGITS = explode("0123456789");
    HEXS = explode("0123456789abcdefABCDEF");
    ASCIIP = explode(" !\"#\$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~");
}
func idx_in(ws: Array<str>, c: str): int {
    var i = 0;
    while i < ws.len() { if ws[i] == c { return i; } i = i + 1; }
    return 0 - 1;
}
func digit_val(c: str): int { return idx_in(DIGITS, c); }
func is_digit(c: str): bool { return digit_val(c) >= 0; }
func hex_val(c: str): int {
    let i = idx_in(HEXS, c);
    if i < 0 { return 0 - 1; }
    if i < 16 { return i; }
    return i - 6;
}
func is_ws(c: str): bool { return c == " " or c == "\t" or c == "\n" or c == "\r"; }
func esc(s: str): str {
    var out = "\"";
    let cs = explode(s);
    var i = 0;
    while i < cs.len() {
        let c = cs[i];
        if c == "\"" { out = out + "\\\""; }
        else if c == "\\" { out = out + "\\\\"; }
        else if c == "\n" { out = out + "\\n"; }
        else if c == "\t" { out = out + "\\t"; }
        else if c == "\r" { out = out + "\\r"; }
        else { out = out + c; }
        i = i + 1;
    }
    return out + "\"";
}
func pad6(v: int): str {
    var s = "${v}";
    while s.len() < 6 { s = "0" + s; }
    return s;
}
func fmt6(v: float): str {
    var neg = false;
    if v < 0.0 { neg = true; v = 0.0 - v; }
    let scaled = int(v * 1000000.0 + 0.5);
    let whole = scaled / 1000000;
    let frac = scaled % 1000000;
    var fs = "${frac}";
    while fs.len() < 6 { fs = "0" + fs; }
    var out = "${whole}" + "." + fs;
    if neg { out = "-" + out; }
    return out;
}

trait Json { func dump(): str; }
class JNull impl Json { func dump(): str { return "null"; } }
class JBool impl Json {
    var b: bool;
    func __init__(b: bool) { this.b = b; }
    func dump(): str { if this.b { return "true"; } return "false"; }
}
class JInt impl Json {
    var v: int;
    func __init__(v: int) { this.v = v; }
    func dump(): str { return "${this.v}"; }
}
class JFloat impl Json {
    var v: float;
    func __init__(v: float) { this.v = v; }
    func dump(): str { return fmt6(this.v); }
}
class JStr impl Json {
    var v: str;
    func __init__(v: str) { this.v = v; }
    func dump(): str { return esc(this.v); }
}
class JArr impl Json {
    var items: Array<dyn Json> = [];
    func add(x: dyn Json): unit { this.items.push(x); }
    func dump(): str {
        var s = "[";
        var i = 0;
        while i < this.items.len() {
            if i > 0 { s = s + ","; }
            s = s + this.items[i].dump();
            i = i + 1;
        }
        return s + "]";
    }
}
class JObj impl Json {
    var keys: Array<str> = [];
    var vals: Array<dyn Json> = [];
    func put(k: str, v: dyn Json): unit { this.keys.push(k); this.vals.push(v); }
    func dump(): str {
        var s = "{";
        var i = 0;
        while i < this.keys.len() {
            if i > 0 { s = s + ","; }
            s = s + esc(this.keys[i]) + ":" + this.vals[i].dump();
            i = i + 1;
        }
        return s + "}";
    }
}

func sum_ints(x: dyn Json): int {
    if x is JInt { return x.v; }
    if x is JArr {
        var s = 0;
        for it in x.items { s = s + sum_ints(it); }
        return s;
    }
    if x is JObj {
        var s = 0;
        var i = 0;
        while i < x.vals.len() { s = s + sum_ints(x.vals[i]); i = i + 1; }
        return s;
    }
    return 0;
}

class Parser {
    var cs: Array<str> = [];
    var i: int = 0;
    var err: bool = false;
    func __init__(s: str) { this.cs = explode(s); this.i = 0; this.err = false; }
    func fail(): unit { this.err = true; }
    func peek(): str { if this.i < this.cs.len() { return this.cs[this.i]; } return ""; }
    func peekn(k: int): str { if this.i + k < this.cs.len() { return this.cs[this.i + k]; } return ""; }
    func skip_ws(): unit { while this.i < this.cs.len() and is_ws(this.cs[this.i]) { this.i = this.i + 1; } }
    func finish(): unit { this.skip_ws(); if this.i != this.cs.len() { this.fail(); } }
    func lit(s: str): bool {
        let ls = explode(s);
        if this.i + ls.len() > this.cs.len() { return false; }
        var k = 0;
        while k < ls.len() { if this.cs[this.i + k] != ls[k] { return false; } k = k + 1; }
        this.i = this.i + ls.len();
        return true;
    }
    func parse_value(): dyn Json {
        this.skip_ws();
        if this.i >= this.cs.len() { this.fail(); return JNull(); }
        let c = this.cs[this.i];
        if c == "{" { return this.parse_object(); }
        if c == "[" { return this.parse_array(); }
        if c == "\"" { return JStr(this.parse_string()); }
        if c == "t" { if this.lit("true") { return JBool(true); } this.fail(); return JNull(); }
        if c == "f" { if this.lit("false") { return JBool(false); } this.fail(); return JNull(); }
        if c == "n" { if this.lit("null") { return JNull(); } this.fail(); return JNull(); }
        return this.parse_number();
    }
    func parse_string(): str {
        this.i = this.i + 1;
        var s = "";
        while true {
            if this.i >= this.cs.len() { this.fail(); return s; }
            let c = this.cs[this.i];
            if c == "\"" { this.i = this.i + 1; return s; }
            if c == "\\" {
                this.i = this.i + 1;
                if this.i >= this.cs.len() { this.fail(); return s; }
                let e = this.cs[this.i];
                if e == "n" { s = s + "\n"; this.i = this.i + 1; }
                else if e == "t" { s = s + "\t"; this.i = this.i + 1; }
                else if e == "r" { s = s + "\r"; this.i = this.i + 1; }
                else if e == "b" { s = s + "\\b"; this.i = this.i + 1; }
                else if e == "f" { s = s + "\\f"; this.i = this.i + 1; }
                else if e == "\"" { s = s + "\""; this.i = this.i + 1; }
                else if e == "\\" { s = s + "\\"; this.i = this.i + 1; }
                else if e == "/" { s = s + "/"; this.i = this.i + 1; }
                else if e == "u" { this.i = this.i + 1; let ch = this.parse_u(); s = s + ch; }
                else { this.fail(); return s; }
            } else {
                s = s + c;
                this.i = this.i + 1;
            }
        }
        return s;
    }
    func parse_u(): str {
        if this.i + 4 > this.cs.len() { this.fail(); return ""; }
        var code = 0;
        var k = 0;
        while k < 4 {
            let h = hex_val(this.cs[this.i + k]);
            if h < 0 { this.fail(); return ""; }
            code = code * 16 + h;
            k = k + 1;
        }
        this.i = this.i + 4;
        if code == 9 { return "\t"; }
        if code == 10 { return "\n"; }
        if code == 13 { return "\r"; }
        if code == 34 { return "\""; }
        if code == 92 { return "\\"; }
        if code >= 32 and code <= 126 { return ASCIIP[code - 32]; }
        this.fail();
        return "";
    }
    func parse_number(): dyn Json {
        var neg = false;
        if this.peek() == "-" { neg = true; this.i = this.i + 1; }
        var mant = 0;
        var ndig = 0;
        if this.peek() == "0" { this.i = this.i + 1; ndig = 1; }
        else {
            while is_digit(this.peek()) { mant = mant * 10 + digit_val(this.peek()); this.i = this.i + 1; ndig = ndig + 1; }
        }
        if ndig == 0 { this.fail(); return JNull(); }
        var isfloat = false;
        var scale = 1;
        if this.peek() == "." {
            isfloat = true;
            this.i = this.i + 1;
            if not is_digit(this.peek()) { this.fail(); }
            while is_digit(this.peek()) { mant = mant * 10 + digit_val(this.peek()); this.i = this.i + 1; scale = scale * 10; }
        }
        var exp = 0;
        if this.peek() == "e" or this.peek() == "E" {
            isfloat = true;
            this.i = this.i + 1;
            var esign = 1;
            if this.peek() == "+" { this.i = this.i + 1; }
            else if this.peek() == "-" { esign = 0 - 1; this.i = this.i + 1; }
            if not is_digit(this.peek()) { this.fail(); }
            while is_digit(this.peek()) { exp = exp * 10 + digit_val(this.peek()); this.i = this.i + 1; }
            exp = exp * esign;
        }
        if isfloat {
            var v = float(mant) / float(scale);
            var k = 0;
            if exp > 0 { while k < exp { v = v * 10.0; k = k + 1; } }
            else { while k < 0 - exp { v = v / 10.0; k = k + 1; } }
            if neg { v = 0.0 - v; }
            return JFloat(v);
        }
        if neg { mant = 0 - mant; }
        return JInt(mant);
    }
    func parse_array(): dyn Json {
        let a = JArr();
        this.i = this.i + 1;
        this.skip_ws();
        if this.peek() == "]" { this.i = this.i + 1; return a; }
        while true {
            let v = this.parse_value();
            a.add(v);
            if this.err { return a; }
            this.skip_ws();
            let c = this.peek();
            if c == "," { this.i = this.i + 1; continue; }
            if c == "]" { this.i = this.i + 1; return a; }
            this.fail();
            return a;
        }
        return a;
    }
    func parse_object(): dyn Json {
        let o = JObj();
        this.i = this.i + 1;
        this.skip_ws();
        if this.peek() == "}" { this.i = this.i + 1; return o; }
        while true {
            this.skip_ws();
            if this.peek() != "\"" { this.fail(); return o; }
            let k = this.parse_string();
            this.skip_ws();
            if this.peek() != ":" { this.fail(); return o; }
            this.i = this.i + 1;
            let v = this.parse_value();
            o.put(k, v);
            if this.err { return o; }
            this.skip_ws();
            let c = this.peek();
            if c == "," { this.i = this.i + 1; continue; }
            if c == "}" { this.i = this.i + 1; return o; }
            this.fail();
            return o;
        }
        return o;
    }
}

func gen_str_content(): str {
    var n = rnd() % 16;
    var s = "";
    var i = 0;
    while i < n {
        let k = rnd() % 8;
        if k == 0 { s = s + "a"; }
        else if k == 1 { s = s + "Z"; }
        else if k == 2 { s = s + "0"; }
        else if k == 3 { s = s + " "; }
        else if k == 4 { s = s + "\""; }
        else if k == 5 { s = s + "\\"; }
        else if k == 6 { s = s + "\n"; }
        else { s = s + "\t"; }
        i = i + 1;
    }
    return s;
}
func gen_value(depth: int): str {
    var t = rnd() % 8;
    if depth >= 3 and t >= 6 { t = t % 6; }
    if t == 0 { return "null"; }
    if t == 1 { return "true"; }
    if t == 2 { return "false"; }
    if t == 3 { let v = rnd() % 200001 - 100000; return "${v}"; }
    if t == 4 { let iv = rnd() % 1000; let fv = rnd() % 1000000; return "${iv}." + pad6(fv); }
    if t == 5 { return esc(gen_str_content()); }
    if t == 6 {
        let n = rnd() % 6;
        var s = "[";
        var i = 0;
        while i < n { if i > 0 { s = s + ","; } s = s + gen_value(depth + 1); i = i + 1; }
        return s + "]";
    }
    let n = rnd() % 6;
    var s = "{";
    var i = 0;
    while i < n {
        if i > 0 { s = s + ","; }
        s = s + esc(gen_str_content()) + ":" + gen_value(depth + 1);
        i = i + 1;
    }
    return s + "}";
}
func report(src: str): unit {
    let p = Parser(src);
    let v = p.parse_value();
    p.finish();
    if p.err { print("ERR"); } else { print(v.dump() + "|" + "${sum_ints(v)}"); }
}
func main(): unit {
    setup();
    report("null");
    report("true");
    report("false");
    report("42");
    report("-7");
    report("3.14");
    report("1e3");
    report("2.5e-2");
    report("[]");
    report("{}");
    report("[1,2,3]");
    report("  [ 1 , 2 ]  ");
    report("{\"a\":1,\"b\":[true,null]}");
    report("{\"x\":{\"y\":{\"z\":1}}}");
    report("\"a\\\"b\"");
    report("\"tab\\there\"");
    report("\"\\u0041\\u0042\"");
    report("\"slash\\/done\"");
    report("{\"\":0}");
    report("[[[[]]]]");
    report("{");
    report("[1,]");
    report("tru");
    report("\"abc");
    report("01");
    report("[1 2]");
    report("nul");
    report("-");
    report("1.");
    report("{\"a\" 1}");
    report("\"\\x\"");
    report("\"\\u00G1\"");
    report("{\"a\":1,}");
    report("0");
    report("-0");
    report("0.0");
    report("0e0");
    report("1E+2");
    report("123.456e2");
    report("\"\\u0020\\u007e\"");
    report("\"\\u006a\\u004A\"");
    report("\"café\"");
    report("\"\\u4e2d\"");
    report("+1");
    report("00");
    report("-01");
    report(".5");
    report("1e");
    report("1e+");
    report("1.2.3");
    report("[");
    report("{\"a\":}");
    report("{\"a\":1");
    report("\r\n\t [\n1\r,\t2 ]\n");
    report("{\"a\":1,\"a\":2}");
    report("[[],{}]");
    report("   ");
    report("null,1");
    var ta = "[";
    var x = 0;
    while x < 2000 { if x > 0 { ta = ta + ","; } ta = ta + "${x}"; x = x + 1; }
    report(ta + "]");
    var to = "{";
    x = 0;
    while x < 200 { if x > 0 { to = to + ","; } to = to + "\"k${x}\":" + "${x}"; x = x + 1; }
    report(to + "}");
    var ls = "\"";
    x = 0;
    while x < 400 { ls = ls + "a"; x = x + 1; }
    report(ls + "\"");
    var d = "";
    var y = 0;
    while y < 300 { d = d + "["; y = y + 1; }
    y = 0;
    while y < 300 { d = d + "]"; y = y + 1; }
    report(d);
    var ra = "[";
    y = 0;
    while y < 1000 { if y > 0 { ra = ra + ","; } ra = ra + "[]"; y = y + 1; }
    report(ra + "]");
    var t = 0;
    while t < 600 {
        let src = gen_value(0);
        report(src);
        t = t + 1;
    }
}
