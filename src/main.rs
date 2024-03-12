use ic::{
    app_cfg, read, AppCfg, ClassifiedIris, UnclassifiedIris, APP_CFG, PATH_TO_TESTING_IRISES,
};
use num_rational::Ratio;

fn main() -> anyhow::Result<()> {
    let app_args: ic::AppArgs = clap::Parser::parse();
    APP_CFG
        .set(AppCfg::new(app_args))
        .expect("This should be the only app config initialization.");

    // Reading iris data.
    let training_irises = read::training_irises()?;
    // Creating classifier using the classified data.
    let iris_classifier = ic::create_classifier(training_irises, app_cfg().k_for_knn);
    if app_cfg().run_accuracy_measure {
        let testing_irises = read::testing_irises()?;
        let unclassified_t_irises: Vec<UnclassifiedIris> = testing_irises
            .iter()
            .map(|classified| classified.parameters)
            .collect();
        let reclassified_t_irises = ic::classify_irises(&iris_classifier, unclassified_t_irises);

        let good_classifications_count = reclassified_t_irises
            .iter()
            .zip(testing_irises.iter())
            .filter(|&(ri, ti)| ri.classification == ti.classification)
            .count();
        eprintln!(
            "Classification accuracy for \"{PATH_TO_TESTING_IRISES}\": {}",
            Ratio::new_raw(good_classifications_count, testing_irises.len())
        );
    }
    let user_irises = read::user_irises()?;

    // Classifying all unclassified irises using classifier.
    let now_classified_irises: Vec<ClassifiedIris> =
        ic::classify_irises(&iris_classifier, user_irises);
    // Displaying the results.
    let table_with_classified = tabled::Table::new(now_classified_irises);
    print!("{}", table_with_classified);

    Ok(())
}
