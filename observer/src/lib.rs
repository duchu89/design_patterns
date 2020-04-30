mod news_agency;
mod news_channel;
mod observer;

use crate::observer::Observable;
use news_agency::NewsAgency;
use news_channel::NewsChannel;

pub fn test() {
    let mut news_agency = NewsAgency::default();

    let abc_channel = NewsChannel{name: String::from("ABC")};

    println!("adding observer: {}", &abc_channel.name);
    news_agency.add_observer(&abc_channel);
    println!("publishing new news");
    news_agency.add_news("first breaking news".to_string());

    let fox_news_channel = NewsChannel{name: String::from("FoxNews")};

    println!("adding observer: {}", &fox_news_channel.name);
    news_agency.add_observer(&fox_news_channel);
    println!("publishing new news");
    news_agency.add_news("second breaking news".to_string());

    println!("removing observer: {}", &abc_channel.name);
    news_agency.delete_observer(&abc_channel);
    println!("publishing new news");
    news_agency.add_news("third breaking news".to_string());
}
