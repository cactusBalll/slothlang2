// seed: `mutex.with` requires a `(unit) -> unit` closure (extension G5)
func main(): unit {
    let m = mutex.new();
    m.with(|g: int| -> int { return g; });
}
// diag: mutex.with expects a `(unit) -> unit` closure
