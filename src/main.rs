#[derive(Debug, PartialEq, Clone)]
struct Foo {
    boolean: bool,
    number: i64,
    string: String,
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::future::FutureExt;
    use pretty_assertions::{assert_eq, assert_ne};
    use std::panic;

    #[test]
    fn test_sample() {
        assert_eq!(
            Foo { boolean: true, number: 0, string: "Hello".to_string() },
            Foo { boolean: false, number: 1, string: "Hello World".to_string() }
        );
    }

    #[test]
    fn test_success() {
        run_test(|| {
            assert_ne!(
                Foo { boolean: true, number: 0, string: "Hello".to_string() },
                Foo { boolean: false, number: 1, string: "Hello World".to_string() }
            );
        });
    }

    #[test]
    fn test_failure() {
        run_test(|| {
            assert_eq!(
                Foo { boolean: true, number: 0, string: "Hello".to_string() },
                Foo { boolean: false, number: 1, string: "Hello World".to_string() }
            );
        });
    }

    fn setup() {
        println!("before setup");
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!("after setup");
    }

    fn teardown() {
        println!("before teeardown");
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!("after teeardown");
    }

    fn run_test<F>(f: F)
    where
        F: FnOnce(),
    {
        setup();
        let result = panic::catch_unwind(panic::AssertUnwindSafe(f));
        teardown();

        if let Err(err) = result {
            panic::resume_unwind(err);
        }
    }

    #[tokio::test]
    async fn test_success_async() {
        run_test_async(async {
            assert_ne!(
                Foo { boolean: true, number: 0, string: "Hello".to_string() },
                Foo { boolean: false, number: 1, string: "Hello World".to_string() },
            );
        }).await;
    }

    #[tokio::test]
    async fn test_failure_async() {
        run_test_async(async {
            assert_eq!(
                Foo { boolean: true, number: 0, string: "Hello".to_string() },
                Foo { boolean: false, number: 1, string: "Hello World".to_string() },
            );
        }).await;
    }

    async fn setup_async() {
        println!("before setup_async");
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        println!("after setup_async");
    }

    async fn teardown_async() {
        println!("before teardown_async");
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        println!("after teardown_async");
    }

    async fn run_test_async<F>(test: F)
    where
        F: std::future::Future,
    {
        setup_async().await;
        let result = panic::AssertUnwindSafe(test).catch_unwind().await;
        teardown_async().await;

        if let Err(err) = result {
            panic::resume_unwind(err);
        }
    }
}
