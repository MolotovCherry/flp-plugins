use std::env;
use rust_strings::{FileConfig, BytesConfig, strings, dump_strings, Encoding};

fn main() {
    let mut args = env::args();
    if args.len() <= 1 {
        println!("Usage: flp-plugins <flp_file>.flp");
        return;
    }

    let file = args.nth(1).unwrap();
    let file = FileConfig::new(&file);

    let all_strings = strings(&file).unwrap();

    for s in all_strings {
        if s.0.contains(".vst3") ||
           s.0.contains(".dll") {
            println!("{}", s.0);
        }
    }
}
