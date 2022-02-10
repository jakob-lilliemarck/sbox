/*extern crate celery;
use celery::broker::AMQPBroker;
use celery::task::TaskResult;
use rocket::request::{self, FromRequest, Request};
use rocket_okapi::{
    gen::OpenApiGenerator, request::OpenApiFromRequest, request::RequestHeaderInput, OpenApiError,
};

/*
Celery tasks
*/
#[celery::task]
pub fn add(x: i32, y: i32) -> TaskResult<i32> {
    println!("yaaaay: {:?} + {:?}", x, y);
    Ok(x + y)
}

/*
Create celery app with pre-registered tasks and queues
*/
pub async fn create_app<'a>() -> std::sync::Arc<celery::Celery<AMQPBroker>> {
    celery::app!(
        broker = AMQPBroker { std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672".into()) },
        tasks = [add],
        task_routes = [
            "*" => "celery",
        ],
    )
    .await.expect("Err creating celery app")
}


Rocket Request guard to send tasks from routes.
Example usage:

#[get("/source/<id>")]
pub async fn read_source(
    sender: TaskSender,
) {
    sender.send(|| task::new(1, 2));
}


pub struct Sender;

impl Sender {
    pub fn send<F: FnOnce() -> celery::task::Signature<add>>(&self, get_task: F) {
        let task = get_task();
        tokio::spawn(async {
            println!("Tokio async!");
            let celery_app = create_app().await;
            celery_app
                .send_task(task)
                .await
                .expect("Error sending celery task");
        });
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Sender {
    type Error = rocket::error::Error;

    async fn from_request(_req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        request::Outcome::Success(Sender {})
    }
}

impl OpenApiFromRequest<'static> for Sender {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> Result<RequestHeaderInput, OpenApiError> {
        Ok(RequestHeaderInput::None)
    }
}*/
