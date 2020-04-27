use crate::abstract_factory::buttons::{Button, MaterialButton, CupertinoButton};
use crate::abstract_factory::progress_bar::{ProgressBar, MaterialProgressBar, CupertinoProgressBar};

pub trait GUIFactory {
    fn create_button(&self) -> Box<dyn Button>;
    fn create_progress_bar(&self) -> Box<dyn ProgressBar>;
}

pub struct AndroidFactory;

impl GUIFactory for AndroidFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(MaterialButton)
    }
    fn create_progress_bar(&self) -> Box<dyn ProgressBar> {
        Box::new(MaterialProgressBar)
    }
}

pub struct IOSFactory;

impl GUIFactory for IOSFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(CupertinoButton)
    }
    fn create_progress_bar(&self) -> Box<dyn ProgressBar> {
        Box::new(CupertinoProgressBar)
    }
}