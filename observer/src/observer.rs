use std::rc::Weak;

pub trait Observer {
	fn update(&self, news: &String);
}

pub trait Observable<T: Observer> {
	fn add_observer(&mut self, observer: Weak<T>);
	fn delete_observer(&mut self, observer: &T);
	fn notify_observers(&mut self);
}
