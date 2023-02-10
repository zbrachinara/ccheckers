use common::{events, model, update};

mod common;

fn main() {
    nannou::app(model).event(events).update(update).run()
}
