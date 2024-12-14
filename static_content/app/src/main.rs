extern crate actix;
#[macro_use]
extern crate patoka;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate slog;

mod tasks;

fn main() {
    patoka::run_app("static_content", || tasks::start());
}
