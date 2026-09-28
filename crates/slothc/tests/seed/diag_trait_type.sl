// seed: a trait name in a type position must be spelled `dyn Trait`
// (oop / modules D3)
trait Animal { func speak(): str; }
class Dog impl Animal { func speak(): str { return "w"; } }
func main(): unit {
    var a: Animal = Dog();
    print(a.speak());
}
// diag: cannot be used as a type
