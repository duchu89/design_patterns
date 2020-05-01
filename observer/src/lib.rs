
mod news_agency;
mod news_channel;
mod observer;

use std::rc::Rc;
use crate::observer::Observable;
use news_agency::NewsAgency;
use news_channel::NewsChannel;

pub fn test() {
    let mut news_agency = NewsAgency::default();

    let abc_channel = Rc::new(NewsChannel{name: String::from("ABC")});

    println!("adding observer: {}", &abc_channel.name);
    news_agency.add_observer(Rc::downgrade(&abc_channel));
    println!("publishing new news");
    news_agency.add_news("first breaking news".to_string());

    let fox_news_channel = Rc::new(NewsChannel{name: String::from("FoxNews")});

    println!("adding observer: {}", &fox_news_channel.name);
    news_agency.add_observer(Rc::downgrade(&fox_news_channel));
    println!("publishing new news");
    news_agency.add_news("second breaking news".to_string());

    {
        let bbc_channel = Rc::new(NewsChannel{name: String::from("BBC")});
        println!("adding observer: {}", &bbc_channel.name);
        news_agency.add_observer(Rc::downgrade(&bbc_channel));
        println!("publishing new news");
        news_agency.add_news("scoped breaking news".to_string());
    }

    println!("removing observer: {}", &abc_channel.name);
    news_agency.delete_observer(&abc_channel);
    println!("publishing new news");
    news_agency.add_news("third breaking news".to_string());
}
