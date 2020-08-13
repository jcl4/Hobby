use std::path::Path;

fn main() {
    let name = env!("CARGO_PKG_NAME");
    let full_name = format!("{}.log", name);

    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let example_dir = root_dir.join("examples").join(name);
    let log_file = example_dir.join(full_name);

    hobby::setup_logging(&log_file);

}
