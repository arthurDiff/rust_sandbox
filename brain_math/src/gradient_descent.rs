#[derive(Clone, Debug)]
pub struct DataSet {
    pub x: f64,
    pub y: f64,
}

trait SetLength {
    fn set_len(&self) -> f64;
}

impl SetLength for Vec<DataSet> {
    fn set_len(&self) -> f64 {
        self.len() as f64
    }
}

impl SetLength for [DataSet] {
    fn set_len(&self) -> f64 {
        self.len() as f64
    }
}

/// wx - b = 0
#[derive(Clone, Debug)]
pub struct HyperplaneParameters {
    ///A real-valued vector 'w' of the same dimensionality as input feature vector x
    ///A vector of weights that symbolizes a vector normal to the hyperplane
    pub w: f64,
    ///An offset or bias term that represents the hyperplane's position within the input space
    pub b: f64,
}

#[derive(Clone, Debug)]
pub struct GradientDescent {
    data: Vec<DataSet>,
    hp: HyperplaneParameters,
}

impl GradientDescent {
    pub fn new(data: Vec<DataSet>, hp: Option<HyperplaneParameters>) -> Self {
        Self {
            data,
            hp: hp.unwrap_or(HyperplaneParameters { w: 0., b: 0. }),
        }
    }

    pub fn predict(&self, x: f64) -> f64 {
        self.hp.w * x + self.hp.b
    }

    /// loops over multiple epochs is shown below:
    /// One pass through all training examples is called an epoch.
    pub fn train(&mut self, alpha: f64, epoch: usize) {
        for e in 0..epoch {
            self.hp = Self::update_w_and_b(&self.data, self.hp.clone(), alpha);

            if e % 400 == 0 {
                println!(
                    "epoch: {}; loss: {}",
                    e,
                    Self::avg_loss(&self.data, &self.hp)
                );
            }
        }
    }

    /// initialize2 w = 0 and b = 0 and then iterate through our training examples,
    /// alpha = learning rate
    fn update_w_and_b(
        data: &[DataSet],
        hp: HyperplaneParameters,
        alpha: f64,
    ) -> HyperplaneParameters {
        let set_len = data.set_len();
        let (mut dl_dw, mut dl_db) = (0., 0.);

        for d_set in data {
            dl_dw += -2. * d_set.x * (d_set.y - (hp.w * d_set.x + hp.b));
            dl_db += -2. * (d_set.y - (hp.w * d_set.x + hp.b));
        }

        HyperplaneParameters {
            w: hp.w - (1. / set_len) * dl_dw * alpha,
            b: hp.b - (1. / set_len) * dl_db * alpha,
        }
    }

    /// computes the mean squared error
    fn avg_loss(data: &[DataSet], hp: &HyperplaneParameters) -> f64 {
        let set_len = data.set_len();
        let mut total_err = 0.;

        for d_set in data {
            total_err += f64::sqrt(d_set.y - (hp.w * d_set.x + hp.b));
        }

        total_err / set_len
    }
}
