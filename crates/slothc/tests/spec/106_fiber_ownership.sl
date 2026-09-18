// spec: coroutine extension CE-P2 — payload ownership across switches settles
// to the rc baseline; cooperative cancel retires a suspended fiber

func churn(n: int): unit {
    var i = 0;
    while i < n {
        let f = fiber.create(|init: str| -> unit {
            var s = init;
            let a = fiber.yield("a${init}");
            let b = fiber.yield("b${init}");
        }, "seed${i}");
        var k = 0;
        while k < 3 {
            let g = fiber.resume(f, "x${k}");
            let s = g ?: "nil";
            k = k + 1;
        }
        i = i + 1;
    }
}

func cancel_values(): unit {
    let f = fiber.create(|init: int| -> unit {
        var i = 0;
        while true {
            let got = fiber.yield(i);
            i = i + 1;
        }
    }, 0);
    let a = fiber.resume(f, 0);
    print("first: ${a ?: -1}");
    print("resumable: ${fiber.resumable(f)}");
    fiber.cancel(f);
    print("after cancel: ${fiber.resumable(f)}");
    print("check: ${fiber.check(f)}");
}

func helper_err(): unit {
    let h = "helper held";
    fiber.error("nested");
}

func nested_err(): unit {
    let f = fiber.create(|init: int| -> unit {
        let e = "entry held";
        helper_err();
    }, 0);
    let r = fiber.resume(f, 0);
}

func helper_yield(): int {
    let h = "helper held";
    let a = fiber.yield(1);
    return a;
}

func nested_cancel(): unit {
    let f = fiber.create(|init: int| -> unit {
        let e = "entry held";
        let x = helper_yield();
        let y = fiber.yield(2);
    }, 0);
    let r = fiber.resume(f, 0);
    fiber.cancel(f);
}

func main(): unit {
    let base = sloth_rc_live();
    churn(50);
    cancel_values();
    nested_err();
    nested_cancel();
    print("balanced: ${sloth_rc_live() == base}");
    // expect: first: 0
    // expect: resumable: true
    // expect: after cancel: false
    // expect: check: false
    // expect: balanced: true
}
