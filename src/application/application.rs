use super::graphics;

pub struct Application {
    running: bool,
}

impl Application {
    pub fn new() -> Self {
        Application { running: false }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn setup(&mut self) {
        self.running = graphics::open_window();

        // TODO: setup objects in the scene
    }

    pub fn input(&mut self) {
        for event in sdl2::event::poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => self.running = false,
                sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => self.running = false,
                _ => {}
            }
        }
    }

    pub fn update(&mut self) {
        // TODO: update all objects in the scene
    }

    pub fn render(&mut self) {
        graphics::clear_screen(0xFF056263);
        graphics::draw_fill_circle(200, 200, 40, 0xFFFFFFFF);
        graphics::render_frame();
    }

    pub fn destroy(&mut self) {
        // TODO: destroy all objects in the scene

        graphics::close_window();
    }
}
