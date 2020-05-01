use crate::observer::{Observable, Observer};
use std::rc::Weak;

pub struct NewsAgency<T: Observer> {
    pub channels: Vec<Weak<T>>,
    pub news: String,
}

impl<T> NewsAgency<T>
where
    T: Observer + PartialEq,
{
    pub fn add_news(&mut self, news: String) {
        self.news = news;
        self.notify_observers();
    }

    pub fn default() -> NewsAgency<T> {
        NewsAgency {
            channels: Vec::new(),
            news: String::from(""),
        }
    }
}

impl<T> Observable<T> for NewsAgency<T>
where
    T: Observer + PartialEq,
{
    fn add_observer(&mut self, observer: Weak<T>) {
        self.channels.push(observer);
    }

    fn delete_observer(&mut self, observer: &T) {
        if let Some(idx) = self.channels.iter().position(|x| match x.upgrade() {
            None => false,
            Some(obs) => *obs == *observer,
        }) {
            self.channels.remove(idx);
        }
    }

    fn notify_observers(&mut self) {
        for channel in self.channels.iter() {
            if let Some(obs) = channel.upgrade() {
                obs.update(&self.news);
            }
        }
    }
}
