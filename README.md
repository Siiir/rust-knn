Basic installation & running:
1. `git clone https://github.com/Siiir/iris_classifier`
2. `cd iris_classifier`
3. `cat ./data/unclassified_irises.csv | cargo run --release`

Above approach requires:
1. `git`
2. `cargo` that is usually installed with [rustup](https://www.rust-lang.org/tools/install)

You can also pass arguments to the app after --, which is cargo's way to separate cargo args from app args.  
Try: `cargo r -r -- --help`
