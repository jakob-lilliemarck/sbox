extern crate celery;

use celery::{broker::AMQPBroker, TaskResult};

#[celery::task]
pub fn add(x: i32, y: i32) -> TaskResult<i32> {
    println!("{:?} + {:?}", x, y);
    Ok(x + y)
}

pub fn create_app<'a>() -> &'a celery::Celery<AMQPBroker> {
    celery::app!(
        broker = AMQP { std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672".into()) },
        tasks = [add],
        task_routes = ["*" => "celery"],
        prefetch_count = 2,
        heartbeat = Some(10),
    )
}
