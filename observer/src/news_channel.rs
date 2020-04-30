use crate::observer::Observer;

#[derive(PartialEq)]
pub struct NewsChannel {
    pub name: String,
}

impl Observer for NewsChannel {
    fn update(&self, news: &String) {
        println!("{} received news: {}", self.name, news);
    }
}