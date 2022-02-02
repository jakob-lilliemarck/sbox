extern crate anyhow;
extern crate celery;

use anyhow::Result;

#[tokio::main]
async fn main() {
    let my_app = sbox::tasks::create_app();
    my_app.consume_from(&["celery"]).await.unwrap();
}
