//! Parallel execution utilities

use futures::future::{join_all, BoxFuture};
use std::future::Future;
use tokio::sync::Semaphore;
use std::sync::Arc;
use tracing::debug;

/// Execute multiple async tasks with bounded parallelism
pub async fn execute_parallel<T, F, Fut>(
    items: Vec<T>,
    max_concurrent: usize,
    f: F,
) -> Vec<Fut::Output>
where
    T: Send + 'static,
    F: Fn(T) -> Fut + Send + Sync + 'static,
    Fut: Future + Send + 'static,
    Fut::Output: Send + 'static,
{
    let semaphore = Arc::new(Semaphore::new(max_concurrent));
    let f = Arc::new(f);

    let futures: Vec<_> = items
        .into_iter()
        .map(|item| {
            let sem = semaphore.clone();
            let func = f.clone();
            async move {
                let _permit = sem.acquire().await.unwrap();
                func(item).await
            }
        })
        .collect();

    join_all(futures).await
}

/// Batch processor for large datasets
pub struct BatchProcessor<T> {
    items: Vec<T>,
    batch_size: usize,
}

impl<T> BatchProcessor<T> {
    pub fn new(items: Vec<T>, batch_size: usize) -> Self {
        Self { items, batch_size }
    }

    pub fn batches(&self) -> impl Iterator<Item = &[T]> {
        self.items.chunks(self.batch_size)
    }

    pub fn batch_count(&self) -> usize {
        (self.items.len() + self.batch_size - 1) / self.batch_size
    }
}

/// Rate limiter for API calls
pub struct RateLimiter {
    semaphore: Arc<Semaphore>,
    delay_ms: u64,
}

impl RateLimiter {
    pub fn new(max_concurrent: usize, delay_ms: u64) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
            delay_ms,
        }
    }

    pub async fn acquire(&self) -> tokio::sync::SemaphorePermit<'_> {
        let permit = self.semaphore.acquire().await.unwrap();
        if self.delay_ms > 0 {
            tokio::time::sleep(std::time::Duration::from_millis(self.delay_ms)).await;
        }
        permit
    }
}

/// Parallel map with error handling
pub async fn parallel_map<T, U, E, F, Fut>(
    items: Vec<T>,
    max_concurrent: usize,
    f: F,
) -> Vec<Result<U, E>>
where
    T: Send + 'static,
    U: Send + 'static,
    E: Send + 'static,
    F: Fn(T) -> Fut + Send + Sync + Clone + 'static,
    Fut: Future<Output = Result<U, E>> + Send + 'static,
{
    let semaphore = Arc::new(Semaphore::new(max_concurrent));

    let futures: Vec<_> = items
        .into_iter()
        .map(|item| {
            let sem = semaphore.clone();
            let func = f.clone();
            async move {
                let _permit = sem.acquire().await.unwrap();
                func(item).await
            }
        })
        .collect();

    join_all(futures).await
}

/// Retry with exponential backoff
pub async fn retry_with_backoff<T, E, F, Fut>(
    f: F,
    max_retries: u32,
    initial_delay_ms: u64,
) -> Result<T, E>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T, E>>,
{
    let mut delay = initial_delay_ms;
    let mut last_error = None;

    for attempt in 0..max_retries {
        match f().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                last_error = Some(e);
                if attempt < max_retries - 1 {
                    debug!("Attempt {} failed, retrying in {}ms", attempt + 1, delay);
                    tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
                    delay *= 2; // Exponential backoff
                }
            }
        }
    }

    Err(last_error.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_parallel_execution() {
        let items = vec![1, 2, 3, 4, 5];
        let results = execute_parallel(items, 2, |x| async move { x * 2 }).await;
        assert_eq!(results.len(), 5);
    }

    #[test]
    fn test_batch_processor() {
        let items = vec![1, 2, 3, 4, 5, 6, 7];
        let processor = BatchProcessor::new(items, 3);
        assert_eq!(processor.batch_count(), 3);
    }
}
