use crate::command::Command;
use std::cell::RefCell;

type MyCommand<'a> = Box<dyn Command + 'a>;

pub struct Broker<'a> {
    order_list: RefCell<Vec<MyCommand<'a>>>,
}

impl<'a> Broker<'a> {
    pub fn new() -> Broker<'a> {
        Broker {
            order_list: RefCell::new(Vec::new()),
        }
    }

    pub fn take_order(&self, order: MyCommand<'a>) {
        self.order_list.borrow_mut().push(order);
    }

    pub fn place_orders(&self) {
        let mut order_list_mut = self.order_list.borrow_mut();
        for order in order_list_mut.iter() {
            order.execute();
        }
        order_list_mut.clear();
    }
}
