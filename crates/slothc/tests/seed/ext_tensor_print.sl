// ext: print and type_name of tensors (any rank / element kind).
func main(): unit {
    var f2: Tensor<float, 2> = tensor.from_array([1.0, 2.0, 3.0, 4.0], [2, 2]);
    print(f2);
    print(type_name(f2));
    var f1: Tensor<float, 1> = tensor.zeros([3]);
    print(f1);
    var i1: Tensor<int, 1> = tensor.from_array([7, 8], [2]);
    print(i1);
    print(type_name(i1));
    var f3: Tensor<float, 3> = tensor.zeros([2, 1, 2]);
    print(f3);
    // expects
    // expect: tensor([[1, 2], [3, 4]])
    // expect: Tensor<float, 2>
    // expect: tensor([0, 0, 0])
    // expect: tensor([7, 8])
    // expect: Tensor<int, 1>
    // expect: tensor([[[0, 0]], [[0, 0]]])
}
