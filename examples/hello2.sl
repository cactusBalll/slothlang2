// sloth-lang 2.0 §9.1 example (map is stdlib-future; explicit for keeps the
// semantics identical for the 2.0 regression gold)
pub func main(): unit {
    let names = ["Curry", "Dijkstra", "Benjamin", "Hitori", "foo"];
    for (var name: names) {
        print("Hello , ${name} for 6 times!\n");
        for i in 0..5 {
            print("${name}!");
        }
        print("\n");
    }
    // §2.1/§5.3 map runtime: literal, read/write, len, for-in-keys (patch #10)
    let scores: Map<str, int> = @("Curry": 6, "Dijkstra": 5, "Hitori": 4);
    scores["foo"] = 8;
    print(len(scores));
    for (var e: scores) {
        print("${e.key} = ${e.val}\n");
    }
}
