use sfml::window::{ContextSettings, Style};
use sfml::graphics::{Color, RenderWindow, RenderTarget};
use sfml::window::Event;

use crate::entities::ball;

pub struct Engine<'a> {
    window: RenderWindow,
    ball: ball::Ball<'a>
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
        Engine {
            window,
            ball
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
    }

    pub fn render(&mut self) {
        self.window.clear(Color::BLACK);
        self.ball.render(&mut self.window);
        self.window.display();
    }
}
