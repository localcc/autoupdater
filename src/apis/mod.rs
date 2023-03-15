use std::cmp::Ordering;

use crate::{error::Error, ReleaseAsset};

pub mod github;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct SimpleTag {
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
}

impl SimpleTag {
    pub fn from_str(data: &str) -> Option<SimpleTag> {
        let mut ver = data
            .trim_start_matches(char::is_alphabetic)
            .trim_end_matches(char::is_alphabetic)
            .split('.')
            .map(str::parse)
            .filter_map(Result::ok);

        Some(SimpleTag {
            major: ver.next()?,
            minor: ver.next()?,
            patch: ver.next()?,
        })
    }

    pub fn simple_compare(a: &str, b: &str) -> Ordering {
        let a_version = SimpleTag::from_str(a);
        let b_version = SimpleTag::from_str(b);

        match (a_version, b_version) {
            (Some(a_version), Some(b_version)) => a_version.cmp(&b_version),
            _ => Ordering::Equal,
        }
    }
}

pub trait DownloadApiTrait {
    /// Download any fetched asset using this api
    ///
    /// download_callback parameter value is 0..1 float value indicating the download progress.
    ///
    /// * Errors:
    ///    * `reqwest` errors
    ///    * `std::io::Error` io errors when writing/replacing asset files
    ///
    fn download<Asset: ReleaseAsset>(
        &self,
        asset: &Asset,
        download_callback: Option<Box<dyn Fn(f32)>>,
    ) -> Result<(), Error>;
}
