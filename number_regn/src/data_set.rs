use crate::network::cvec::CVec;

/// Vec<(input, output)>
pub type Dataset = Vec<(CVec, CVec)>;
pub type TestDataSet = Vec<(CVec, f32)>;
pub struct MnistDataset {
    pub training_set: Dataset,
    pub test_set: TestDataSet,
}

impl Default for MnistDataset {
    fn default() -> Self {
        let mnist = rust_mnist::Mnist::new("number_regn/data/");
        Self {
            training_set: mnist
                .train_data
                .iter()
                .zip(mnist.train_labels)
                .map(|(d, l)| {
                    (
                        CVec::from(d.map(|v| vec![v as f32]).to_vec()),
                        CVec::from(
                            (0..10)
                                .map(|v| if v == l { vec![1.] } else { vec![0.] })
                                .collect::<Vec<Vec<f32>>>(),
                        ),
                    )
                })
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
