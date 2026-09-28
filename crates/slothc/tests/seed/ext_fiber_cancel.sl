// ext: fiber cancellation / resumability / rc discipline. A suspended fiber
// that is driven to Done or explicitly cancelled releases its payloads and the
// live-object count returns to baseline.

func drain_to_done(): unit {
    let f = fiber.create(|init: int| -> unit {
        var i = 0;
        while i < 3 {
            fiber.yield(i);
            i = i + 1;
        }
    }, 0);
    print("resumable0=${fiber.resumable(f)}");
    var n = 0;
    while fiber.resumable(f) {
        let v = fiber.resume(f, 0);
        n = n + 1;
    }
    print("resumable1=${fiber.resumable(f)}");
    print("check=${fiber.check(f)}");
    print("n=${n}");
}


func cancel_suspended(): unit {
    let f = fiber.create(|init: str| -> unit {
        while true {
            fiber.yield(init);
        }
    }, "held");
    print("first=${fiber.resume(f, "x") ?: "nil"}");
    print("before=${fiber.resumable(f)}");
    fiber.cancel(f);
    print("after=${fiber.resumable(f)}");
    print("check=${fiber.check(f)}");
}

func cancel_nested(): unit {
    let f = fiber.create(|init: int| -> unit {
        while true {
            let x = fiber.yield(1);
            let y = fiber.yield(2);
        }
    }, 0);
    print(fiber.resume(f, 0) ?: -1);
    print(fiber.resume(f, 0) ?: -1);
    fiber.cancel(f);
    print("nested check=${fiber.check(f)}");
}

func main(): unit {
    let base = sloth_rc_live();
    drain_to_done();
    cancel_suspended();
    cancel_nested();
    print("balanced=${sloth_rc_live() == base}");
    // expect: resumable0=true
    // expect: resumable1=false
    // expect: check=false
    // expect: n=4
    // expect: first=held
    // expect: before=true
    // expect: after=false
    // expect: check=false
    // expect: 1
    // expect: 2
    // expect: nested check=false
    // expect: balanced=true
}
