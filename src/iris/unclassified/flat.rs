use derive_more::Constructor;

use crate::UnclassifiedIris;

#[derive(serde::Deserialize, Constructor, Clone, Copy, Debug, PartialEq, tabled::Tabled)]
pub struct FlatUnclassifiedIris {
    pub sepal_length: f32,
    pub sepal_width: f32,
    pub petal_length: f32,
    pub petal_width: f32,
}

// CRUD-C: Constructors
impl From<[f32; 4]> for FlatUnclassifiedIris {
    fn from([sepal_length, sepal_width, petal_length, petal_width]: [f32; 4]) -> Self {
        Self {
            sepal_length,
            sepal_width,
            petal_length,
            petal_width,
        }
    }
}
// CRUD-C: Conversions
impl From<UnclassifiedIris> for FlatUnclassifiedIris {
    fn from(unclassified_iris: UnclassifiedIris) -> Self {
        let [sepal_length, sepal_width, petal_length, petal_width] = unclassified_iris.into();
        Self {
            sepal_length,
            sepal_width,
            petal_length,
            petal_width,
        }
    }
}
// CRUD-D: Deconstructors
impl From<FlatUnclassifiedIris> for [f32; 4] {
    fn from(record: FlatUnclassifiedIris) -> Self {
        [
            record.sepal_length,
            record.sepal_width,
            record.petal_length,
            record.petal_width,
        ]
    }
}
