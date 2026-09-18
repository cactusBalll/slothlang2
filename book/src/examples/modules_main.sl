import "modules_lib.sl" as lib;

func main() {
    print(lib.double(21));       // 42
    print(lib.counter);          // 5  跨模块读 pub var
}
