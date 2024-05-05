use std::{env, path::Path};

use rust_strings::{strings, FileConfig};

fn main() {
    let mut args = env::args().skip(1);

    let Some(file) = args.next() else {
        println!("Usage: flp-plugins <flp_file>.flp");
        return;
    };

    let file = FileConfig::new(Path::new(&file));

    let all_strings = strings(&file).unwrap();

    for (plugin, _) in all_strings {
        let mut type_ = None;

        if plugin.contains(".vst3") {
            type_ = Some(("vst3", ".vst3"));
        } else if plugin.contains(".dll") {
            type_ = Some(("dll", ".dll"));
        } else if plugin.contains(".clap") {
            type_ = Some(("clap", ".clap"));
        }

        if let Some((type_, ext)) = type_ {
            let mat = plugin.rmatch_indices(type_).collect::<Vec<_>>();
            let Some((idx, _)) = mat.first() else {
                continue;
            };

            let Some(idx) = idx.checked_sub(1) else {
                continue;
            };

            // slice at index, then tack on the end
            let base = &plugin[..idx];

            println!("{base}{ext}");
        }
    }
}
