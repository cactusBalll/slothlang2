// spec: TH-P2 channels — FIFO, bounded/unbounded, close -> nil
func main(): unit {
    let ch = channel.new<int>(2);
    ch.send(10);
    ch.send(20);
    ch.close();
    let a: int? = ch.recv();
    let b: int? = ch.recv();
    let c: int? = ch.recv();
    print(a ?: -1); // expect: 10
    print(b ?: -1); // expect: 20
    print(c is nil); // expect: true
    print(c ?: -1); // expect: -1

    // ref elements, unbounded queue
    let sc = channel.new<str>(0);
    sc.send("x");
    sc.send("y");
    let s1: str? = sc.recv();
    print(s1 ?: "?"); // expect: x
    print("channel OK"); // expect: channel OK
}
