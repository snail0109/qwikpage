#[get("/<name>/<age>")]
pub fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}
