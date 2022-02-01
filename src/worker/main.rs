#[macro_use]
extern crate celery;
extern crate anyhow;
extern crate dotenv;
extern crate env_logger;

use celery::TaskResult;

#[celery::task]
fn add(x: i32, y: i32) -> TaskResult<i32> {
    Ok(x + y)
}

#[tokio::main]
async fn main() {
    let my_app = celery::app!(
        broker = AMQP { std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672".into()) },
        tasks = [add],
        task_routes = [],
    );

    my_app.send_task(add::new(1, 2)).await;
}
