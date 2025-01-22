#[derive(Debug, thiserror::Error)]
pub enum MappingTableError {
    #[error("Invalid page id: {0}")]
    NewPageError(usize)
}