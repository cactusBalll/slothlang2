// seed: discarding a `unit` result with `let _ = …` is allowed (the
// placeholder nil word keeps the store well-formed; probe containers/x7)
func main(): unit {
    let _ = print("hi");
    print("done");
    // expect: hi
    // expect: done
}
