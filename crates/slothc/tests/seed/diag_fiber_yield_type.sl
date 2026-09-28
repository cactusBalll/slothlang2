// seed: `fiber.yield` argument is checked against the entry payload `Y`
// (extension G2/G3)
func main(): unit {
    let f = fiber.create(|init: int| -> unit {
        let s = "hello";
        fiber.yield(s);
    }, 0);
    let r: int? = fiber.resume(f, 0);
    print(r ?: -1);
}
// diag: fiber payload mismatch
