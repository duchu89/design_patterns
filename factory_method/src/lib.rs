mod phone;
mod factory;
mod specification;

use factory::{Factory, AppleFactory, GoogleFactory};
use specification::Specification;

pub fn test() {
    let apple_factory = AppleFactory;
    let google_factory = GoogleFactory;

    let small_apple_phone = apple_factory.new_phone(Specification::SMALL);
    small_apple_phone.desc();

    let big_apple_phone = apple_factory.new_phone(Specification::BIG);
    big_apple_phone.desc();

    let small_google_phone = google_factory.new_phone(Specification::SMALL);
    small_google_phone.desc();

    let big_google_phone = google_factory.new_phone(Specification::BIG);
    big_google_phone.desc();
}