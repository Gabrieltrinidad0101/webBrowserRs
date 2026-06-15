use std::sync::{LazyLock, RwLock};

pub static CODE: LazyLock<RwLock<Vec<u8>>> = LazyLock::new(|| RwLock::new(Vec::new()));
pub static HTML_CODE: LazyLock<RwLock<Vec<u8>>> = LazyLock::new(|| RwLock::new(Vec::new()));
