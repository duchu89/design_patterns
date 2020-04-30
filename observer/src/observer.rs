pub trait Observer {
	fn update(&self, news: &String);
}

pub trait Observable<'a, T: Observer> {
	fn add_observer(&mut self, observer: &'a T);
	fn delete_observer(&mut self, observer: &'a T);
	fn notify_observers(&mut self);
}
