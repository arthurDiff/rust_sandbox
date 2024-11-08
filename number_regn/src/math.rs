use crate::network::value::CVec;

pub struct Math;

impl Math {
    pub fn sigmoid(input: &CVec) -> CVec {
        input
            .iter()
            .map(|v| v.iter().map(|v| 1. / (1. + f32::exp(-1. * v))).collect())
            .collect::<Vec<Vec<f32>>>()
            .into()
    }
}
