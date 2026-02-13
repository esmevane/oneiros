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
}
