// ext: fiber payload ownership — payloads crossing switches settle to the
// sloth_rc_live() baseline after fibers are driven to Done, errored or
// explicitly cancelled.

func churn(n: int): unit {
    var i = 0;
    while i < n {
        let f = fiber.create(|init: str| -> unit {
            var k = 0;
            while k < 3 {
                let s = "p${init}-${k}";
                fiber.yield(s);
                k = k + 1;
            }
        }, "seed${i}");
        while fiber.resumable(f) {
            let g = fiber.resume(f, "r");
        }
        i = i + 1;
    }
}

func cancel_rc(n: int): unit {
    var i = 0;
    while i < n {
        let f = fiber.create(|init: str| -> unit {
            while true {
                fiber.yield(init);
            }
        }, "held${i}");
        fiber.resume(f, "z");
        fiber.cancel(f);
        i = i + 1;
    }
}

func error_rc(n: int): unit {
    var i = 0;
    while i < n {
        let f = fiber.create(|init: str| -> unit {
            let held = "e${i}";
            fiber.yield(held);
            fiber.error("done");
        }, "init${i}");
        fiber.resume(f, "a");
        fiber.resume(f, "b");
        i = i + 1;
    }
}

func main(): unit {
    let base = sloth_rc_live();
    churn(50);
    cancel_rc(100);
    error_rc(100);
    print("balanced=${sloth_rc_live() == base}");
    // expect: balanced=true
}
