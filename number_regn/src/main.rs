use number_regn::data_set::MnistDataset;

const IMAGE_ROWS: usize = 28;
const IMAGE_COLUMNS: usize = 28;
// http://neuralnetworksanddeeplearning.com/chap1.html#a_simple_network_to_classify_handwritten_digits
fn main() {
    let ds = MnistDataset::new();
    println!("Mnist dataset ready");
    let mut nw = number_regn::network::Network::new(&[IMAGE_ROWS * IMAGE_COLUMNS, 30, 10]);
    nw.sgd(ds.training_set, 30, 10, 3., Some(ds.test_set));
    let _ = nw.save_as_json("number_recgn".into());
}
