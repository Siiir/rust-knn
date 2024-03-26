//! Unclassified iris functionalities.

use std::borrow::Cow;

use derive_more::{AsMut, AsRef, From, Into};
use nalgebra as na;

use crate::Iris;

/// Unclassified iris.
#[derive(From, Clone, Copy, Debug, PartialEq, AsRef, AsMut, Into)]
pub struct UnclassifiedIris(na::SVector<f32, 4>);

impl UnclassifiedIris {
    pub fn as_na_svec(&self) -> &na::SVector<f32, 4> {
        &self.0
    }
    pub fn as_na_svec_mut(&mut self) -> &mut na::SVector<f32, 4> {
        &mut self.0
    }
}

// CRUD-C: Constructors
impl From<[f32; 4]> for UnclassifiedIris {
    fn from(value: [f32; 4]) -> Self {
        na::SVector::from(value).into()
    }
}
// CRUD-C: Conversions
impl From<FlatUnclassifiedIris> for UnclassifiedIris {
    fn from(record: FlatUnclassifiedIris) -> Self {
        let arr: [f32; 4] = record.into();
        arr.into()
    }
}
// CRUD-C: Deserializers
impl<'de> serde::Deserialize<'de> for UnclassifiedIris {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        FlatUnclassifiedIris::deserialize(deserializer).map(Self::from)
    }
}

// CRUD-R: Getters
impl Iris for UnclassifiedIris {
    fn sepal_length(&self) -> f32 {
        self.0.x
    }

    fn sepal_width(&self) -> f32 {
        self.0.y
    }

    fn petal_length(&self) -> f32 {
        self.0.z
    }

    fn petal_width(&self) -> f32 {
        self.0.w
    }
}

// CRUD-R: Displayers
impl tabled::Tabled for UnclassifiedIris {
    const LENGTH: usize = FlatUnclassifiedIris::LENGTH;

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        FlatUnclassifiedIris::headers()
    }

    fn fields(&self) -> Vec<std::borrow::Cow<'_, str>> {
        FlatUnclassifiedIris::from(*self)
            .fields()
            .into_iter()
            .map(Cow::into_owned)
            .map(Cow::from)
            .collect()
    }
}

/// CRUD-D: Consuming conversions
impl From<UnclassifiedIris> for [f32; 4] {
    fn from(value: UnclassifiedIris) -> Self {
        value.0.into()
    }
}

pub use flat::FlatUnclassifiedIris;
pub mod flat;
