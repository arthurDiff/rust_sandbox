use std::cmp::Ordering;

/// Connection Vector
#[derive(Debug, Clone)]
pub struct CVec(pub Vec<Vec<f32>>);

impl CVec {
    pub fn new((len, val_count): (usize, usize)) -> Self {
        Self(vec![vec![0.; val_count]; len])
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
        Self(vec![vec![0.; self.0[0].len()]])
    }

    pub fn index_of_max(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .max_by(|(_, x), (_, y)| x[0].partial_cmp(&y[0]).unwrap_or(Ordering::Equal))
            .unwrap()
            .0
    }

    fn assert_comparable(lfs: &Self, rhs: &Self) {
        let (lhs_len, rhs_len) = (lfs.0.len(), rhs.0.len());
        if lhs_len == 0 || rhs_len == 0 {
            panic!(
                "Cannot multiply lfs CVec of len: {:?} with rhs CVec of len: {:?}",
                lhs_len, rhs_len
            )
        }
        let (lhs_c_len, rhs_c_len) = (lfs.0[0].len(), rhs.0[0].len());
        if lhs_len != rhs_len || rhs_c_len == 1 || lhs_c_len != rhs_c_len {
            panic!(
                "Cannot multiply lfs CVec of dim: {:?} with rhs CVec of dim: {:?}",
                (lhs_len, lhs_c_len),
                (rhs_len, rhs_c_len)
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
        Self::assert_comparable(&self, &rhs);

        if rhs.0[0].len() == 1 {
            self.0.iter_mut().enumerate().for_each(|(idx, tv)| {
                tv.iter_mut().for_each(|v| {
                    *v *= rhs.0[idx][0];
                })
            });
            return self;
        }

        self.0.iter_mut().enumerate().for_each(|(idx, tv)| {
            tv.iter_mut().enumerate().for_each(|(inner_idx, v)| {
                *v *= rhs.0[idx][inner_idx];
            })
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
        Self::assert_comparable(&self, &rhs);

        if rhs.0[0].len() == 1 {
            self.0.iter_mut().enumerate().for_each(|(idx, tv)| {
                tv.iter_mut().for_each(|v| {
                    *v += rhs.0[idx][0];
                })
            });
            return self;
        }

        self.0.iter_mut().enumerate().for_each(|(idx, tv)| {
            tv.iter_mut().enumerate().for_each(|(inner_idx, v)| {
                *v += rhs.0[idx][inner_idx];
            })
        });
        self
    }
}
impl std::ops::AddAssign for CVec {
    fn add_assign(&mut self, rhs: Self) {
        Self::assert_comparable(self, &rhs);

        if rhs.0[0].len() == 1 {
            self.0.iter_mut().enumerate().for_each(|(idx, tv)| {
                tv.iter_mut().for_each(|v| {
                    *v += rhs.0[idx][0];
                })
            });
            return;
        }

        self.0.iter_mut().enumerate().for_each(|(idx, tv)| {
            tv.iter_mut().enumerate().for_each(|(inner_idx, v)| {
                *v += rhs.0[idx][inner_idx];
            })
        });
    }
}

impl std::ops::Sub for CVec {
    type Output = CVec;

    fn sub(mut self, rhs: Self) -> Self::Output {
        Self::assert_comparable(&self, &rhs);

        if rhs.0[0].len() == 1 {
            self.0.iter_mut().enumerate().for_each(|(idx, tv)| {
                tv.iter_mut().for_each(|v| {
                    *v -= rhs.0[idx][0];
                })
            });
            return self;
        }

        self.0.iter_mut().enumerate().for_each(|(idx, tv)| {
            tv.iter_mut().enumerate().for_each(|(inner_idx, v)| {
                *v -= rhs.0[idx][inner_idx];
            })
        });
        self
    }
}

impl std::ops::SubAssign for CVec {
    fn sub_assign(&mut self, rhs: Self) {
        Self::assert_comparable(self, &rhs);

        if rhs.0[0].len() == 1 {
            self.0.iter_mut().enumerate().for_each(|(idx, tv)| {
                tv.iter_mut().for_each(|v| {
                    *v -= rhs.0[idx][0];
                })
            });
            return;
        }

        self.0.iter_mut().enumerate().for_each(|(idx, tv)| {
            tv.iter_mut().enumerate().for_each(|(inner_idx, v)| {
                *v -= rhs.0[idx][inner_idx];
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
