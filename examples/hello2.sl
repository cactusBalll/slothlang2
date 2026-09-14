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
}
