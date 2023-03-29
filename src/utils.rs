/// Trait for validating a string
pub trait Validation<T> {
    fn validate(s: T) -> bool;
}
