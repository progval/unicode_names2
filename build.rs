use std::{env, path::PathBuf};
use unicode_names2_generator as generator;

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    {
        let mut generated_path = out_dir.clone();
        generated_path.push("generated.rs");
        generator::generate(Some(&generated_path), None);
    }
    {
        let mut generated_phf_path = out_dir.clone();
        generated_phf_path.push("generated_phf.rs");
        generator::generate_phf(Some(&generated_phf_path), None, 3, 2);
    }
}
