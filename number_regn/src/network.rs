use std::collections;

use cvec::CVec;
use rand::seq::SliceRandom;
use rand_distr::{Distribution, Normal};

use crate::math::Math;

pub mod cvec;

/// Vec<(input, output)>
type Dataset = Vec<(f32, f32)>;
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

            if let (Some(_td)) = test_data.as_ref() {
                todo!()
            }
            println!("Epoch: {} complete", e)
        }
    }

    /// batch: &[(input, output)]
    /// eta: learning rate
    fn update_batch(&mut self, batch: &[(f32, f32)], eta: f32) {
        let batch_size = batch.len() as f32;
        let (mut nabla_w, mut nabla_b) = (
            self.weights
                .iter()
                .map(|w| w.zeroes())
                .collect::<Vec<CVec>>(),
            self.biases
                .iter()
                .map(|b| b.zeroes())
                .collect::<Vec<CVec>>(),
        );
        batch.iter().for_each(|(input, output)| {
            let (delta_nabla_w, delta_nabla_b) = self.backprop(*input, *output);
            nabla_w.iter_mut().zip(delta_nabla_w).for_each(|(nw, dnw)| {
                *nw = nw.add(&dnw);
            });
            nabla_b.iter_mut().zip(delta_nabla_b).for_each(|(nb, dnb)| {
                *nb = nb.add(&dnb);
            });
        });
        self.weights
            .iter_mut()
            .zip(nabla_w.iter())
            .for_each(|(w, nw)| *w = w.clone() - nw.clone() * (eta / batch_size));

        self.biases
            .iter_mut()
            .zip(nabla_b.iter())
            .for_each(|(b, nb)| *b = b.clone() - nb.clone() * (eta / batch_size))
    }

    /// Back Propagation
    /// returns: (nabla_biases, nabla_weights)
    fn backprop(&self, input: f32, output: f32) -> (Vec<CVec>, Vec<CVec>) {
        todo!()
    }

    pub fn feedforward(&self, input: CVec) -> CVec {
        self.weights
            .iter()
            .zip(self.biases.iter())
            .fold(input, |accu, (w, b)| Math::sigmoid(&(w.dot(&accu).add(b))))
    }
}
