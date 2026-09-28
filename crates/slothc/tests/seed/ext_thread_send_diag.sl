// ext: `Send` marker — a `Fiber<Y>` may not cross a thread boundary
// (thread.spawn payload) nor be a channel element.
func main(): unit {
    let f = fiber.create(|x: int| -> unit {
        fiber.yield(x);
    }, 0);
    let h = thread.spawn(|g: Fiber<int>| -> int {
        return 0;
    }, f);
    h.join();
    // diag: Send
}
