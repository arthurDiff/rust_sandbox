use crate::number::Number;

/// Procedure during which the feature
/// values are rescaled so that they have the properties of a standard normal distribution with
/// mean(the average value of the feature, averaged over all examples in the dataset) = 0 and
/// std_deviation(the standard deviation from the mean) = 1.
pub fn standardization<T: Number>(value: T, mean: T, std_deviation: T) -> f64 {
    (value.to_f64() - mean.to_f64()) / std_deviation.to_f64()
}
