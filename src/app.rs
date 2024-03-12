pub mod args {
    //! Facilitates usage of this app's arguments.

    use crate::util;

    #[derive(clap::Parser, Debug)]
    pub struct AppArgs {
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
    use core::panic;
    use std::sync::OnceLock;

    use derive_more::{Constructor, Deref, DerefMut};

    pub static APP_CFG: OnceLock<AppCfg> = OnceLock::new();
    pub fn app_cfg() -> &'static AppCfg {
        if let Some(app_cfg) = APP_CFG.get() {
            return app_cfg;
        } else {
            panic!("Logical error: app config used before being initialized.")
        }
    }

    #[derive(Constructor, Debug, Deref, DerefMut)]
    pub struct AppCfg {
        app_args: crate::AppArgs,
    }
}
