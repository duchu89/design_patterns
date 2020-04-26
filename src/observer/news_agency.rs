use crate::observer::observer::{Observable, Observer};

pub struct NewsAgency<'a, T: Observer> {
    pub channels: Vec<Box<&'a T>>,
    pub news: String,
}

impl<'a, T> NewsAgency<'a, T>
where
    T: Observer + PartialEq,
{
    pub fn add_news(&mut self, news: String) {
        self.news = news;
        self.notify_observers();
    }

    pub fn default() -> NewsAgency<'a, T> {
        NewsAgency {
            channels: Vec::new(),
            news: String::from(""),
        }
    }
}

impl<'a, T> Observable<'a, T> for NewsAgency<'a, T>
where
    T: Observer + PartialEq,
{
    fn add_observer(&mut self, observer: &'a T) {
        self.channels.push(Box::new(observer));
    }

    fn delete_observer(&mut self, observer: &'a T) {
        if let Some(idx) = self.channels.iter().position(|x| **x == observer) {
            self.channels.remove(idx);
        }
    }

    fn notify_observers(&mut self) {
        for channel in self.channels.iter() {
            channel.update(&self.news);
        }
    }
}
