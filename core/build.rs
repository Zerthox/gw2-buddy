#[path = "src/data/skill_def.rs"]
mod skill_def;

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use skill_def::SkillDef;
use std::{
    env,
    fs::{self, File},
    path::PathBuf,
};

fn main() {
    let manifest = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let in_dir = PathBuf::from(manifest).join("src/data/skills");
    let files = fs::read_dir(in_dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            matches!(
                path.extension().and_then(|ext| ext.to_str()),
                Some("json" | "yml" | "yaml")
            )
        });

    let data = files.flat_map(|file| {
        println!("cargo:rerun-if-changes={}", file.display());
        let file = File::open(file).unwrap();
        serde_yaml::from_reader::<_, Vec<SkillDef>>(file).unwrap()
    });

    let contents = data.map(|skill| {
        let SkillDef {
            id,
            enabled,
            hit_ids,
            hits,
            expected,
            max_duration,
            minion,
        } = skill;
        let hits = quote_option(hits);
        let expected = quote_option(expected);
        let max_duration = quote_option(max_duration);
        quote! {
            SkillDef {
                id: #id,
                enabled: #enabled,
                hit_ids: vec![ #(#hit_ids),* ],
                hits: #hits,
                expected: #expected,
                max_duration: #max_duration,
                minion: #minion,
            }
        }
    });

    let result = quote! { [ #(#contents),* ] };

    fs::write(PathBuf::from(out_dir).join("skills.rs"), result.to_string()).unwrap();
}

fn quote_option(option: Option<impl ToTokens>) -> TokenStream {
    match option {
        Some(value) => quote! { Some(#value) },
        None => quote! { None },
    }
}
