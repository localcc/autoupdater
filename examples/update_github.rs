#[macro_use]
extern crate autoupdater;

use autoupdater::{
    apis::{github::GithubApi, DownloadApiTrait},
    error::Error,
};

fn main() -> Result<(), Error> {
    let api = GithubApi::new("localcc", "somerepo").current_version(cargo_crate_version!());

    let download = api.get_newer(&None)?;
    println!("{:?}", download);

    if let Some(download) = download {
        api.download(
            &download.assets[0],
            Some(|progress| println!("Download progress {progress}")),
        )?;
    }

    Ok(())
}
