// Unit tests for Cache
#[cfg(test)]
mod tests {
    use tokio::sync::Mutex;
    struct Cache<T>(Mutex<Vec<T>>);

    impl<T> Cache<T> {
        async fn get(&self, index: usize) -> Option<T>
        where
            T: Clone,
        {
            self.0.lock().await.get(index).cloned()
        }
    }

    #[tokio::test]
    async fn test_get_existing_index() {
        let cache = Cache(Mutex::new(vec![
            "Rust ".to_string(),
            "is ".to_string(),
            "awesome!".to_string(),
        ]));

        let result = cache.get(0).await;
        assert_eq!(result, Some("Rust ".to_string()));

        let result = cache.get(1).await;
        assert_eq!(result, Some("is ".to_string()));

        let result = cache.get(2).await;
        assert_eq!(result, Some("awesome!".to_string()));
    }

    #[tokio::test]
    async fn test_get_non_existing_index() {
        let cache = Cache(Mutex::new(vec![
            "Rust ".to_string(),
            "is ".to_string(),
            "awesome!".to_string(),
        ]));

        let result = cache.get(3).await;
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_get_empty_cache() {
        let cache: Cache<String> = Cache(Mutex::new(vec![]));

        let result = cache.get(0).await;
        assert_eq!(result, None);
    }
}
