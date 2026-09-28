// ext: fiber payload channel — `resume` returns the value the fiber yields
// (yielded value -> Y, completion -> nil), for value and reference payloads,
// with the `Y?` surface distinguishing a yielded value from completion nil.
// Also covers `fiber.transfer` control flow (the transferrer receives the
// value the target yields). NOTE: the *return* of `fiber.yield` (the next
// resume argument) is covered separately in
// test_workspace/extensions/bug1_fiber_yield_return.sl, which is currently
// wrong in the runtime; this seed deliberately does not assert it.

func value_roundtrip(): unit {
    let f = fiber.create(|init: int| -> unit {
        fiber.yield(1);
        fiber.yield(2);
    }, 0);
    print("a=${fiber.resume(f, 42) ?: -1}");   // starts, yields 1
    print("b=${fiber.resume(f, 77) ?: -1}");   // yields 2
    print("c=${fiber.resume(f, 88) ?: -1}");   // completes
    print("d=${fiber.resume(f, 99) ?: -1}");   // resume of a Done fiber
}

func nil_vs_zero(): unit {
    // a yielded 0 is Some(0), not completion nil
    let f = fiber.create(|init: int| -> unit {
        fiber.yield(0);
    }, 0);
    let a: int? = fiber.resume(f, 1);
    print("zero is nil=${a is nil}");
    print("zero=${a ?: -1}");
    let b: int? = fiber.resume(f, 2);
    print("done is nil=${b is nil}");
}

func ref_roundtrip(): unit {
    let f = fiber.create(|init: str| -> unit {
        fiber.yield("one");
        fiber.yield("two");
    }, "seed");
    print(fiber.resume(f, "x") ?: "?");
    print(fiber.resume(f, "y") ?: "?");
    print(fiber.resume(f, "z") ?: "?");
}

func transfer_case(): unit {
    let b = fiber.create(|x: int| -> unit {
        print("b start");
        fiber.yield(5);
    }, 0);
    let a = fiber.create(|x: int| -> unit {
        print("a start");
        let v = fiber.transfer(b, 99);
        print("a got ${v ?: -1}");
    }, 0);
    print("main r=${fiber.resume(a, 0) ?: -1}");
    print("main q=${fiber.resume(b, 7) ?: -1}");
}

func main(): unit {
    value_roundtrip();
    nil_vs_zero();
    ref_roundtrip();
    transfer_case();
    // expect: a=1
    // expect: b=2
    // expect: c=-1
    // expect: d=-1
    // expect: zero is nil=false
    // expect: zero=0
    // expect: done is nil=true
    // expect: one
    // expect: two
    // expect: ?
    // expect: a start
    // expect: b start
    // expect: a got 5
    // expect: main r=-1
    // expect: main q=-1
}
