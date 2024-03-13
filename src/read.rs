//! Dedicated to reading data.

use anyhow::Context;

use crate::{
    app_cfg, read, ClassifiedIris, UnclassifiedIris, PATH_TO_TESTING_IRISES,
    PATH_TO_TRAINING_IRISES,
};

// Helpful local type aliases.
type UnclassifiedIrises = Vec<UnclassifiedIris>;
type ClassifiedIrises = Vec<ClassifiedIris>;

/// Reads users data from stdin.
///
/// Returned error reports what failed, not only why.
pub fn user_irises() -> anyhow::Result<UnclassifiedIrises> {
    let mut irises_reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(app_cfg().separator.into())
        .from_reader(std::io::stdin().lock());
    let irises: Result<Vec<_>, _> = irises_reader.deserialize().collect();
    Ok(irises.context("Failed to read unclassified irises from stdin.")?)
}

/// Reads the training data from pre-existing file in [`PATH_TO_TRAINING_IRISES`].
///
/// Returned error reports what failed, not only why.
pub fn training_irises() -> anyhow::Result<ClassifiedIrises> {
    (|| -> Result<_, _> {
        let mut irises_reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(PATH_TO_TRAINING_IRISES)?;
        let result: Result<Vec<_>, _> = irises_reader.deserialize().collect();
        result
    })()
    .with_context(|| {
        format!("Failed to read classified irises from \"{PATH_TO_TRAINING_IRISES}\".")
    })
}

/// Returned error reports what failed, not only why.
pub fn testing_irises() -> anyhow::Result<ClassifiedIrises> {
    (|| -> Result<_, _> {
        let mut irises_reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(PATH_TO_TESTING_IRISES)?;
        let result: Result<Vec<_>, _> = irises_reader.deserialize().collect();
        result
    })()
    .with_context(|| format!("Failed to read testing irises from \"{PATH_TO_TESTING_IRISES}\"."))
}

#[deprecated]
pub fn data() -> anyhow::Result<(UnclassifiedIrises, ClassifiedIrises)> {
    // Reading both data sets in parallel.
    let (unclassified_irises, classified_irises) =
        rayon::join(|| read::user_irises(), || read::training_irises());
    // Passing errors up with some context.
    (|| -> anyhow::Result<_> { Ok((unclassified_irises?, classified_irises?)) })()
        .context("Reading of the data failed.")
}
