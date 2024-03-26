//! Defines recognized iris classifications (species).

use serde::{Deserialize, Deserializer};
use strum_macros as sm;

#[derive(
    Clone,
    Copy,
    num_enum::TryFromPrimitive,
    sm::AsRefStr,
    sm::IntoStaticStr,
    Debug,
    sm::Display,
    PartialEq,
    Eq,
    Hash,
    num_enum::IntoPrimitive,
)]
#[repr(u8)]
pub enum IrisSpecies {
    Setosa = 0,
    Versicolor = 1,
    Virginica = 2,
}
impl<'de> Deserialize<'de> for IrisSpecies {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let iris_code = u8::deserialize(deserializer)?;
        num_enum::TryFromPrimitive::try_from_primitive(iris_code).map_err(serde::de::Error::custom)
    }
}
