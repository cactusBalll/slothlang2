// ext: fiber state machine — New/Suspended/Done/Error, check, resumable,
// cooperative cancel (cancel is not Error) and resume of a finished fiber.

func states(): unit {
    let f = fiber.create(|init: int| -> unit {
        fiber.yield(1);
    }, 0);
    print("new check=${fiber.check(f)}");
    print("new resumable=${fiber.resumable(f)}");
    let a: int? = fiber.resume(f, 0);
    print("yielded=${a ?: -1}");
    print("susp check=${fiber.check(f)}");
    print("susp resumable=${fiber.resumable(f)}");
    let b: int? = fiber.resume(f, 0);
    print("done nil=${b is nil}");
    print("done check=${fiber.check(f)}");
    print("done resumable=${fiber.resumable(f)}");
    // resuming a finished fiber keeps returning nil
    let c: int? = fiber.resume(f, 0);
    print("again nil=${c is nil}");
}

func errored(): unit {
    let f = fiber.create(|init: int| -> unit {
        fiber.error("intentional");
    }, 0);
    print("pre check=${fiber.check(f)}");
    let r: int? = fiber.resume(f, 0);
    print("err nil=${r is nil}");
    print("err check=${fiber.check(f)}");
    print("err resumable=${fiber.resumable(f)}");
}

func cancelled(): unit {
    let f = fiber.create(|init: str| -> unit {
        while true {
            fiber.yield(init);
        }
    }, "held");
    let a: str? = fiber.resume(f, "x");
    print("first=${a ?: "nil"}");
    print("before=${fiber.resumable(f)}");
    fiber.cancel(f);
    print("after=${fiber.resumable(f)}");
    print("cancel check=${fiber.check(f)}");
}

func main(): unit {
    states();
    errored();
    cancelled();
    // expect: new check=false
    // expect: new resumable=true
    // expect: yielded=1
    // expect: susp check=false
    // expect: susp resumable=true
    // expect: done nil=true
    // expect: done check=false
    // expect: done resumable=false
    // expect: again nil=true
    // expect: pre check=false
    // expect: err nil=true
    // expect: err check=true
    // expect: err resumable=false
    // expect: first=held
    // expect: before=true
    // expect: after=false
    // expect: cancel check=false
}
