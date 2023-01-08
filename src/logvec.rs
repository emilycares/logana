#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogVec<'a> {
    storage: Vec<&'a [&'a str]>,
    current: Vec<&'a str>,
}

impl<'b> LogVec<'b> {
    pub fn new() -> Self {
        Self {
            storage: vec![],
            current: vec![],
        }
    }

    pub fn insert(&mut self, value: &str) {
        self.current.push(value);
    }

    pub fn collect(&mut self) -> usize {
        let slice = self.current.clone().as_slice();
        self.current = vec![];

        self.storage.push(slice);
        let index = self.storage.len() - 1;

        return index;
    }
}

#[cfg(test)]
mod tests {

    use std::sync::Arc;

    use tokio::sync::Mutex;

    use super::LogVec;

    #[tokio::test]
    async fn initialize() {
        let logvec: LogVec<u8> = LogVec::new();

        assert_eq!(logvec, LogVec { storage: vec![] });
    }

    #[tokio::test]
    async fn get_present_entry() {
        let mut logvec: LogVec<u8> = LogVec::new();

        let value: u8 = 1;

        logvec.insert(value);

        assert_eq!(logvec.get(0).await, Some(&value));
    }
}
