use std::{cmp::Ordering, fmt::Display};

use reqwest::header::{self, HeaderMap};
use serde::Deserialize;

use crate::{error::Error, ReleaseAsset};

use super::{DownloadApiTrait, SimpleTag};

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Clone)]
pub struct GithubAsset {
    pub name: String,
    pub url: String,
}

impl Display for GithubAsset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Name: {}", self.name)?;
        writeln!(f, "Name: {}", self.url)
    }
}

impl ReleaseAsset for GithubAsset {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_download_url(&self) -> &str {
        &self.url
    }

    fn download(
        &self,
        additional_headers: HeaderMap,
        download_callback: Option<Box<dyn Fn(f32)>>,
    ) -> Result<(), Error> {
        crate::download(self, additional_headers, download_callback)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Clone)]
pub struct GithubRelease {
    pub tag_name: String,
    pub target_commitish: String,
    pub name: String,
    pub prerelease: bool,
    pub assets: Vec<GithubAsset>,
    pub body: String,
}

impl Display for GithubRelease {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Tag: {}", self.tag_name)?;
        writeln!(f, "Branch: {}", self.target_commitish)?;
        writeln!(f, "Name: {}", self.name)?;
        writeln!(f, "Prerelease: {}", self.prerelease)?;
        writeln!(f, "Assets:")?;
        for asset in &self.assets {
            writeln!(f, "{}", asset)?;
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct GithubApi {
    api_url: Option<String>,
    owner: String,
    repo: String,
    auth_token: Option<String>,
    branch: Option<String>,
    prerelease: bool,
    specific_tag: Option<String>,
    current_version: Option<String>,
    asset_name: Option<String>,
}

type SortFunc = Box<dyn Fn(&str, &str) -> Ordering>;

impl GithubApi {
    pub fn new(owner: &str, repo: &str) -> Self {
        GithubApi {
            api_url: None,
            owner: owner.to_string(),
            repo: repo.to_string(),
            auth_token: None,
            branch: None,
            prerelease: false,
            specific_tag: None,
            current_version: None,
            asset_name: None,
        }
    }

    /// Sets custom github api url
    pub fn api_url(mut self, api_url: &str) -> Self {
        self.api_url = Some(api_url.to_string());
        self
    }

    /// Sets auth token.
    pub fn auth_token(mut self, auth_token: &str) -> Self {
        self.auth_token = Some(auth_token.to_string());
        self
    }

    /// Sets branch from which to get releases.
    pub fn branch(mut self, branch: &str) -> Self {
        self.branch = Some(branch.to_string());
        self
    }

    /// Sets if prerelease should be included in the list of releases.
    pub fn prerelease(mut self, prerelease: bool) -> Self {
        self.prerelease = prerelease;
        self
    }

    /// Sets specific version tag to get.
    pub fn specific_tag(mut self, specific_tag: &str) -> Self {
        self.specific_tag = Some(specific_tag.to_string());
        self
    }

    /// Sets current version of the application, this is used to determine if the latest release is newer than the current version.
    pub fn current_version(mut self, current_version: &str) -> Self {
        self.current_version = Some(current_version.to_string());
        self
    }

    /// Sets asset name to download.
    pub fn asset_name(mut self, asset_name: &str) -> Self {
        self.asset_name = Some(asset_name.to_string());
        self
    }

    fn get_releases(&self, per_page: i32, page: i32) -> Result<Vec<GithubRelease>, Error> {
        let api_url = format!(
            "https://{}/repos/{}/{}/releases?per_page={}&page={}",
            self.api_url.as_deref().unwrap_or("api.github.com"),
            self.owner,
            self.repo,
            per_page,
            page
        );

        let mut headers = HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            "rust-reqwest/updater".parse().expect("Invalid user agent"),
        );

        if let Some(token) = &self.auth_token {
            headers.insert(
                header::AUTHORIZATION,
                format!("token {}", token)
                    .parse()
                    .expect("Invalid authorization"),
            );
        }

        let response = reqwest::blocking::Client::new()
            .get(api_url)
            .headers(headers)
            .send()?;

        let release_list: Vec<GithubRelease> = response.json()?;
        Ok(release_list)
    }

    fn filter_release(&self, e: &GithubRelease) -> bool {
        if !self.prerelease && e.prerelease {
            return false;
        }
        let specific_tag = match self.specific_tag {
            Some(ref tag) => *tag == e.tag_name,
            None => true,
        };

        let branch = match self.branch {
            Some(ref branch) => *branch == e.target_commitish,
            None => true,
        };

        let asset_name = match self.asset_name {
            Some(ref asset_name) => e.assets.iter().any(|e| e.name == *asset_name),
            None => true,
        };

        specific_tag && branch && asset_name
    }

    fn match_releases<'releases>(
        &self,
        releases: &'releases [GithubRelease],
    ) -> Vec<&'releases GithubRelease> {
        releases.iter().filter(|e| self.filter_release(e)).collect()
    }

    /// Gets the latest release
    pub fn send<Sort: Fn(&str, &str) -> Ordering>(
        &self,
        sort_func: &Option<Sort>,
    ) -> Result<GithubRelease, Error> {
        let mut releases = self.get_releases(100, 1)?;

        let mut page = 3;
        let mut new_releases = self.get_releases(100, 2)?;
        while !new_releases.is_empty() {
            releases.extend(new_releases);
            new_releases = self.get_releases(100, page)?;
            page += 1;
        }

        let mut matching = self.match_releases(&releases);
        if matching.is_empty() {
            return Err(Error::no_release());
        }

        match sort_func {
            Some(sort_func) => {
                matching.sort_by(|a, b| sort_func(&a.tag_name, &b.tag_name));
            }
            None => matching.sort_by(|a, b| SimpleTag::simple_compare(&a.tag_name, &b.tag_name)),
        };

        let latest_release = matching.last().ok_or_else(Error::no_release)?;
        Ok((*latest_release).clone())
    }

    /// Gets the latest release
    pub fn get_latest<Sort: Fn(&str, &str) -> Ordering>(
        &self,
        sort_func: &Option<Sort>,
    ) -> Result<GithubRelease, Error> {
        self.send(sort_func)
    }

    /// Gets all releases
    /// NOTE: this is kinda slow so use it only if you need it
    pub fn get_all(&self) -> Result<Vec<GithubRelease>, Error> {
        let mut releases = Vec::new();
        let mut page = 1;
        loop {
            let fetched_releases = self.get_releases(100, page)?;
            if fetched_releases.is_empty() {
                break;
            }

            releases.extend(fetched_releases);
            page += 1;
        }

        Ok(releases
            .into_iter()
            .filter(|e| self.filter_release(e))
            .collect::<Vec<_>>())
    }

    /// Gets the newest release if the newest release is newer than the current one.
    ///
    /// sort_func is used to compare two release versions if the format is not x.y.z
    pub fn get_newer(&self, sort_func: &Option<SortFunc>) -> Result<Option<GithubRelease>, Error> {
        let latest_release = self.send(sort_func)?;
        let is_newer = match self.current_version {
            Some(ref current_version) => {
                let newer = match sort_func {
                    Some(sort_func) => sort_func(&latest_release.tag_name, current_version),
                    None => SimpleTag::simple_compare(&latest_release.tag_name, current_version),
                };

                newer == Ordering::Greater
            }
            None => true,
        };

        if is_newer {
            Ok(Some(latest_release))
        } else {
            Ok(None)
        }
    }
}

impl DownloadApiTrait for GithubApi {
    fn download<Asset: ReleaseAsset>(
        &self,
        asset: &Asset,
        download_callback: Option<Box<dyn Fn(f32)>>,
    ) -> Result<(), Error> {
        let mut headers = HeaderMap::new();

        if let Some(token) = &self.auth_token {
            headers.insert(
                header::AUTHORIZATION,
                format!("token {}", token)
                    .parse()
                    .expect("Invalid authorization"),
            );
        }

        asset.download(headers, download_callback)
    }
}
