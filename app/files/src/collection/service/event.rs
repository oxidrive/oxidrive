use crate::collection::Collection;

#[derive(Debug, Clone)]
pub enum CollectionEvent {
    Changed(Collection),
}
