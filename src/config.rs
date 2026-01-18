use std::{
    fs, io,
    path::{Path, PathBuf},
};

pub fn bin_name() -> String {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;

    let file_name = match os {
        "windows" => format!("frpc_{}_{}.exe", os, arch),
        _ => format!("frpc_{}_{}", os, arch),
    };

    file_name
}

#[derive(Debug)]
pub struct AppHome {
    dir: PathBuf,
}

impl AppHome {
    pub fn new(name: impl AsRef<str>) -> io::Result<Self> {
        let home = dirs_next::home_dir().ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, "cannot determine home directory")
        })?;

        let dir = home.join(format!(".{}", name.as_ref()));
        Ok(Self { dir })
    }

    /// Ensure ~/.<app> exists
    pub fn ensure(&self) -> io::Result<()> {
        fs::create_dir_all(&self.dir)
    }

    pub fn dir(&self) -> &Path {
        &self.dir
    }

    /// Pure join (NO side effects)
    pub fn join<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        self.dir.join(path)
    }

    /// Join and ensure the directory exists
    pub fn join_dir<P: AsRef<Path>>(&self, path: P) -> io::Result<PathBuf> {
        let dir = self.dir.join(path);
        fs::create_dir_all(&dir)?;
        Ok(dir)
    }
}
