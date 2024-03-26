use crate::{Iris, UnclassifiedIris};

pub trait IrisParams {
    // Required
    fn iris_params(&self) -> &UnclassifiedIris;
}

impl<IP> Iris for IP
where
    IP: IrisParams,
{
    fn sepal_length(&self) -> f32 {
        self.iris_params().sepal_length()
    }

    fn sepal_width(&self) -> f32 {
        self.iris_params().sepal_width()
    }

    fn petal_length(&self) -> f32 {
        self.iris_params().petal_length()
    }

    fn petal_width(&self) -> f32 {
        self.iris_params().petal_width()
    }
}
