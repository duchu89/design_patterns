use crate::observer::observer::{Observer, Observable};

pub struct NewsAgency<'a> {
    pub channels: Vec<Box<&'a dyn Observer>>,
    pub news: String,
}

impl <'a>NewsAgency<'a> {
    pub fn add_news(&mut self, news: String) {
        self.news = news;
        self.notify_observers();
    }

    pub fn default() -> NewsAgency<'a> {
        NewsAgency {
            channels: Vec::new(),
            news: String::from(""),
        }
    }
}

impl <'a>Observable<'a> for NewsAgency<'a> {
    fn add_observer(&mut self, observer: &'a dyn Observer) {
         self.channels.push(Box::new(observer));
    }

    fn delete_observer(&mut self, observer:  &'a dyn Observer) {
        //TODO
    }

	fn notify_observers(&mut self) {
        for channel in self.channels.iter() {
            channel.update(&self.news);
        }
    }
}