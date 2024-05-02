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
           s.0.contains(".dll") ||
           s.0.contains(".clap") {

            let plugin = s.0;
            let matchs;
            let ext;
            if plugin.contains(".vst3") {
                matchs = plugin.rmatch_indices("vst3").collect::<Vec<_>>()[0];
                ext = ".vst3";
            } else if plugin.contains(".dll") {
                matchs = plugin.rmatch_indices("dll").collect::<Vec<_>>()[0];
                ext = ".dll";
            } else {
                matchs = plugin.rmatch_indices("clap").collect::<Vec<_>>()[0];
                ext = ".clap";
            }

            // slice at index, then tack on the end
            let base = &plugin[..matchs.0-1];
            let plugin = format!("{base}{ext}");

            println!("{plugin}");
        }
    }
}
