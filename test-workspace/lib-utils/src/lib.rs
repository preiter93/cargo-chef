pub fn helper() {
    println!(
        "lib-utils helper called with runtime: {}",
        get_runtime_name()
    );
}

pub fn get_runtime_name() -> &'static str {
    "tokio"
}

pub async fn do_async_work() {
    tokio::task::yield_now().await;
}
