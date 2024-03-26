pub use iris::{
    classified::ClassifiedIris, params::IrisParams, species::IrisSpecies,
    unclassified::UnclassifiedIris, Iris,
};
pub mod iris;

pub use paths::*;
pub mod paths {
    //! Defines paths used by this app.

    pub const PATH_TO_TRAINING_IRISES: &str = "./data/training_irises.csv";
    pub const PATH_TO_TESTING_IRISES: &str = "./data/testing_irises.csv";
}

pub mod read;
pub mod util;

pub use app::{
    args::AppArgs,
    cfg::{app_cfg, AppCfg, APP_CFG},
};
pub mod app;

use itertools::Itertools;

#[macro_export]
macro_rules! executable_desc{
    () => {
        "App for classifying irises into one of 3 species {Setosa, Versicolor, Virginica}.\n\
        \n\
        After running this app, it will:\n\
        1. Perform an accuracy measure for the testing iris data file dwelling in the ./data folder. \
            This can be disabled by app argument. \
            Measure will be rendered to stderr.\n\
        2. Read all irises from stdin untill EOF. Then classify them. \
            This will be displayed in stdout.\
        \n\
        EXAMPLE usages for app is installed as `iris_classifier`:\n\
        * `echo '1.,2,3,4' | iris_classifier`\n\
        * `cat ./data/unclassified_irises.csv | iris_classifier`\n\
        * `echo 'some.invalid.data' | iris_classifier --separator '.'`\n\
        * `echo '1 2 3 4' | iris_classifier -s=' '`\n\
        "
    }
}

/// Creates classifier that will map any unclassified iris into classified.
///
/// Created classifier will guess classification for iris using KNN algorithm.
pub fn create_classifier(
    classified_irises: Vec<ClassifiedIris>,
    k: usize,
) -> impl Fn(UnclassifiedIris) -> ClassifiedIris {
    move |unclassified| {
        let nearest_classifications = classified_irises
            .iter()
            .map(|classified| {
                util::ComparedByDistSq::new(classified, classified.dist_sq(&unclassified))
            })
            .k_smallest(k)
            .map(|cmpd_by_dist_sq| cmpd_by_dist_sq.val.classification);
        let mode = util::mode(nearest_classifications).unwrap();

        ClassifiedIris::new(unclassified, mode)
    }
}

/// Classifies unclassified irises using classifier.
pub fn classify_irises<F>(
    iris_classifier: F,
    unclassified_irises: Vec<UnclassifiedIris>,
) -> Vec<ClassifiedIris>
where
    F: (Fn(UnclassifiedIris) -> ClassifiedIris) + Send + Sync,
{
    use rayon::iter::{IntoParallelIterator, ParallelIterator};

    unclassified_irises
        .into_par_iter()
        .map(iris_classifier)
        .collect()
}
