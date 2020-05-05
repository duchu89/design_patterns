
mod broker;
mod command;
mod stock;

use broker::Broker;
use stock::Stock;
use command::{BuyStock, SellStock};

pub fn test() {
    let stock_shell = Stock::new(String::from("Royal Dutch Shell A"), 10);
    let stock_coca_cola = Stock::new(String::from("Coca Cola"), 50);

    let buy_stock_order = BuyStock::new(&stock_shell);
    let sell_stock_order = SellStock::new(&stock_shell);
    let buy_stock_order2 = BuyStock::new(&stock_coca_cola);

    let broker = Broker::new();
    broker.take_order(Box::new(buy_stock_order));
    broker.take_order(Box::new(sell_stock_order));
    broker.take_order(Box::new(buy_stock_order2));

    broker.place_orders();
}