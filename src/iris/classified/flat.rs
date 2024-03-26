use derive_more::Constructor;

use crate::{iris::unclassified::FlatUnclassifiedIris, ClassifiedIris};

#[derive(serde::Deserialize, Constructor, Clone, Copy, Debug, PartialEq, tabled::Tabled)]
pub struct FlatClassifiedIris {
    pub sepal_length: f32,
    pub sepal_width: f32,
    pub petal_length: f32,
    pub petal_width: f32,
    pub classification: crate::IrisSpecies,
}

// CRUD-C: Conversions
impl From<ClassifiedIris> for FlatClassifiedIris {
    fn from(deep: ClassifiedIris) -> Self {
        let ClassifiedIris {
            parameters,
            classification,
        } = deep;
        let FlatUnclassifiedIris {
            sepal_length,
            sepal_width,
            petal_length,
            petal_width,
        } = parameters.into();
        Self {
            sepal_length,
            sepal_width,
            petal_length,
            petal_width,
            classification,
        }
    }
}
