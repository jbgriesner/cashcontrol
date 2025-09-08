pub struct JsonFileRepository {
    file_path: String,
}

impl JsonFileRepository {
    pub fn new(file_path: impl Into<String>) -> Self {
        Self {
            file_path: file_path.into(),
        }
    }
}
