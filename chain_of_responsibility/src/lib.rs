mod check_user;
mod user;

use check_user::{CheckEmail, CheckId, CheckName, CheckUser};
use user::User;

pub fn test() {
    let user_incorrect = User {
        id: 1234,
        name: "Captain America".to_string(),
        email: "abcdef".to_string(),
    };

    let user_correct = User {
        id: 1234,
        name: "Iron Man".to_string(),
        email: "ironman@avengers.com".to_string(),
    };

    validate_user(user_incorrect);
    validate_user(user_correct);
}

fn validate_user(user: User) {
    let check_email = CheckEmail { next: None };
    let check_name = CheckName {
        next: Some(Box::new(&check_email)),
    };
    let check_id = CheckId {next: Some(Box::new(&check_name))};

    match check_id.check(&user) {
        Ok(()) => {
            println!("user: {}, succesfully validated", user.name);
        }
        Err(err) => {
            println!("user: {}, validation error: {}", user.name, err);
        }
    }
}
