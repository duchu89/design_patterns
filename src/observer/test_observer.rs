mod news_agency;
mod news_channel;
mod observer;

use observer::Observable;
use news_agency::NewsAgency;
use news_channel::NewsChannel;

pub fn test() {
    let mut news_agency = NewsAgency::default();

    let channel = NewsChannel{name: String::from("ABC")};

    news_agency.add_observer(&channel);

    news_agency.add_news("first breaking news".to_string());

    let channel2 = NewsChannel{name: String::from("FoxNews")};
    news_agency.add_observer(&channel2);

    news_agency.add_news("second breaking news".to_string());

    news_agency.delete_observer(&channel);
    news_agency.add_news("third breaking news".to_string());
}
