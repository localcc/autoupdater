use std::{
    env,
    fs::{self, File},
    io::{BufRead, BufReader, Write},
};

use error::Error;

pub mod apis;
pub mod error;
mod macros;

pub trait ReleaseAsset {
    /// Gets asset filename
    fn get_name(&self) -> &str;

    /// Gets asset download url
    fn get_download_url(&self) -> &str;

    /// Downloads asset.
    /// This function can be used directly or with the api through which the asset was fetched.
    /// You may want to use it with the api for automatically passing the token
    ///
    /// download_callback parameter value is 0..1 float value indicating the download progress.
    ///
    /// * Errors:
    ///    * `ureq` errors
    ///    * `std::io::Error` io errors when writing/replacing asset files
    ///
    fn download(
        &self,
        additional_headers: Vec<(&str, &str)>,
        download_callback: Option<impl Fn(f32)>,
    ) -> Result<(), Error>;
}

pub(crate) fn download<Asset: ReleaseAsset>(
    asset: &Asset,
    additional_headers: Vec<(&str, &str)>,
    download_callback: Option<impl Fn(f32)>,
) -> Result<(), Error> {
    let mut request = ureq::get(asset.get_download_url())
        .set("user-agent", "rust-ureq/updater")
        .set("accept", "application/octet-stream");
    for (header, value) in additional_headers {
        request = request.set(header, value);
    }
    let response = request.call()?;

    if response.status_text() != "OK" {
        return Err(Error::http(response.status_text()));
    }

    set_ssl_vars!();

    let tmp_dir = tempfile::Builder::new()
        .prefix(&format!("{}_dl", asset.get_name()))
        .tempdir()?;

    let tmp_file = tmp_dir.path().join(asset.get_name());
    let mut updated_file = File::create(&tmp_file)?;

    let total_size: u64 = response
        .header("content-length")
        .and_then(|len| len.parse().ok())
        .unwrap_or(0);

    let mut src = BufReader::new(response.into_reader());

    let mut downloaded = 0;
    loop {
        let n = {
            let buf = src.fill_buf()?;
            updated_file.write_all(buf)?;
            buf.len()
        };

        if n == 0 {
            break;
        }

        src.consume(n);

        downloaded = u64::min(total_size, downloaded + n as u64);

        if let Some(ref download_callback) = download_callback {
            download_callback(f32::min(downloaded as f32 / total_size as f32, 1.0));
        }
    }

    if let Some(ref download_callback) = download_callback {
        download_callback(1.0);
    }

    // todo: archive support

    #[cfg(not(windows))]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = fs::metadata(&tmp_file)?.permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(&tmp_file, permissions)?;
    }

    let current_executable = env::current_exe()?;
    let old_executable = current_executable.with_extension("exe.old");
    let updated = current_executable.with_extension("updated");

    fs::remove_file(&old_executable).ok();
    fs::rename(&current_executable, &old_executable)?;
    fs::copy(&tmp_file, &updated)?;
    fs::rename(&updated, &current_executable)?;

    Ok(())
}
