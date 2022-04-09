use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize, Clone)]
pub struct Bang {
    #[serde(rename = "c", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "d")]
    pub domain: String,
    #[serde(rename = "r")]
    pub rate: usize,
    #[serde(rename = "s")]
    pub title: String,
    #[serde(rename = "sc", skip_serializing_if = "Option::is_none")]
    pub search_category: Option<String>,
    #[serde(rename = "t")]
    pub tag: String,
    #[serde(rename = "u")]
    pub url: String,
}

pub fn merge_bangs(bangs: &mut Vec<Bang>, override_bangs: Vec<Bang>) {
    for override_bang in override_bangs {
        if let Some(bang) = bangs.iter_mut().find(|bang| bang.tag == override_bang.tag) {
            *bang = override_bang
        } else {
            bangs.push(override_bang)
        }
    }
}

fn registry_path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .join(format!("{name}_registry.json"))
}

pub fn read_registry(name: &str) -> Vec<Bang> {
    serde_json::from_reader(
        File::open(registry_path(name))
            .unwrap_or_else(|e| panic!("Could not  open {}: {}", registry_path(name).display(), e)),
    )
    .unwrap_or_else(|e| {
        panic!(
            "Could not deserialize {}: {}",
            registry_path(name).display(),
            e
        )
    })
}

pub fn write_registry(mut bangs: Vec<Bang>, name: &str) {
    bangs.sort_by_key(|b| b.tag.clone());
    serde_json::to_writer_pretty(
        File::create(registry_path(name)).unwrap_or_else(|e| {
            panic!("Could not create {}: {}", registry_path(name).display(), e)
        }),
        &bangs,
    )
    .unwrap_or_else(|e| {
        panic!(
            "Could not serialize {}: {}",
            registry_path(name).display(),
            e
        )
    })
}
