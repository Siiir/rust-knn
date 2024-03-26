/// Iris functionalities that are independent of classification or lack of it.
pub trait Iris {
    // Required
    fn sepal_length(&self) -> f32;
    fn sepal_width(&self) -> f32;
    fn petal_length(&self) -> f32;
    fn petal_width(&self) -> f32;

    // Provided
    fn dist_sq(&self, other: &dyn Iris) -> f32 {
        let lhs = self;
        let rhs = other;
        [
            (lhs.sepal_length() - rhs.sepal_length()),
            (lhs.sepal_width() - rhs.sepal_width()),
            (lhs.petal_length() - rhs.petal_length()),
            (lhs.petal_width() - rhs.petal_width()),
        ]
        .into_iter()
        .map(|x| x * x)
        .sum()
    }
}

pub mod classified;
pub mod params;
pub mod species;
pub mod unclassified;
