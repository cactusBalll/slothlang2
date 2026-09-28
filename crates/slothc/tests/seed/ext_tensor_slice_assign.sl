// seed: a range slice in a non-final assignment position keeps rank
// (extension G8)
func main(): unit {
    var d: Tensor<float, 2> = tensor.zeros([2, 3]);
    var row: Tensor<float, 1> = tensor.from_array([5.0, 6.0, 7.0], [3]);
    d[0..2][1] = row;
    print(d[1][0]);                 // expect: 5
}
