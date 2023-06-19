#[allow(non_snake_case)]

use sfml::graphics::*;
use sfml::system::*;

pub struct Ball<'a>{
    pub property: CircleShape<'a>,
    radius: f32,
    velocity: Vector2f,
    acceleration: Vector2f
}

impl Ball<'_>{
    pub fn new(radius: f32) -> Self {
        let mut property = CircleShape::new(radius, 30_usize);
        property.set_fill_color(Color::WHITE);
        property.set_origin((radius, radius));
        property.set_position(Vector2f::new(800_f32/2_f32, 600_f32/2_f32));

        Ball {
            property,
            radius,
            velocity: Vector2f::new(0_f32, 0_f32),
            acceleration: Vector2f::new(0_f32, 0_f32)
        }
    }

    //updater and displayer
    pub fn update(&mut self){
        //...
    }
    pub fn render(&mut self, target: &mut dyn RenderTarget){
        target.draw(&self.property);
    }

    //accessors and mutators
    pub fn getRadius(&mut self) -> f32{
        return self.radius;
    }
    pub fn getVelocity(&mut self) -> Vector2f{
        return self.velocity;
    }
    pub fn getAcceleration(&mut self) -> Vector2f{
        return self.acceleration;
    }
    pub fn setVelocity(&mut self, velocity: Vector2f){
        self.velocity = velocity;
    }
    pub fn setAcceleration(&mut self, acceleration: Vector2f){
        self.acceleration = acceleration;
    }
}
