// spec: fixed-width & unsigned integer types (`uint`/`int8`/`uint32`/…)
func id_u8(x: uint8): uint8 { return x; }
func main(): unit {
    // `uint` = unsigned 64-bit
    var u: uint = 5;
    print(u);                       // expect: 5
    print(u + 3);                   // expect: 8
    print(u > 3);                   // expect: true
    var z: uint = 0;
    print(z - 1);                   // expect: 18446744073709551615
    print(uint(10) / uint(3));      // expect: 3
    print(uint(10) % uint(3));      // expect: 1
    print(uint(-1));                // expect: 18446744073709551615

    // signed fixed width wraps at its bit width
    var i8: int8 = int8(200);
    print(i8);                      // expect: -56
    print(i8 + 100);               // expect: 44
    print(int8(127) + 1);          // expect: -128
    print(~int8(0));               // expect: -1
    print(int8(-128) - 1);         // expect: 127

    // unsigned fixed width wraps modulo 2^n
    var u8: uint8 = uint8(250);
    print(u8);                     // expect: 250
    print(u8 + 10);               // expect: 4
    print(u8 > 1);                // expect: true
    print(uint8(1) << uint8(7));   // expect: 128
    print(uint8(128) >> uint8(7)); // expect: 1

    // 16/32-bit signed & unsigned
    var i16: int16 = int16(-5);
    print(i16);                    // expect: -5
    var i32: int32 = 70000;
    print(i32 / int32(2));         // expect: 35000
    print(-i32);                   // expect: -70000
    print(uint16(65535));          // expect: 65535
    var u32: uint32 = uint32(4000000000);
    print(u32);                    // expect: 4000000000
    print(u32 / uint32(3));        // expect: 1333333333
    print(u32 > uint32(100));      // expect: true

    // conversions truncate to the target width
    print(int8(300));              // expect: 44
    print(uint8(300));             // expect: 44
    print(int32(5000000000));      // expect: 705032704

    // params/returns coerce to the declared width
    print(id_u8(300));             // expect: 44

    // aliases
    var a: u8 = 1;
    var b: i16 = -2;
    print(a);                      // expect: 1
    print(b);                      // expect: -2
    var c: u64 = 7;
    print(c);                      // expect: 7

    // containers store/emit at the element width
    var arr: Array<uint8> = [];
    arr.push(200);
    arr.push(300);
    print(arr[1]);                 // expect: 44
    var mm: Map<str, uint8> = @("a": 300);
    print(mm["a"]);                // expect: 44
}
