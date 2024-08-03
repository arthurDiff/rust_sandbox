pub trait Number {
    fn to_f64(&self) -> f64;
}

impl Number for f64 {
    fn to_f64(&self) -> f64 {
        *self
    }
}
impl Number for f32 {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
}
impl Number for i8 {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
}
impl Number for i16 {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
}
impl Number for i32 {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
}
impl Number for i64 {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
}
impl Number for i128 {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
}
impl Number for u8 {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
}
impl Number for u16 {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
}

impl Number for u32 {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
}
impl Number for u64 {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
}
impl Number for u128 {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
}
impl Number for usize {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
}
