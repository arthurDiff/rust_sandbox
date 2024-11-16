use std::cmp::Ordering;

/// Connection Vector
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CVec(pub Vec<Vec<f32>>);

impl CVec {
    pub fn new((len, val_count): (usize, usize)) -> Self {
        Self(vec![vec![0.; val_count]; len])
    }
    pub fn dim(&self) -> (usize, usize) {
        (self.0.len(), self.0[0].len())
    }
    pub fn fill_with<F>(&mut self, mut f: F)
    where
        F: FnMut() -> f32,
    {
        for r in self.0.iter_mut() {
            r.fill_with(&mut f);
        }
    }

    pub fn dot(&self, rhs: &Self) -> Self {
        std::ops::Mul::mul(self.clone(), rhs.clone())
    }

    pub fn mul(&self, rhs: f32) -> Self {
        std::ops::Mul::mul(self.clone(), rhs)
    }

    pub fn add(&self, rhs: &Self) -> Self {
        std::ops::Add::add(self.clone(), rhs.clone())
    }
    pub fn sub(&self, rhs: &Self) -> Self {
        std::ops::Sub::sub(self.clone(), rhs.clone())
    }

    pub fn zeroes(&self) -> Self {
        Self(vec![vec![0.; self.0[0].len()]; self.0.len()])
    }

    pub fn index_of_max(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .max_by(|(_, x), (_, y)| x[0].partial_cmp(&y[0]).unwrap_or(Ordering::Equal))
            .unwrap()
            .0
    }

    fn assert_mul_comparable(lhs: &Self, rhs: &Self) {
        let ((lhs_x, lhs_y), (rhs_x, rhs_y)) = (lhs.dim(), rhs.dim());
        if lhs_x == 0 || rhs_x == 0 {
            panic!(
                "AssertMul: cannot multiply lfs CVec of len: {:?} with rhs CVec of len: {:?}",
                lhs_x, rhs_y
            )
        }
        if !(lhs_x == rhs_x && (lhs_y == rhs_y || rhs_y == 1) || lhs_y == rhs_x) {
            panic!(
                "AssertMul: cannot multiply lfs CVec of dim: {:?} with rhs CVec of dim: {:?}",
                (lhs_x, lhs_y),
                (rhs_x, rhs_y)
            )
        }
    }

    fn assert_add_sub_comparable(lhs: &Self, rhs: &Self) {
        let ((lhs_x, lhs_y), (rhs_x, rhs_y)) = (lhs.dim(), rhs.dim());
        if lhs_x == 0 || rhs_x == 0 {
            panic!(
                "AssertAddSub: cannot add or subtract lfs CVec of len: {:?} with rhs CVec of len: {:?}",
                lhs_x, rhs_y
            )
        }
        if !(lhs_x == rhs_x && (lhs_y == rhs_y || rhs_y == 1) || lhs_y == rhs_x && rhs_y == 1) {
            panic!(
                "AssertAddSub: cannot add or subtract lfs CVec of dim: {:?} with rhs CVec of dim: {:?}",
                (lhs_x, lhs_y),
                (rhs_x, rhs_y)
            )
        }
    }
}

impl From<Vec<Vec<f32>>> for CVec {
    fn from(value: Vec<Vec<f32>>) -> Self {
        Self(value)
    }
}

impl std::ops::Mul for CVec {
    type Output = CVec;

    fn mul(mut self, rhs: Self) -> Self::Output {
        Self::assert_mul_comparable(&self, &rhs);
        let ((lhs_x, lhs_y), (rhs_x, rhs_y)) = (self.dim(), rhs.dim());

        if lhs_x == rhs_x && lhs_y == rhs_y {
            self.0.iter_mut().enumerate().for_each(|(idx, tv)| {
                tv.iter_mut().enumerate().for_each(|(inner_idx, v)| {
                    *v *= rhs.0[idx][inner_idx];
                })
            });
            return self;
        }

        if lhs_x == rhs_x && rhs_y == 1 {
            self.0.iter_mut().enumerate().for_each(|(idx, tv)| {
                tv.iter_mut().for_each(|v| {
                    *v *= rhs.0[idx][0];
                })
            });
            return self;
        }

        // lhs_y == rhs_x
        if rhs_y == 1 {
            self.0
                .iter_mut()
                .enumerate()
                .for_each(|(idx, tv)| *tv = vec![tv.iter().sum::<f32>() * rhs.0[idx][0]]);
            return self;
        }

        self.0.iter_mut().for_each(|tv| {
            *tv = (0..lhs_y)
                .map(|idx| {
                    tv.iter()
                        .enumerate()
                        .fold(0., |accu, (inner_idx, v)| accu + v * rhs.0[inner_idx][idx])
                })
                .collect();
        });
        self
    }
}

