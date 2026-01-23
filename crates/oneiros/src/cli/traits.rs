pub(crate) trait Cli {
    fn project_name(&self) -> &str;
    fn project_dir(&self) -> std::path::PathBuf;

    fn context(&self) -> Option<crate::context::Context>
    where
        Self: Clone,
    {
        crate::context::Context::new(self.clone())
    }
}
