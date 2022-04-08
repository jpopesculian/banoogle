use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Bang {
    #[serde(rename = "c", skip_serializing_if = "Option::is_none")]
    category: Option<String>,
    #[serde(rename = "d")]
    domain: String,
    #[serde(rename = "r")]
    rate: usize,
    #[serde(rename = "s")]
    title: String,
    #[serde(rename = "sc", skip_serializing_if = "Option::is_none")]
    search_category: Option<String>,
    #[serde(rename = "t")]
    tag: String,
    #[serde(rename = "u")]
    url: String,
}

fn fetch_registry() {
    let mut bangs: Vec<Bang> = reqwest::blocking::get("https://duckduckgo.com/bang.js")
        .expect("Could not fetch bang operators")
        .json()
        .expect("Could not deserialize bang operators");
    bangs.sort_by_key(|b| b.tag.clone());

    for bang in bangs.iter_mut() {
        if bang.domain == "duckduckgo.com" {
            bang.domain = "google.com".to_string();
            bang.url = bang.url.replacen("duckduckgo.com", "google.com", 1);
        }
    }

    std::fs::write(
        "./assets/registry.json",
        serde_json::to_string(&bangs).expect("invalid output json"),
    )
    .unwrap();
}

fn build_registry() {
    let bangs: Vec<Bang> = serde_json::from_reader(
        std::fs::File::open("./assets/registry.json").expect("could not read registry json"),
    )
    .expect("invalid registry json");

    let mut out = String::new();
    out.push_str(
        r#"
lazy_static::lazy_static! {
    pub static ref BANG_REGISTRY: hashbrown::HashMap<&'static str, crate::bang::BangUrl> = {
        use rayon::prelude::*;
        hashbrown::HashMap::from_par_iter([
"#,
    );

    for bang in bangs {
        let bang_parts = bang.url.split("{{{s}}}").collect::<Vec<_>>();
        out.push_str(&format!(
            r#"({:?}, crate::bang::BangUrl(&{:?})), "#,
            bang.tag, bang_parts
        ));
    }

    out.push_str(
        r#"
        ])
    };
}
    "#,
    );

    let out_path =
        std::path::PathBuf::from(std::env::var("OUT_DIR").expect("could not get OUT DIR"));
    std::fs::write(out_path.join("registry.rs"), out).expect("could not write to registry.rs");
}

fn main() {
    println!("cargo:rerun-if-changed=./assets/registry.json");
    println!("cargo:rerun-if-env-changed=BANOOGLE_FETCH_BANGS");

    if std::env::var_os("BANOOGLE_FETCH_BANGS").is_some() {
        fetch_registry();
    }

    build_registry();
}
