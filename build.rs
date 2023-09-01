use std::{env, path::PathBuf};
use unicode_names2_generator as generator;

const UNICODE_DATA: &str = include_str!("data/UnicodeData.txt");

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=data/");
    println!("cargo:rerun-if-changed=generator/");
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    {
        let mut generated_path = out_dir.clone();
        generated_path.push("generated.rs");
        generator::generate(UNICODE_DATA, Some(&generated_path), None);
    }
    {
        let mut generated_phf_path = out_dir;
        generated_phf_path.push("generated_phf.rs");
        generator::generate_phf(UNICODE_DATA, Some(&generated_phf_path), None, 3, 2);
    }
}
