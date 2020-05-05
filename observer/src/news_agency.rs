use crate::observer::{Observable, Observer};
use std::cell::RefCell;
use std::rc::Weak;

pub struct NewsAgency<T: Observer> {
    pub channels: RefCell<Vec<Weak<T>>>,
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
            channels: RefCell::new(Vec::new()),
            news: String::from(""),
        }
    }
}

impl<T> Observable<T> for NewsAgency<T>
where
    T: Observer + PartialEq,
{
    fn add_observer(&self, observer: Weak<T>) {
        self.channels.borrow_mut().push(observer);
    }

    fn delete_observer(&self, observer: &T) {
        let mut mut_channels = self.channels.borrow_mut();
        if let Some(idx) = mut_channels.iter().position(|x| match x.upgrade() {
            None => false,
            Some(obs) => *obs == *observer,
        }) {
            mut_channels.remove(idx);
        }
    }

    fn notify_observers(&self) {
        for channel in self.channels.borrow().iter() {
            if let Some(obs) = channel.upgrade() {
                obs.update(&self.news);
            }
        }
    }
}
