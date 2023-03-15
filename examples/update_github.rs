use autoupdater::{
    apis::{github::GithubApi, DownloadApiTrait},
    cargo_crate_version,
    error::Error,
    Sort,
};

fn main() -> Result<(), Error> {
    let api = GithubApi::new("localcc", "somerepo").current_version(cargo_crate_version!());

    let download = api.get_newer(None::<Sort>)?;
    println!("{:?}", download);

    if let Some(download) = download {
        api.download(
            &download.assets[0],
            Some(|progress| println!("Download progress {progress}")),
        )?;
    }

    Ok(())
}
