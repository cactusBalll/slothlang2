// seed: `fiber.error` message is checked as `str` (bug B4)
func main(): unit {
    fiber.error(5);
}
// diag: fiber.error message
