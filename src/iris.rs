use crate::UnclassifiedIris;

/// Iris functionalities that are independent of classification or lack of it.
pub trait Iris {
    // Required
    fn data(&self) -> &UnclassifiedIris;
    // Provided
    fn dist_sq(&self, other: &dyn Iris) -> f32 {
        let lhs = self.data();
        let rhs = other.data();
        [
            (lhs.sepal_length - rhs.sepal_length),
            (lhs.sepal_width - rhs.sepal_width),
            (lhs.petal_length - rhs.petal_length),
            (lhs.petal_width - rhs.petal_width),
        ]
        .into_iter()
        .map(|x| x * x)
        .sum()
    }
}

pub mod species {
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
            num_enum::TryFromPrimitive::try_from_primitive(iris_code)
                .map_err(serde::de::Error::custom)
        }
    }
}
pub mod unclassified {
    //! Unclassified iris functionalities.
    use crate::Iris;
    use derive_more::Constructor;
    use nalgebra as na;

    /// Unclassified iris.
    #[derive(serde::Deserialize, Constructor, Clone, Copy, Debug, PartialEq, tabled::Tabled)]
    pub struct UnclassifiedIris {
        pub sepal_length: f32,
        pub sepal_width: f32,
        pub petal_length: f32,
        pub petal_width: f32,
    }
    impl Iris for UnclassifiedIris {
        fn data(&self) -> &UnclassifiedIris {
            self
        }
    }
    impl From<UnclassifiedIris> for na::SVector<f32, 4> {
        fn from(iris: UnclassifiedIris) -> Self {
            [
                iris.sepal_length,
                iris.sepal_width,
                iris.petal_length,
                iris.petal_width,
            ]
            .into()
        }
    }
}
pub mod classified {
    //! Classified iris functionalities.

    use derive_more::{Constructor, Deref, DerefMut};
    use num_enum::TryFromPrimitive;
    use serde::{Deserialize, Deserializer};
    use tabled::Tabled;

    use crate::{Iris, IrisSpecies, UnclassifiedIris};

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
    impl Iris for ClassifiedIris {
        fn data(&self) -> &UnclassifiedIris {
            &self.parameters
        }
    }
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
    impl<'de> Deserialize<'de> for ClassifiedIris {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct IrisVisitor;

            impl<'de> serde::de::Visitor<'de> for IrisVisitor {
                type Value = ClassifiedIris;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("struct ClassifiedIris")
                }

                fn visit_seq<V>(self, mut seq: V) -> Result<ClassifiedIris, V::Error>
                where
                    V: serde::de::SeqAccess<'de>,
                {
                    let sepal_length = seq
                        .next_element()?
                        .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                    let sepal_width = seq
                        .next_element()?
                        .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                    let petal_length = seq
                        .next_element()?
                        .ok_or_else(|| serde::de::Error::invalid_length(2, &self))?;
                    let petal_width = seq
                        .next_element()?
                        .ok_or_else(|| serde::de::Error::invalid_length(3, &self))?;
                    let classification_code: u8 = seq
                        .next_element()?
                        .ok_or_else(|| serde::de::Error::invalid_length(4, &self))?;
                    let classification = IrisSpecies::try_from_primitive(classification_code)
                        .map_err(serde::de::Error::custom)?;

                    Ok(ClassifiedIris {
                        parameters: UnclassifiedIris {
                            sepal_length,
                            sepal_width,
                            petal_length,
                            petal_width,
                        },
                        classification,
                    })
                }
            }

            deserializer.deserialize_tuple(5, IrisVisitor)
        }
    }
}
