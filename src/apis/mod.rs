use std::cmp::Ordering;

use lazy_static::lazy_static;
use regex::Regex;

use crate::{error::Error, ReleaseAsset};

pub mod github;

lazy_static! {
    static ref SIMPLE_VERSION_REGEX: Regex = Regex::new(r"(\d+)\.(\d+)\.(\d+)").unwrap();
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct SimpleTag {
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
}

impl SimpleTag {
    pub fn from_str(data: &str) -> Option<SimpleTag> {
        let version = SIMPLE_VERSION_REGEX.captures(data)?;
        let major = version.get(1)?.as_str().parse::<i32>().ok()?;
        let minor = version.get(2)?.as_str().parse::<i32>().ok()?;
        let patch = version.get(3)?.as_str().parse::<i32>().ok()?;

        Some(SimpleTag {
            major,
            minor,
            patch,
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
