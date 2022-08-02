# autoupdater

This crate was written to allow for easy rust application auto-updating.

## Usage

To use this crate add it as a dependency to your `Cargo.toml`

```
autoupdater = "0.1.0"
```

## Examples

To fetch and download an update you may do something like this

```rs
    let mut api = autoupdater::apis::github::GithubApi::new("localcc", "somerepo");
    api.current_version(cargo_crate_version!());

    let download = api.get_newer(&None)?;
    println!("{:?}", download);

    if let Some(download) = download {
        api.download(
            &download.assets[0],
            None
        )?;
    }
```

For more examples look in the [examples](https://github.com/localcc/autoupdater/examples) directory.

## Features

`rustls-tls`: Enables native rust TLS implementation for requests.
