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

pub struct CeleryAppInstance {
    pub app: &'static celery::Celery<AMQPBroker>,
}

use rocket::request::{self, FromRequest, Request};
use rocket_okapi::{
    gen::OpenApiGenerator, request::OpenApiFromRequest, request::RequestHeaderInput, OpenApiError,
};

#[rocket::async_trait]
impl<'r> FromRequest<'r> for CeleryAppInstance {
    type Error = rocket::error::Error;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        request::Outcome::Success(CeleryAppInstance { app: create_app() })
    }
}

impl OpenApiFromRequest<'static> for CeleryAppInstance {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> Result<RequestHeaderInput, OpenApiError> {
        Ok(RequestHeaderInput::None)
    }
}
