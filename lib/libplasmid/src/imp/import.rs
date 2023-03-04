pub trait Import {
    type Output;

    fn import<S>(s: S) -> anyhow::Result<Self::Output>
    where
        S: AsRef<str>;

    fn import_from_file(file: std::fs::File) -> anyhow::Result<Self::Output> {
        use std::io::Read;
        let mut reader = std::io::BufReader::new(file);
        let mut contents = String::new();
        reader.read_to_string(&mut contents)?;
        Self::import(contents)
    }

    fn import_from_path<P>(path: P) -> anyhow::Result<Self::Output>
    where
        P: AsRef<std::path::Path>,
    {
        let file = std::fs::File::open(path)?;
        Self::import_from_file(file)
    }
}
