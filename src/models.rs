#[derive(Queryable)]
pub struct Source {
    pub id: i32,
    pub lang: String,
    pub src: String,
}
