//! Classified iris functionalities.

use derive_more::{Constructor, Deref, DerefMut};
use serde::{Deserialize, Deserializer};
use tabled::Tabled;

use crate::{IrisParams, IrisSpecies, UnclassifiedIris};

/// Classified iris.
///
/// Inherits many functionalities from unclassified iris.
#[derive(Constructor, Clone, Copy, Debug, PartialEq, Deref, DerefMut)]
pub struct ClassifiedIris {
    #[deref]
    #[deref_mut]
    pub parameters: UnclassifiedIris,
    pub classification: IrisSpecies,
}
// CRUD-C: Conversions
impl From<FlatClassifiedIris> for ClassifiedIris {
    fn from(value: FlatClassifiedIris) -> Self {
        let FlatClassifiedIris {
            sepal_length,
            sepal_width,
            petal_length,
            petal_width,
            classification,
        } = value;
        Self {
            parameters: UnclassifiedIris::from([
                sepal_length,
                sepal_width,
                petal_length,
                petal_width,
            ]),
            classification,
        }
    }
}
// CRUD-C: Deserializers
impl<'de> Deserialize<'de> for ClassifiedIris {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        FlatClassifiedIris::deserialize(deserializer).map(Self::from)
    }
}
// CRUD-R: Getters
impl IrisParams for ClassifiedIris {
    fn iris_params(&self) -> &UnclassifiedIris {
        &self.parameters
    }
}
// CRUD-R: Displayers
impl Tabled for ClassifiedIris {
    const LENGTH: usize = <UnclassifiedIris as Tabled>::LENGTH + 1;

    fn fields(&self) -> Vec<std::borrow::Cow<'_, str>> {
        let mut parameters = self.parameters.fields();
        parameters.push(self.classification.as_ref().into());
        parameters
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        let mut parameters = <UnclassifiedIris as Tabled>::headers();
        parameters.push("species".into());
        parameters
    }
}

pub use flat::FlatClassifiedIris;
pub mod flat;
