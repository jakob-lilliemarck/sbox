extern crate celery;

#[tokio::main]
async fn main() {
    let my_app = sbox::celery::create_app();
    my_app.consume_from(&["celery"]).await.unwrap();
}
