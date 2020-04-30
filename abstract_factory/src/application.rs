use crate::factory::GUIFactory;
use crate::buttons::Button;
use crate::progress_bar::ProgressBar;

pub struct Application {
    button: Box<dyn Button>,
    progress_bar: Box<dyn ProgressBar>,
}

impl Application {
    pub fn create(factory: Box<dyn GUIFactory>) -> Application {
        Application {
            button: factory.create_button(),
            progress_bar: factory.create_progress_bar(),
        }
    }

    pub fn draw(&self) {
        self.button.draw();
        self.progress_bar.draw();
    }
}