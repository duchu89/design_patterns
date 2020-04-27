mod application;
mod factory;
mod buttons;
mod progress_bar;

use factory::{AndroidFactory, IOSFactory};
use application::Application;


pub fn test() {
    let android_application = android_application();
    android_application.draw();

    let ios_application = ios_application();
    ios_application.draw();
}

fn android_application() -> Application {
    let factory = AndroidFactory;
    Application::create(Box::new(factory))
}

fn ios_application() -> Application {
    let factory = IOSFactory;
    Application::create(Box::new(factory))
}