use banoogle_registry::{write_registry, Bang};
use url::Url;

pub fn preprocess(bang: &mut Bang) -> bool {
    replace_duckduckgo_with_google(bang)
}

fn replace_duckduckgo_with_google(bang: &mut Bang) -> bool {
    if bang.domain == "duckduckgo.com" && !bang.title.starts_with("DuckDuckGo") {
        if let Ok(mut url) = Url::parse(&bang.url) {
            if url.path().trim_start_matches('/').is_empty() {
                url.set_host(Some("www.google.com"))
                    .expect("Could not set host");
                url.set_path("search");
                bang.url = url.to_string();
                bang.domain = "google.com".to_string();
                return true;
            }
        }
    }
    false
}

#[tokio::main]
async fn main() {
    let original_bangs: Vec<Bang> = reqwest::get("https://duckduckgo.com/bang.js")
        .await
        .expect("Could not fetch bang operators")
        .json()
        .await
        .expect("Could not deserialize bang operators");

    let mut automated_override_bangs = Vec::new();
    for mut bang in original_bangs.clone() {
        if preprocess(&mut bang) {
            automated_override_bangs.push(bang)
        }
    }

    write_registry(original_bangs, "original");
    write_registry(automated_override_bangs, "automated_override");
}
