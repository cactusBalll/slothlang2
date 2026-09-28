// seed: `Channel<T>.send` argument is checked against the element type
// (extension G5)
func main(): unit {
    let ch = channel.new<str>(0);
    ch.send(5);
    let v: str? = ch.recv();
    print(v ?: "nil");
}
// diag: channel element type mismatch
