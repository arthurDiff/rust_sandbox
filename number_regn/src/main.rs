use number_regn::math::Math;

// https://github.com/mnielsen/neural-networks-and-deep-learning/tree/master
// http://neuralnetworksanddeeplearning.com/chap1.html#a_simple_network_to_classify_handwritten_digits
fn main() {
    let nw = number_regn::network::Network::new([2, 3, 1]);
    println!("{:#?}", nw);
}
