/*extern crate anyhow;
extern crate celery;

use anyhow::Result;
use celery::beat::CronSchedule;
use celery::broker::AMQPBroker;

#[tokio::main]
async fn main() -> Result<()> {
    let mut beat = celery::beat!(
        broker = AMQPBroker { std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672".into()) },
            tasks = [
                "add" => {
                    sbox::tasks::add,
                    schedule = CronSchedule::from_string("*/1 * * * *")?,
                    args = (1, 2),
                },
            ],
            task_routes = [
                "*" => "celery",
            ],
        ).await.expect("Error creating celery beat instance");

    beat.start().await.expect("Error starting celery beat");

    Ok(())
}
*/
