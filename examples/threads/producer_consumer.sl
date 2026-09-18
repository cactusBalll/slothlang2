// TH-P3: mpmc producer/consumer over a bounded `Channel<int>` (design §8.1.2).
// 4 producers and 4 consumers run concurrently; sends transfer ownership and
// receives hand it back (happens-before edge). Sums must match exactly.

func main(): unit {
    let ch = channel.new<int>(64);
    let results = channel.new<int>(0);
    let per = 10000;

    var producers: Array<JoinHandle<int>> = [];
    for (var t: 0..4) {
        producers.push(thread.spawn(|seed: int| -> int {
            var i = 0;
            while (i < per) {
                ch.send(i);
                i = i + 1;
            }
            return seed;
        }, t));
    }

    var consumers: Array<JoinHandle<int>> = [];
    for (var t: 0..4) {
        consumers.push(thread.spawn(|seed: int| -> int {
            var got = 0;
            var s = 0;
            while (got < per) {
                let v: int? = ch.recv();
                s = s + (v ?: 0);
                got = got + 1;
            }
            results.send(s);
            return seed;
        }, t));
    }

    for (var h: producers) {
        h.join();
    }
    for (var h: consumers) {
        h.join();
    }

    var total = 0;
    var k = 0;
    while (k < 4) {
        let v: int? = results.recv();
        total = total + (v ?: 0);
        k = k + 1;
    }
    print(total);
    print("producer/consumer OK");
}
