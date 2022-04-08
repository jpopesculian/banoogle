pub struct BangUrl(pub &'static [&'static str]);

include!(concat!(env!("OUT_DIR"), "/registry.rs"));
