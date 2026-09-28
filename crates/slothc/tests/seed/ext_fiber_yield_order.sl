// seed: `fiber.yield` returns the payload of the *next* resume (extension G1)
func main(): unit {
    let f = fiber.create(|init: int| -> unit {
        let r = fiber.yield(1);
        print("r=${r}");
    }, 0);
    let a = fiber.resume(f, 42);
    print("a=${a ?: -1}");
    let b = fiber.resume(f, 77);
    print("b=${b ?: -1}");
}
// expect: a=1
// expect: r=77
// expect: b=-1
