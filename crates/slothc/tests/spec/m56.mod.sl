// spec: exported globals sibling for 56_globals_mod.sl (not run directly)
pub var counter = 5;
pub var label = "mod";
pub func bump() { counter = counter + 1; }
pub func get(): int { return counter; }
