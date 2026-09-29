// seed: a closure nested inside a fiber entry body inherits the entry payload
// `Y`, so its `fiber.yield` is checked too (bug B4)
func main(): unit {
    let f = fiber.create(|init: int| -> unit {
        let inner = |x: int| -> unit {
            fiber.yield("bad");
        };
        inner(0);
    }, 0);
    let r: int? = fiber.resume(f, 0);
    print(r ?: -1);
}
// diag: fiber payload mismatch: expected int, got str
