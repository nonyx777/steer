use sfml::window::{ContextSettings, Style};
use sfml::graphics::{Color, RenderWindow, RenderTarget, View, Transformable};
use sfml::system::{Vector2f, Vector2i};
use sfml::window::{Event, mouse};

use crate::entities::ball;

pub struct Engine<'a> {
    window: RenderWindow,
    ball: ball::Ball<'a>,
    mouse_position_view: Vector2f,
}

impl Engine<'_> {
    pub fn new(width: u32, height: u32) -> Engine<'static> {
        let mut window = RenderWindow::new(
            (width, height),
            "Window",
            Style::DEFAULT,
            &ContextSettings::default(),
        );
        window.set_framerate_limit(60);
        let mut ball = ball::Ball::new(10_f32);
        let mouse_position_view: Vector2f = Vector2f::default();
        Engine {
            window,
            ball,
            mouse_position_view,
        }
    }

    pub fn running(&mut self) -> bool {
        return self.window.is_open();
    }

    pub fn poll_event(&mut self) {
        while let Some(event) = self.window.poll_event() {
            match event {
                Event::Closed => self.window.close(),
                _ => {}
            }
        }
    }

    pub fn update(&mut self) {
        self.poll_event();
        self.mouse_position_view = self.window.map_pixel_to_coords(mouse::desktop_position(), &mut self.window.view());
        self.mouse_position_view = Vector2f::new(self.mouse_position_view.x - 95_f32, self.mouse_position_view.y - 100_f32);

        self.ball.property.set_position(self.mouse_position_view);
    }

    pub fn render(&mut self) {
        self.window.clear(Color::BLACK);
        self.ball.render(&mut self.window);
        self.window.display();
    }
}
