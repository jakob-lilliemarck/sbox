#[macro_use]
extern crate celery;

#[tokio::main]
async fn main() {
    let my_app = sbox::celery::create_app();
    /*
    TODO
    - Keep alive?
    - Consume tasks!
    */
}
