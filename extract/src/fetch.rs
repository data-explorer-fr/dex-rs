//! Fetching files from remote localtions.
//!
//! This crate provides a `fetch` function to retrieve files from remote locations,
//! returing a stream of bytes.
//!

pub mod fetch {

    use bytes::Bytes;
    use futures_core::stream::Stream;
    use std::error::{Error};

    pub struct BasicCredentials {
        pub username: Option<String>,
        pub password: Option<String>,
    }

    pub struct TokenCredentials {
        pub token: Option<String>,
        pub header_format: Option<String>,
    }

    pub struct SshCredentials {
        pub username: Option<String>,
        pub key: Option<String>,
    }

    pub enum HttpCredentials {
        BasicCredentials,
        TokenCredentials,
    }

    pub enum Credentials {
        HttpCredentials,
        SshCredentials,
    }

    pub struct FetchConfig {
        pub url: String,
        pub credentials: Option<Credentials>,
    }

    // 
    pub fn fetch(config: FetchConfig) -> impl Stream<Item = Result<Bytes, Box<dyn Error>>> {
        tokio::stream::empty()
    }
}