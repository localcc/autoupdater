#[macro_use]
extern crate autoupdater;

use std::error::Error;

use autoupdater::apis::DownloadApiTrait;

fn main() -> Result<(), Box<dyn Error>> {
    let api = autoupdater::apis::github::GithubApi::new("localcc", "somerepo")
        .current_version(cargo_crate_version!());

    let download = api.get_newer(&None)?;
    println!("{:?}", download);

    if let Some(download) = download {
        api.download(
            &download.assets[0],
            Some(Box::new(|progress| {
                println!("Download progress {}", progress);
            })),
        )?;
    }

    Ok(())
}
