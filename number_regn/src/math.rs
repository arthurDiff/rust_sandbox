use crate::network::cvec::CVec;

pub struct Math;

impl Math {
    pub fn sigmoid(input: &CVec) -> CVec {
        input
            .0
            .iter()
            .map(|v| v.iter().map(|v| 1. / (1. + f32::exp(-1. * v))).collect())
            .collect::<Vec<Vec<f32>>>()
            .into()
    }
}
