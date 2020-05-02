use std::rc::Weak;

pub trait Observer {
	fn update(&self, news: &String);
}

pub trait Observable<T: Observer> {
	fn add_observer(&self, observer: Weak<T>);
	fn delete_observer(&self, observer: &T);
	fn notify_observers(&self);
}
