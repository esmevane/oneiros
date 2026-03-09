pub struct FileOps;

impl FileOps {
    pub fn ensure_dir(&self, path: impl AsRef<std::path::Path>) -> Result<(), std::io::Error> {
        std::fs::create_dir_all(path)
    }

    pub fn read(&self, path: impl AsRef<std::path::Path>) -> Result<Vec<u8>, std::io::Error> {
        std::fs::read(path)
    }

    pub fn read_to_string(
        &self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<String, std::io::Error> {
        std::fs::read_to_string(path)
    }

    pub fn write(
        &self,
        path: impl AsRef<std::path::Path>,
        contents: impl AsRef<[u8]>,
    ) -> Result<(), std::io::Error> {
        std::fs::write(path, contents)
    }

    pub fn append(
        &self,
        path: impl AsRef<std::path::Path>,
        contents: impl AsRef<[u8]>,
    ) -> Result<(), std::io::Error> {
        use std::io::Write;
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        file.write_all(contents.as_ref())
    }

    pub fn contains(
        &self,
        path: impl AsRef<std::path::Path>,
        pattern: &str,
    ) -> Result<bool, std::io::Error> {
        match std::fs::read_to_string(path) {
            Ok(contents) => Ok(contents.contains(pattern)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
            Err(e) => Err(e),
        }
    }
}
