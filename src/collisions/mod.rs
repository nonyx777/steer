use sfml::graphics::*;
use sfml::system::*;

use crate::entities::ball;
use crate::operations;

//intersection check
pub fn ballOverlapping(radius_a: f32, radius_b: f32, origin_a: Vector2f, origin_b: Vector2f) -> bool{
   return operations::magnitude(origin_b - origin_a) <= (radius_a + radius_b);
}

//penetration and collision resolution
pub fn ballPenetrationResolution(ball_a: &mut ball::Ball, ball_b: &mut ball::Ball){
    let normal: Vector2f = operations::displacement(ball_b.property.position(), ball_a.property.position());
    let distance: f32 = operations::magnitude(normal);
    let penetration_depth: f32 = (ball_a.getRadius() + ball_b.getRadius()) - distance;
    let penetration_depth_vector: Vector2f = operations::unitVector(normal) * penetration_depth;
}