impl std::ops::Mul<f32> for CVec {
    type Output = CVec;

    fn mul(mut self, rhs: f32) -> Self::Output {
        self.0.iter_mut().for_each(|tv| {
            tv.iter_mut().for_each(|v| {
                *v *= rhs;
            })
        });
        self
    }
}

impl std::ops::Add for CVec {
    type Output = CVec;

    fn add(mut self, rhs: Self) -> Self::Output {
        Self::assert_add_sub_comparable(&self, &rhs);
        std::ops::AddAssign::add_assign(&mut self, rhs);
        self
    }
}

impl std::ops::AddAssign for CVec {
    fn add_assign(&mut self, rhs: Self) {
        Self::assert_add_sub_comparable(self, &rhs);

        let ((lhs_x, lhs_y), (rhs_x, rhs_y)) = (self.dim(), rhs.dim());

        println!("{:?} == add assign", ((lhs_x, lhs_y), (rhs_x, rhs_y)));
        if lhs_x == rhs_x && lhs_y == rhs_y {
            self.0.iter_mut().enumerate().for_each(|(idx, tv)| {
                tv.iter_mut().enumerate().for_each(|(inner_idx, v)| {
                    *v += rhs.0[idx][inner_idx];
                })
            });
            return;
        }

        if lhs_x == rhs_x && rhs_y == 1 {
            self.0.iter_mut().enumerate().for_each(|(idx, tv)| {
                tv.iter_mut().for_each(|v| {
                    *v += rhs.0[idx][0];
                })
            });
            return;
        }

        //lhs_y == rhs_x
        self.0.iter_mut().for_each(|tv| {
            tv.iter_mut().enumerate().for_each(|(idx, v)| {
                *v += rhs.0[idx][0];
            })
        });
    }
}

impl std::ops::Sub for CVec {
    type Output = CVec;

    fn sub(mut self, rhs: Self) -> Self::Output {
        Self::assert_add_sub_comparable(&self, &rhs);
        std::ops::SubAssign::sub_assign(&mut self, rhs);
        self
    }
}

impl std::ops::SubAssign for CVec {
    fn sub_assign(&mut self, rhs: Self) {
        Self::assert_add_sub_comparable(self, &rhs);

        let ((lhs_x, lhs_y), (rhs_x, rhs_y)) = (self.dim(), rhs.dim());

        if lhs_x == rhs_x && lhs_y == rhs_y {
            self.0.iter_mut().enumerate().for_each(|(idx, tv)| {
                tv.iter_mut().enumerate().for_each(|(inner_idx, v)| {
                    *v -= rhs.0[idx][inner_idx];
                })
            });
            return;
        }

        if lhs_x == rhs_x && rhs_y == 1 {
            self.0.iter_mut().enumerate().for_each(|(idx, tv)| {
                tv.iter_mut().for_each(|v| {
                    *v -= rhs.0[idx][0];
                })
            });
            return;
        }

        //lhs_y == rhs_x
        self.0.iter_mut().for_each(|tv| {
            tv.iter_mut().enumerate().for_each(|(idx, v)| {
                *v -= rhs.0[idx][0];
            })
        });
    }
}

impl std::ops::Sub<f32> for CVec {
    type Output = CVec;

    fn sub(mut self, rhs: f32) -> Self::Output {
        self.0.iter_mut().for_each(|tv| {
            tv.iter_mut().for_each(|v| {
                *v -= rhs;
            })
        });
        self
    }
}

impl std::ops::SubAssign<f32> for CVec {
    fn sub_assign(&mut self, rhs: f32) {
        self.0.iter_mut().for_each(|tv| {
            tv.iter_mut().for_each(|v| {
                *v -= rhs;
            })
        });
    }
}

impl std::ops::Sub<CVec> for f32 {
    type Output = CVec;

    fn sub(self, mut rhs: CVec) -> Self::Output {
        rhs.0.iter_mut().for_each(|tv| {
            tv.iter_mut().for_each(|v| {
                *v -= self;
            })
        });
        rhs
    }
}

pub trait NetworkCVec {
    fn dim(&self) -> Vec<(usize, usize)>;
}

impl NetworkCVec for Vec<CVec> {
    fn dim(&self) -> Vec<(usize, usize)> {
        self.iter()
            .map(|w| w.dim())
            .collect::<Vec<(usize, usize)>>()
    }
}
