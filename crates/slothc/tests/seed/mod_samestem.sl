// seed: two distinct files with the same stem must not collide (module M8 / F8)
import "m8/lib.slt" as A;
import "m8sub/lib.slt" as B;
func main(): unit {
    print(A.f());   // expect: 100
    print(B.f());   // expect: 200
}
