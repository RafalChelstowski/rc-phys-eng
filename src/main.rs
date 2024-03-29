mod application;

use application::Application;

fn main() {
    let mut app = Application::new();

    app.setup();

    while app.is_running() {
        app.input();
        app.update();
        app.render();
    }

    app.destroy();
}
