use rand_distr::{Distribution, Normal};
use value::CVec;

pub mod value;
#[derive(Debug)]
pub struct Network<const LC: usize> {
    pub size: [usize; LC],
    pub biases: Vec<CVec>,
    pub weights: Vec<CVec>,
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
                    let mut bias_cvec = CVec::new((*v, 1));
                    bias_cvec.fill_with(|| normal.sample(&mut rand));
                    bias_cvec
                })
                .collect(),
            weights: size[1..]
                .to_vec()
                .iter()
                .enumerate()
                .map(|(idx, v)| {
                    let mut weight_cvec = CVec::new((*v, size[idx]));
                    weight_cvec.fill_with(|| normal.sample(&mut rand));
                    weight_cvec
                })
                .collect(),
        }
    }

    pub fn feedforward(self, input: CVec) -> CVec {
        todo!()
    }
}
