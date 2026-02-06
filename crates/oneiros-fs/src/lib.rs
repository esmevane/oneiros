pub struct FileOps;

impl FileOps {
    pub fn ensure_dir(&self, path: impl AsRef<std::path::Path>) -> Result<(), std::io::Error> {
        std::fs::create_dir_all(path)
    }

    pub fn write(
        &self,
        path: impl AsRef<std::path::Path>,
        contents: impl AsRef<[u8]>,
    ) -> Result<(), std::io::Error> {
        std::fs::write(path, contents)
    }
}
