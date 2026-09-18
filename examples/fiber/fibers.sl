// Coroutine extension (CE) example: the sloth-lang 1.0 §3.5 fiber walkthrough,
// typed for 2.0. `fiber.error` ends the cooperative loop; the completed fiber
// is reclaimed by ARC with no leaks.

func doc_example(): unit {
    let f = fiber.create(|init: int| -> unit {
        var i = 0;
        var v = init;
        while (i < 10) {
            v = fiber.yield(i);
            print("v from main fiber: ${v}");
            i = i + 1;
            if (i > 5) {
                print("error in fiber");
                fiber.error("i exceeded 5");
            }
        }
    }, 0);

    var cnt = 0;
    while (not fiber.check(f)) {
        cnt = cnt + 3;
        print("resumable: ${fiber.resumable(f)}");
        let got = fiber.resume(f, cnt);
        print("got i from fiber: ${got ?: -1}");
    }
}

func main(): unit {
    doc_example();
    print("fiber OK");
}
