use banoogle_bang::BangUrl;
use banoogle_registry::{merge_bangs, read_registry};
use std::path::Path;

#[tokio::main]
async fn main() {
    let mut bangs = read_registry("original");
    merge_bangs(&mut bangs, read_registry("automated_override"));
    merge_bangs(&mut bangs, read_registry("manual_override"));

    let entries = bangs
        .iter()
        .map(|b| {
            (
                b.tag.as_str(),
                BangUrl::parse(&b.url)
                    .unwrap_or_else(|e| panic!("Could not parse bang url {}: {}", b.url, e)),
            )
        })
        .collect::<Vec<_>>();

    let db_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("db");
    let db = sled::open(&db_path).unwrap_or_else(|e| {
        panic!(
            "should be able to create db at {}: {}",
            db_path.display(),
            e
        )
    });

    db.clear().expect("Couldn't clear database");
    for (tag, url) in entries {
        db.insert(tag, url.clone())
            .unwrap_or_else(|e| panic!("Could not insert entry for ({}, {}): {}", tag, url, e));
    }
    db.flush_async().await.expect("Could not flush database");
}
