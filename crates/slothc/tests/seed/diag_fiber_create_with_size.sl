// seed: `fiber.create_with` stack size is checked as `int` (bug B4)
func main(): unit {
    let f = fiber.create_with(|init: int| -> unit { fiber.yield(init); }, 0, "big");
    let r: int? = fiber.resume(f, 0);
    print(r ?: -1);
}
// diag: fiber.create_with stack size
