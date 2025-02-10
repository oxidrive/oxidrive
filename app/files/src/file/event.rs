use super::File;

#[derive(Debug, Clone)]
pub enum FileEvent {
    Changed(File),
}
