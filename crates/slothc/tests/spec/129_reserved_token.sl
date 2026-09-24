// spec: `import "__sloth";` unlocks the reserved runtime surface (pseudo-import)
import "__sloth";

func main(): unit {
    // an abi.slt-declared runtime symbol; the token authorises the call
    print(__sloth_now_ms() > 0);  // expect: true
}
