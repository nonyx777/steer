#[allow(unused_imports)]
#[allow(unused_variables)]
#[allow(dead_code)]
#[allow(non_snake_case)]

mod engine;
mod entities{
    pub mod ball;
}
mod operations;

fn main() {
    let mut engine = engine::Engine::new(800, 600);

    while engine.running() {
        engine.update();
        engine.render();
    }
}
