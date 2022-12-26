use tokio_retry::{
    strategy::{jitter, ExponentialBackoff},
    Retry,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogVec<T> {
    storage: Vec<T>,
}

impl<T> LogVec<T> {
    pub fn new() -> Self {
        Self { storage: vec![] }
    }

    pub fn insert(&mut self, value: T) {
        self.storage.push(value);
    }

    /// Try's to get data from storage until it is presents
    pub async fn get(&self, i: usize) -> Option<&T> {
        let retry_strategy = ExponentialBackoff::from_millis(500)
            .map(jitter) // add jitter to delays
            .take(3); // limit to 3 retries

        let result = Retry::spawn(retry_strategy, || self.get_internal(i)).await;

        result.ok()
    }

    async fn get_internal(&self, i: usize) -> Result<&T, ()> {
        if let Some(item) = self.storage.get(i) {
            return Ok(item);
        } else {
            return Err(());
        }
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

    #[tokio::test]
    async fn get_not_present_entry() {
        let mut logvec: LogVec<u8> = LogVec::new();

        let value: u8 = 1;

        let mut logvec_a = Arc::new(Mutex::new(logvec));

        logvec_a.lock().await.insert(value);

        let logvec_b = logvec_a.clone();
        tokio::spawn(async move {
            let logvec_b = logvec_b.lock().await;
            assert_eq!(logvec_b.get(0).await, Some(&value));
            assert_eq!(logvec_b.get(1).await, Some(&value));
        });

        logvec_a.lock().await.insert(value);
    }
}
