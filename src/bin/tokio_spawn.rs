use std::sync::Arc;
use tokio::sync::Mutex;

struct ConcurrentCache<T> {
    data: Mutex<Vec<T>>,
}

impl<T> ConcurrentCache<T> {
    async fn new() -> Self {
        Self {
            data: Mutex::new(Vec::new()),
        }
    }

    async fn add(&self, item: T) {
        let mut data = self.data.lock().await;
        data.push(item);
    }

    async fn get(&self, index: usize) -> Option<T>
    where
        T: Clone,
    {
        let data = self.data.lock().await;
        data.get(index).cloned()
    }
}

#[tokio::main]
async fn main() {
    let cache = Arc::new(ConcurrentCache::new().await);

    let cache_clone = cache.clone();
    let handle = tokio::spawn(async move {
        cache_clone.add("Rust ".to_string()).await;
        cache_clone.add("is ".to_string()).await;
        cache_clone.add("awesome!".to_string()).await;
    });

    handle.await.unwrap();

    for i in 0..3 {
        if let Some(item) = cache.get(i).await {
            print!("{}", item);
        } else {
            println!("Item not found");
        }
    }
    println!();
}

// Rust is awesome!
