use std::path::PathBuf;

use crate::error::Error;
use directories::UserDirs;

pub const ZNN_ROOT_DIRECTORY: &str = "znn";
pub const ZNN_ROOT_DIRECTORY_LINUX: &str = "znn";

#[derive(Debug)]
pub struct ZnnPaths {
    pub main: PathBuf,
    pub wallet: PathBuf,
    pub cache: PathBuf,
}

impl ZnnPaths {
    pub fn new(main: PathBuf, wallet: PathBuf, cache: PathBuf) -> Self {
        Self {
            main,
            wallet,
            cache,
        }
    }
}

pub fn znn_default_paths() -> Result<ZnnPaths, Error> {
    let user_dirs: UserDirs = match UserDirs::new() {
        Some(ud) => ud,
        None => {
            return Err(Error::FailedGettingPath(
                "Unable to get home directory.".to_string(),
            ))
        }
    };

    let mut main = user_dirs.home_dir().to_path_buf();

    main = match std::env::consts::OS {
        "macos" => main.join("Library").join(ZNN_ROOT_DIRECTORY),
        "windows" => main.join("AppData").join(ZNN_ROOT_DIRECTORY),
        "linux" => main.join(ZNN_ROOT_DIRECTORY_LINUX),
        _ => main.join(ZNN_ROOT_DIRECTORY),
    };

    let wallet: PathBuf = main.join("wallet");
    let cache: PathBuf = main.join("syrius");

    Ok(ZnnPaths::new(main, wallet, cache))
}

#[cfg(test)]
mod tests {
    use crate::error::Error;
    use crate::global::ZnnPaths;
    // use pretty_assertions::assert_eq;

    use super::znn_default_paths;

    #[test]
    fn test_znn_default_paths() -> Result<(), Error> {
        let p: ZnnPaths = znn_default_paths()?;
        assert!(&p.main.is_absolute());
        assert!(&p.main.is_dir());
        assert!(&p.cache.is_absolute());
        assert!(&p.cache.is_dir());
        assert!(&p.wallet.is_absolute());
        assert!(&p.wallet.is_dir());
        Ok(())
    }
}
