// spec: coroutine extension CE-P1 — typed stackful fibers: create/resume/
// yield/check/resumable/error, reference payloads, optional result surface

func doc_example(): unit {
    let f = fiber.create(|init: int| -> unit {
        var i = 0;
        while (true) {
            let got = fiber.yield(i);
            i = i + 1;
            if (i > 2) {
                fiber.error("stop");
            }
        }
    }, 0);

    var n = 0;
    while (not fiber.check(f)) {
        n = n + 1;
        let got = fiber.resume(f, n);
        print("got: ${got ?: -1}");
    }
    print("checks: ${fiber.check(f)}");
}

func ref_payload(): unit {
    let f = fiber.create(|init: str| -> unit {
        var s = init;
        let a = fiber.yield("one");
        let b = fiber.yield("two");
    }, "seed");
    print("resumable0: ${fiber.resumable(f)}");
    let g1 = fiber.resume(f, "A");
    let s1 = g1 ?: "nil";
    print("g1: ${s1}");
    let g2 = fiber.resume(f, "B");
    let s2 = g2 ?: "nil";
    print("g2: ${s2}");
    print("resumable1: ${fiber.resumable(f)}");
    let g3 = fiber.resume(f, "C");
    let s3 = g3 ?: "nil";
    print("g3: ${s3}");
    print("resumable2: ${fiber.resumable(f)}");
}

func main(): unit {
    doc_example();
    ref_payload();
    // expect: got: 0
    // expect: got: 1
    // expect: got: 2
    // expect: got: -1
    // expect: checks: true
    // expect: resumable0: true
    // expect: g1: one
    // expect: g2: two
    // expect: resumable1: true
    // expect: g3: nil
    // expect: resumable2: false
}
