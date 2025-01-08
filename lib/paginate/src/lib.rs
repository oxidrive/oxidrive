use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Slice<T> {
    pub items: Vec<T>,
    pub next: Option<String>,
    pub previous: Option<String>,
}

impl<T> Slice<T> {
    pub fn new(items: Vec<T>, next: Option<String>, previous: Option<String>) -> Self {
        Self {
            items,
            next,
            previous,
        }
    }

    pub fn map<O, Mapper: Fn(T) -> O>(self, mapper: Mapper) -> Slice<O> {
        Slice {
            items: self.items.into_iter().map(mapper).collect(),
            next: self.next,
            previous: self.previous,
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter()
    }
}

impl<T> IntoIterator for Slice<T> {
    type Item = T;

    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<T> Default for Slice<T> {
    fn default() -> Self {
        Self {
            items: Default::default(),
            next: Default::default(),
            previous: Default::default(),
        }
    }
}

pub const DEFAULT_LIMIT: usize = 200;

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Paginate {
    Forward { after: String, first: usize },
    Backward { before: String, last: usize },
}

impl Paginate {
    pub fn is_forward(&self) -> bool {
        matches!(self, Self::Forward { .. })
    }

    pub fn is_backward(&self) -> bool {
        matches!(self, Self::Backward { .. })
    }
}

impl Default for Paginate {
    fn default() -> Self {
        Self::Forward {
            after: Default::default(),
            first: DEFAULT_LIMIT,
        }
    }
}

impl Paginate {
    pub fn after(after: impl Into<String>) -> Self {
        Self::forward(after, DEFAULT_LIMIT)
    }

    pub fn before(before: impl Into<String>) -> Self {
        Self::backward(before, DEFAULT_LIMIT)
    }

    pub fn first(limit: usize) -> Self {
        Self::forward(String::default(), limit)
    }

    pub fn last(limit: usize) -> Self {
        Self::backward(String::default(), limit)
    }

    pub fn forward(after: impl Into<String>, first: usize) -> Self {
        Self::Forward {
            after: after.into(),
            first,
        }
    }

    pub fn backward(before: impl Into<String>, last: usize) -> Self {
        Self::Backward {
            before: before.into(),
            last,
        }
    }
}
