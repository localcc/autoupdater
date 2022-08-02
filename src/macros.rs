/// Copied from self_update crate src/macros.rs

/// Allows you to pull the version from your Cargo.toml at compile time as
/// `MAJOR.MINOR.PATCH_PKGVERSION_PRE`
#[macro_export]
macro_rules! cargo_crate_version {
    // -- Pulled from clap.rs src/macros.rs
    () => {
        env!("CARGO_PKG_VERSION")
    };
}

/// Set ssl cert env. vars to make sure openssl can find required files
#[macro_export]
macro_rules! set_ssl_vars {
    () => {
        #[cfg(target_os = "linux")]
        {
            if ::std::env::var_os("SSL_CERT_FILE").is_none() {
                ::std::env::set_var("SSL_CERT_FILE", "/etc/ssl/certs/ca-certificates.crt");
            }
            if ::std::env::var_os("SSL_CERT_DIR").is_none() {
                ::std::env::set_var("SSL_CERT_DIR", "/etc/ssl/certs");
            }
        }
    };
}
