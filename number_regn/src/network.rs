use cvec::CVec;
use rand::seq::SliceRandom;
use rand_distr::{Distribution, Normal};

use crate::{data_set::Dataset, math::Math};

pub mod cvec;

#[derive(Debug)]
pub struct Network<const LC: usize> {
    pub size: [usize; LC],
    pub biases: Vec<CVec>,
    pub weights: Vec<CVec>,
}

impl<const LC: usize> Network<LC> {
    pub fn new(size: [usize; LC]) -> Self {
        if LC < 3 {
            panic!("Network: Can't create network with layers less then 3");
        }
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

    /// Stochastic Gradient Descent
    /// trainin_data: Vec<(input, output)>
    /// epoch: number of epochs to train for
    /// mini_batch_size: size of mini-batches to use when sampling
    /// eta: learning rate
    /// test_data: Option<Vec<(input, output)>>
    pub fn sgd(
        &mut self,
        mut training_data: Dataset,
        epoch: usize,
        mini_batch_size: usize,
        eta: f32,
        test_data: Option<Dataset>,
    ) {
        let mut rand = rand::thread_rng();
        for e in 0..epoch {
            training_data.shuffle(&mut rand);
            training_data.chunks(mini_batch_size).for_each(|mb| {
                self.update_batch(mb, eta);
            });

            if let Some(_td) = test_data.as_ref() {
                todo!()
            }
            println!("Epoch: {} complete", e)
        }
    }

    /// batch: &[(input, output)]
    /// eta: learning rate
    fn update_batch(&mut self, batch: &[(CVec, f32)], eta: f32) {
        let batch_size = batch.len() as f32;
        let (mut nabla_w, mut nabla_b) = (self.zeroed_weight(), self.zeroed_biases());

        batch.iter().for_each(|(input, output)| {
            let (delta_nabla_w, delta_nabla_b) = self.backprop(input.clone(), *output);
            nabla_w.iter_mut().zip(delta_nabla_w).for_each(|(nw, dnw)| {
                *nw += dnw;
            });
            nabla_b.iter_mut().zip(delta_nabla_b).for_each(|(nb, dnb)| {
                *nb += dnb;
            });
        });

        self.weights
            .iter_mut()
            .zip(nabla_w.iter())
            .for_each(|(w, nw)| *w -= nw.mul(eta / batch_size));

        self.biases
            .iter_mut()
            .zip(nabla_b.iter())
            .for_each(|(b, nb)| *b -= nb.mul(eta / batch_size))
    }

    /// Back Propagation
    /// returns: (nabla_biases, nabla_weights) - The gradient for the cost function C_x
    fn backprop(&self, mut input: CVec, output: f32) -> (Vec<CVec>, Vec<CVec>) {
        let (mut nabla_w, mut nabla_b) = (self.zeroed_weight(), self.zeroed_biases());
        // (activation, activations, zvectors)
        let (act_vec, z_vec) = self.weights.iter().zip(self.biases.iter()).fold(
            (vec![input.clone()], vec![]),
            |mut accu, (w, b)| {
                let z = w.dot(&input).add(b);
                // activation
                input = Math::sigmoid(&z);
                // activations
                accu.0.push(input.clone());
                // z vec
                accu.1.push(z);

                accu
            },
        );

        let mut delta = Self::cost_derivative(&act_vec[LC - 1], output)
            * Math::sigmoid_prime(z_vec.last().unwrap());
        *nabla_b.last_mut().unwrap() = delta.clone();
        *nabla_w.last_mut().unwrap() = delta.clone() * act_vec[act_vec.len() - 2].clone();

        (2..LC).for_each(|l| {
            let sp = Math::sigmoid_prime(&z_vec[z_vec.len() - l]);
            delta = self.weights[LC - l].dot(&delta) * sp;
            nabla_b[LC - l - 1] = delta.clone();
            nabla_w[LC - l - 1] = delta.dot(&act_vec[LC - l]);
        });

        (nabla_w, nabla_b)
    }

    // test_data: &[(input, output)]
    pub fn evaluate(&self, test_data: &[(CVec, f32)]) -> usize {
        let mut correct_answer_c = 0;
        test_data.iter().for_each(|(x, y)| {
            if self.feedforward(x).index_of_max() != *y as usize {
                return;
            }
            correct_answer_c += 1;
        });
        correct_answer_c
    }

    pub fn feedforward(&self, input: &CVec) -> CVec {
        self.weights
            .iter()
            .zip(self.biases.iter())
            .fold(input.clone(), |accu, (w, b)| {
                Math::sigmoid(&(w.dot(&accu).add(b)))
            })
    }

    fn zeroed_weight(&self) -> Vec<CVec> {
        self.weights.iter().map(|w| w.zeroes()).collect()
    }

    fn zeroed_biases(&self) -> Vec<CVec> {
        self.biases.iter().map(|w| w.zeroes()).collect()
    }

    fn cost_derivative(output_activations: &CVec, output: f32) -> CVec {
        output_activations.clone() - output
    }
}
