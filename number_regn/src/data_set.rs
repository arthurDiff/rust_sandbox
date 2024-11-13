use crate::network::cvec::CVec;

/// Vec<(input, output)>
pub type Dataset = Vec<(CVec, f32)>;

pub struct MnistDataset {
    pub training_set: Dataset,
    pub test_set: Dataset,
}

impl Default for MnistDataset {
    fn default() -> Self {
        let mnist = rust_mnist::Mnist::new("number_regn/data/");
        Self {
            training_set: mnist
                .train_data
                .iter()
                .zip(mnist.train_labels)
                .map(|(d, l)| (CVec::from(d.map(|v| vec![v as f32]).to_vec()), l as f32))
                .collect(),
            test_set: mnist
                .test_data
                .iter()
                .zip(mnist.test_labels)
                .map(|(d, l)| (CVec::from(d.map(|v| vec![v as f32]).to_vec()), l as f32))
                .collect(),
        }
    }
}
impl MnistDataset {
    pub fn new() -> Self {
        Self::default()
    }
}
