pub use iris::{
    classification::IrisSpecies, classified::ClassifiedIris, unclassified::UnclassifiedIris, Iris,
};
pub mod iris;

pub use paths::*;
pub mod paths {
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

pub fn classify_irises(
    iris_classifier: impl (Fn(UnclassifiedIris) -> ClassifiedIris) + Sync + Send,
    unclassified_irises: Vec<UnclassifiedIris>,
) -> Vec<ClassifiedIris> {
    use rayon::iter::{IntoParallelIterator, ParallelIterator};

    unclassified_irises
        .into_par_iter()
        .map(iris_classifier)
        .collect()
}
