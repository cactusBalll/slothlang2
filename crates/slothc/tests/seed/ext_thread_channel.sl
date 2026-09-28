// ext: channels — bounded FIFO, unbounded queue, close -> nil, cross-thread
// producer/consumer, and close waking a blocked recv.

func fifo_bounded(): unit {
    let ch = channel.new<int>(2);
    ch.send(10);
    ch.send(20);
    ch.close();
    let a: int? = ch.recv();
    let b: int? = ch.recv();
    let c: int? = ch.recv();
    print("a=${a ?: -1}");
    print("b=${b ?: -1}");
    print("c is nil=${c is nil}");
    print("c=${c ?: -1}");
}

func unbounded_str(): unit {
    let sc = channel.new<str>(0);
    sc.send("x");
    sc.send("y");
    sc.send("z");
    let s1: str? = sc.recv();
    let s2: str? = sc.recv();
    let s3: str? = sc.recv();
    print("s1=${s1 ?: "?"}");
    print("s2=${s2 ?: "?"}");
    print("s3=${s3 ?: "?"}");
    sc.close();
    let s4: str? = sc.recv();
    print("s4 is nil=${s4 is nil}");
}

func producer_consumer(): unit {
    let ch = channel.new<int>(4);
    let h: JoinHandle<int> = thread.spawn(|seed: int| -> int {
        var i = 0;
        while i < 1000 {
            ch.send(i);
            i = i + 1;
        }
        ch.close();
        return 0;
    }, 0);
    // drain concurrently so a bounded channel cannot deadlock the producer
    let acc = atomic.new(0);
    while true {
        let v: int? = ch.recv();
        if v is nil { break; }
        acc.add(v ?: 0);
    }
    h.join();
    print("acc=${acc.load()}");
    print("acc ok=${acc.load() == 499500}");
}

func close_wakes_recv(): unit {
    let ch = channel.new<int>(0);
    let got = channel.new<int>(0);
    let h: JoinHandle<int> = thread.spawn(|seed: int| -> int {
        let v: int? = ch.recv();
        if v is nil { got.send(-1); } else { got.send(v ?: 0); }
        return 0;
    }, 0);
    thread.yield_now();
    ch.close();
    let r: int? = got.recv();
    print("blocked saw ${r ?: -99}");
    h.join();
}

func main(): unit {
    fifo_bounded();
    unbounded_str();
    producer_consumer();
    close_wakes_recv();
    // expect: a=10
    // expect: b=20
    // expect: c is nil=true
    // expect: c=-1
    // expect: s1=x
    // expect: s2=y
    // expect: s3=z
    // expect: s4 is nil=true
    // expect: acc=499500
    // expect: acc ok=true
    // expect: blocked saw -1
}
