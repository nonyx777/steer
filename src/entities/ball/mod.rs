#[allow(non_snake_case)]

use std::time;
use rand::Rng;
use sfml::graphics::*;
use sfml::system::*;
use sfml::window::{Key};

use crate::operations;

pub struct Ball<'a>{
    pub property: CircleShape<'a>,
    radius: f32,
    velocity: Vector2f,
    acceleration: Vector2f,
    max_speed: f32,
    max_force: f32,
    wonder_point: Vector2f
}

impl Ball<'_>{
    pub fn new(radius: f32) -> Self {
        let mut property = CircleShape::new(radius, 30_usize);
        property.set_fill_color(Color::WHITE);
        property.set_origin((radius, radius));
        property.set_position(Vector2f::new(800_f32/2_f32, 600_f32/2_f32));

        let max_speed: f32 = 10_f32;
        let max_force: f32 = 0.03;
        let wonder_point: Vector2f = Vector2f::default();

        Ball {
            property,
            radius,
            velocity: Vector2f::new(1_f32, 1_f32),
            acceleration: Vector2f::new(0_f32, 0_f32),
            max_speed,
            max_force,
            wonder_point
        }
    }

    //updater and displayer
    pub fn update(&mut self, vector: &Vector2f){
        if Key::Q.is_pressed(){
            self.seek(vector);
        }

        else{
            if operations::magnitude(operations::displacement(self.wonder_point, self.getPosition())) <= 30_f32{
                self.wonder();
            }else{
                let _vec = self.wonder_point;
                self.seek(&_vec);
            }
        }
    }
    pub fn render(&mut self, target: &mut dyn RenderTarget){
        target.draw(&self.property);
    }

    //custom function
    pub fn seek(&mut self, vector: &Vector2f){
        let mut desired: Vector2f = *vector - self.getPosition();
        desired = operations::normalize(desired);
        desired *= self.max_speed;
        let steer: Vector2f = desired - self.getVelocity();

        //integration
        self.setAcceleration(steer * self.max_force);
        self.setVelocity(self.getVelocity() + self.getAcceleration());
        self.property.move_(self.getVelocity() * 0.5);

        self.setAcceleration(Vector2f::default());
    }

    pub fn wonder(&mut self){
        let random_x: f32 = rand::thread_rng().gen_range(20.0..780.0);
        let random_y: f32 = rand::thread_rng().gen_range(20.0..580.0);
        let target_point = Vector2f::new(random_x, random_y);

        self.wonder_point = target_point;
    }

    //accessors and mutators
    pub fn getRadius(&self) -> f32{
        return self.radius;
    }
    pub fn getPosition(&self) -> Vector2f{
        return self.property.position();
    }
    pub fn getVelocity(&self) -> Vector2f{
        return self.velocity;
    }
    pub fn getAcceleration(&self) -> Vector2f{
        return self.acceleration;
    }
    pub fn setPosition(&mut self, position: Vector2f){
        self.property.set_position(position);
    }
    pub fn setVelocity(&mut self, velocity: Vector2f){
        self.velocity = velocity;
    }
    pub fn setAcceleration(&mut self, acceleration: Vector2f){
        self.acceleration = acceleration;
    }
}
