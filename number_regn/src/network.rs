use rand_distr::{Distribution, Normal};

#[derive(Debug)]
pub struct Network<const LC: usize> {
    pub size: [usize; LC],
    pub biases: Vec<Vec<f32>>,
    pub weights: Vec<Vec<Vec<f32>>>,
}

impl<const LC: usize> Network<LC> {
    pub fn new(size: [usize; LC]) -> Self {
        let mut rand = rand::thread_rng();
        let normal = Normal::new(0., 1.).unwrap();
        Self {
            size,
            biases: size[1..]
                .to_vec()
                .iter()
                .map(|v| {
                    let mut bias_vec = vec![0.; *v];
                    bias_vec.fill_with(|| normal.sample(&mut rand));
                    bias_vec
                })
                .collect(),
            weights: size[1..]
                .to_vec()
                .iter()
                .enumerate()
                .map(|(idx, v)| {
                    let weight_size = size[idx];
                    let mut weight_vec = vec![vec![0.; weight_size]; *v];
                    println!("{:?}", weight_size);
                    weight_vec.fill_with(|| {
                        let mut weights = vec![0.; weight_size];
                        weights.fill_with(|| normal.sample(&mut rand));
                        weights
                    });
                    weight_vec
                })
                .collect(),
        }
    }
}
