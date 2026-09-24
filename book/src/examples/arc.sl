// ARC 所有权协议：引用临时量在语句末结算，churn 后计数归零
class Holder {
    var s: str;
    var a: Array<int>;
    func __init__() {
        this.s = "hello";
        this.a = [1, 2, 3];
    }
}

func churn(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        let h = Holder();                 // 对象 + 两个引用字段，作用域退出释放
        acc = acc + h.s.len() + h.a.len();
        i = i + 1;
    }
    return acc;
}

func main() {
    let _ = churn(50);                    // 预热/稳定计数
    let base = sloth_rc_live();
    let r = churn(1000);
    print(r > 0);                         // true
    print(sloth_rc_live() == base);       // true：无泄漏
}
