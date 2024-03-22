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

#[tokio::main]
async fn main() {
    let cache = Cache(Mutex::new(vec![
        "Rust ".to_string(),
        "is ".to_string(),
        "awesome!".to_string(),
    ]));

    for i in 0..3 {
        if let Some(item) = cache.get(i).await {
            print!("{}", item);
        }
    }
}

// Rust is awesome!
