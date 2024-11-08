/// Connection Vector
#[derive(Debug)]
pub struct CVec(Vec<Vec<f32>>);

impl CVec {
    pub fn new((len, val_count): (usize, usize)) -> Self {
        Self(vec![vec![0.; val_count]; len])
    }

    pub fn fill_with<F>(&mut self, mut f: F)
    where
        F: FnMut() -> f32,
    {
        for r in self.iter_mut() {
            r.fill_with(&mut f);
        }
    }

    fn assert_comparable(lfs: &Self, rhs: &Self) {
        let (lhs_len, rhs_len) = (lfs.len(), rhs.len());
        if lhs_len == 0 || rhs_len == 0 {
            panic!(
                "Cannot multiply lfs CVec of len: {:?} with rhs CVec of len: {:?}",
                lhs_len, rhs_len
            )
        }
        let (lhs_c_len, rhs_c_len) = (lfs[0].len(), rhs[0].len());
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

impl std::ops::Deref for CVec {
    type Target = Vec<Vec<f32>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for CVec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::ops::Mul for CVec {
    type Output = CVec;

    fn mul(mut self, rhs: Self) -> Self::Output {
        Self::assert_comparable(&self, &rhs);

        if rhs[0].len() == 1 {
            self.iter_mut().enumerate().for_each(|(idx, tv)| {
                tv.iter_mut().for_each(|v| {
                    *v *= rhs[idx][0];
                })
            });
            return self;
        }

        self.iter_mut().enumerate().for_each(|(idx, tv)| {
            tv.iter_mut().enumerate().for_each(|(inner_idx, v)| {
                *v *= rhs[idx][inner_idx];
            })
        });
        self
    }
}

impl std::ops::Add for CVec {
    type Output = CVec;

    fn add(mut self, rhs: Self) -> Self::Output {
        Self::assert_comparable(&self, &rhs);

        if rhs[0].len() == 1 {
            self.iter_mut().enumerate().for_each(|(idx, tv)| {
                tv.iter_mut().for_each(|v| {
                    *v += rhs[idx][0];
                })
            });
            return self;
        }

        self.iter_mut().enumerate().for_each(|(idx, tv)| {
            tv.iter_mut().enumerate().for_each(|(inner_idx, v)| {
                *v += rhs[idx][inner_idx];
            })
        });
        self
    }
}
