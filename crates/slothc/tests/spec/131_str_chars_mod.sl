// spec: str byte subscript + lazy chars() iterator inside an imported module
import "m131.mod.sl" as M;

func main(): unit {
    print(M.codepoint_sum("aé中"));   // expect: 20343
    print(M.second_byte("héllo"));    // expect: 195
    print(M.codepoint_sum(""));       // expect: 0
}
