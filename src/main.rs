fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use futures::future::FutureExt;
    use std::panic;

    #[tokio::test]
    async fn test_success() {
        run_test(async {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            assert!(true);
        }).await;
    }

    #[tokio::test]
    async fn test_failure() {
        run_test(async {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            assert!(false);
        }).await;
    }

    async fn setup() {
        println!("before setup");
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        println!("after setup");
    }

    async fn teardown() {
        println!("before teardown");
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        println!("after teardown");
    }

    async fn run_test<T>(test: T)
    where
        T: std::future::Future + Send + 'static,
        T::Output: Send + 'static,
    {
        setup().await;

        let result = panic::AssertUnwindSafe(test).catch_unwind().await;

        teardown().await;

        if let Err(err) = result {
            panic::resume_unwind(err);
        }
    }
}