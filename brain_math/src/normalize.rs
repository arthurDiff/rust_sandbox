use crate::number::Number;

/// Process of converting an actual range of values which a numerical feature can take, into a standard range of values, typically in the interval [-1, 1] or [0, 1].
pub fn normalize<T: Number>(value: T, min: T, max: T) -> Result<f64, String> {
    let (value_f, min_f, max_f) = (value.to_f64(), min.to_f64(), max.to_f64());
    if min_f >= max_f {
        return Err("Invlalid min or max value".into());
    }
    if value_f > max_f || value_f < min_f {
        return Err("Invalid Value: value should be between min or max".to_string());
    }
    Ok((value_f - min_f) / (max_f - min_f))
}
