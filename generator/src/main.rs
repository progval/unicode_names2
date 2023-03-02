use std::path::Path;
use std::{char, cmp};

static OUT_FILE: &str = "../src/generated.rs";
static PHF_OUT_FILE: &str = "../src/generated_phf.rs";

fn main() {
    let mut opts = getopts::Options::new();
    opts.optflag("p", "phf", "compute the name -> codepoint PHF");
    opts.optopt("l", "phf-lambda", "the lambda to use for PHF", "N");
    opts.optopt(
        "t",
        "phf-tries",
        "the number of attempts when computing PHF",
        "N",
    );
    opts.optflag("s", "silent", "don't write anything to files");
    opts.optopt("", "truncate", "only handle the first N", "N");
    opts.optflag("h", "help", "print this message");
    let matches = match opts.parse(std::env::args().skip(1)) {
        Ok(m) => m,
        Err(f) => panic!("{}", f.to_string()),
    };
    if matches.opt_present("h") {
        println!(
            "{}",
            opts.usage("generate compressed codepoint <-> name tables")
        );
        return;
    }

    let do_phf = matches.opt_present("phf");
    let path = if matches.opt_present("s") {
        None
    } else if do_phf {
        Some(Path::new(PHF_OUT_FILE))
    } else {
        Some(Path::new(OUT_FILE))
    };

    let truncate = matches
        .opt_str("truncate")
        .map(|s| s.parse().ok().expect("truncate should be an integer"));

    let lambda = matches.opt_str("phf-lambda");
    let tries = matches.opt_str("phf-tries");
    if do_phf {
        let lambda = lambda
            .map(|s| s.parse().ok().expect("invalid -l"))
            .unwrap_or(3);
        let tries = tries
            .map(|s| s.parse().ok().expect("invalid -t"))
            .unwrap_or(2);
        unicode_names2_generator::generate_phf(path, truncate, lambda, tries);
    } else {
        if lambda.is_some() {
            println!("-l/--phf-lambda only applies with --phf")
        }
        if tries.is_some() {
            println!("-t/--phf-tries only applies with --phf")
        }

        unicode_names2_generator::generate(path, truncate);
    }
}
