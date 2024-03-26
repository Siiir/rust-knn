use num_rational::Ratio;

use crate::{ClassifiedIris, UnclassifiedIris};

pub mod args {
    //! Facilitates usage of this app's arguments.

    use crate::util;

    #[derive(clap::Parser, Debug)]
    #[command(version, about, long_about = crate::executable_desc!())]
    pub struct AppArgs {
        #[cfg(feature = "knn")]
        /// `k` that is used in KNN algorithm used by this iris classifier.
        #[arg(short, long, default_value_t = 3)]
        pub k_for_knn: usize,

        /// Delimiter used for provided floating point values.
        ///
        /// Iris data should be provided in CSV format with separator being optionally overwriten by this option.
        #[arg(short, long, default_value_t = util::AsciiChar7Bit::COMMA)]
        pub separator: util::AsciiChar7Bit,

        /// Measures this classifier's accuracy using testing irises data.
        #[arg(short = 'a', long, default_value_t = true)]
        pub run_accuracy_measure: bool,
    }
}
pub mod cfg {
    //! Defines app's configuration.

    use core::panic;
    use std::sync::OnceLock;

    use derive_more::{Constructor, Deref, DerefMut};

    /// The only app configuration object.
    pub static APP_CFG: OnceLock<AppCfg> = OnceLock::new();

    /// Returns the global app configuration.
    ///
    /// # Panics
    /// * If it hasn't been initialized.
    pub fn app_cfg() -> &'static AppCfg {
        if let Some(app_cfg) = APP_CFG.get() {
            return app_cfg;
        } else {
            panic!("Logical error: app config used before being initialized.")
        }
    }

    /// App configuration.
    #[derive(Constructor, Debug, Deref, DerefMut)]
    pub struct AppCfg {
        app_args: crate::AppArgs,
    }
}
/// Measures accuracy of provided iris classifier.
pub fn run_accuracy_measure<F>(iris_classifier: F) -> Result<(), anyhow::Error>
where
    F: (Fn(UnclassifiedIris) -> ClassifiedIris) + Send + Sync,
{
    use crate::PATH_TO_TESTING_IRISES;

    let testing_irises = crate::read::testing_irises()?;

    let Some(all_irises_count) = std::num::NonZeroUsize::new(testing_irises.len()) else {
        eprintln!("Classification accuracy for \"{PATH_TO_TESTING_IRISES}\" couldn't be measured, due to file not containing any iris case.");
        // It is not considered error, just a lack of measurement.
        return Ok(());
    };
    let unclassified_t_irises: Vec<UnclassifiedIris> = testing_irises
        .iter()
        .map(|classified| classified.parameters)
        .collect();
    let reclassified_t_irises = crate::classify_irises(iris_classifier, unclassified_t_irises);
    let good_classifications_count = reclassified_t_irises
        .iter()
        .zip(testing_irises.iter())
        .filter(|&(ri, ti)| ri.classification == ti.classification)
        .count();
    eprintln!(
        "Classification accuracy for \"{PATH_TO_TESTING_IRISES}\" is {} .",
        Ratio::new(good_classifications_count, all_irises_count.get())
    );
    Ok(())
}
