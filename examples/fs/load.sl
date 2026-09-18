// TE-P3 checkpoint loading demo: 7 i32 config header + f32 weights (design D7).
import "sloth/fs.slt";

func main(): unit {
    var b = open_file("fixture.bin");
    let cfg = read_config(b);
    print(cfg[0]);   // dim
    print(cfg[1]);   // hidden_dim
    print(cfg[5]);   // vocab_size
    print(size(b));

    // f32 weights at byte offset 28, widened to an f64 tensor
    var w: Tensor<float, 1> = view_as_f32(b, 28, 4);
    print(w[0]);
    print(w[3]);
    print(tensor.sum(w));
}
