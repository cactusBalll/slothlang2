// ext: fiber.create_with — custom stack sizes are accepted and a smaller
// stack still runs a non-trivial body; byte payloads are delivered.

func main(): unit {
    let f = fiber.create_with(|init: int| -> unit {
        let a = fiber.yield(init + 1);
        let b = fiber.yield(init + 2);
        print("body end");
    }, 10, 65536);
    print("r0=${fiber.resume(f, 0) ?: -1}");
    print("r1=${fiber.resume(f, 0) ?: -1}");
    print("r2=${fiber.resume(f, 0) ?: -1}");

    // a large stack request also works
    let g = fiber.create_with(|init: str| -> unit {
        fiber.yield(init);
    }, "big", 1048576);
    print("big=${fiber.resume(g, "x") ?: "?"}");
    fiber.cancel(g);
    print("big resumable=${fiber.resumable(g)}");
    // expect: r0=11
    // expect: r1=12
    // expect: body end
    // expect: r2=-1
    // expect: big=big
    // expect: big resumable=false
}
