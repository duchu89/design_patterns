pub trait Observer {
	fn update(&self, news: &String);
}

pub trait Observable<'a> {
	fn add_observer(&mut self, observer: &'a dyn Observer);
	fn delete_observer(&mut self, observer: &'a dyn Observer);
	fn notify_observers(&mut self);
}
