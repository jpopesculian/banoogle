use crate::query::{percent_decode, query_pairs};
use axum::response::Redirect;
use axum::{extract::RawQuery, Extension};
use banoogle_bang::{default_with_query, BangUrl, DEFAULT_URL};

fn is_query_whitespace(ch: u8) -> bool {
    ch == b'+' || ch.is_ascii_whitespace()
}

fn get_query_q(query: &str) -> Option<&[u8]> {
    query_pairs(query.as_bytes()).find_map(|(key, val)| {
        let mut chars = percent_decode(key);
        if chars.next() == Some(b'q') && chars.next().is_none() {
            Some(val)
        } else {
            None
        }
    })
}

fn get_bang(query: &[u8]) -> Option<(String, &[u8])> {
    let mut chars = percent_decode(query);
    chars.consume_while(is_query_whitespace);
    if chars.next() != Some(b'!') {
        return None;
    }
    let mut tag_bytes = Vec::new();
    #[allow(clippy::while_let_on_iterator)]
    while let Some(next) = chars.next() {
        if !is_query_whitespace(next) {
            tag_bytes.push(next);
        } else {
            break;
        }
    }
    if let Ok(tag) = String::from_utf8(tag_bytes) {
        chars.consume_while(is_query_whitespace);
        Some((tag, chars.bytes.as_slice()))
    } else {
        None
    }
}

fn redirect_to_default(query: &[u8]) -> Redirect {
    Redirect::to(default_with_query(query).as_deref().unwrap_or(DEFAULT_URL))
}

fn redirect_to_bang(bang: BangUrl, query: &[u8]) -> Redirect {
    Redirect::to(bang.with_query(query).as_deref().unwrap_or(DEFAULT_URL))
}

pub async fn handler(query: RawQuery, db: Extension<sled::Db>) -> Redirect {
    if let Some(query) = query.0.as_ref().and_then(|query| get_query_q(query)) {
        if let Some((bang, bang_query)) = get_bang(query) {
            match db.get(bang) {
                Ok(Some(encoded_url)) => redirect_to_bang(BangUrl::from(&encoded_url), bang_query),
                Ok(None) => redirect_to_default(query),
                Err(err) => {
                    tracing::error!("Error occured querying db: {err}");
                    redirect_to_default(query)
                }
            }
        } else {
            redirect_to_default(query)
        }
    } else {
        Redirect::to(DEFAULT_URL)
    }
}
