extern crate anyhow;
extern crate celery;
use celery::RegularSchedule;
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    let mut beat = celery::beat!(
        broker = AMQP { std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672".into()) },
        task_routes = [
            "*" => "celery",
        ],
    );

    let add_schedule = RegularSchedule::new(Duration::from_secs(1));
    beat.schedule_task(sbox::celery::add::new(1, 2), add_schedule);

    beat.start().await;
}
