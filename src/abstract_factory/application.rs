use crate::abstract_factory::factory::GUIFactory;
use crate::abstract_factory::buttons::Button;
use crate::abstract_factory::progress_bar::ProgressBar;

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