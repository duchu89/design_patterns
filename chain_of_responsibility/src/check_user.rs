use crate::user::User;
extern crate regex;

use regex::Regex;

pub type CheckResult = Result<(), &'static str>;

pub trait CheckUser {
    fn check(&self, user: &User) -> CheckResult;
}

pub struct CheckId<'a> {
    pub next: Option<Box<&'a dyn CheckUser>>,
}

impl<'a> CheckUser for CheckId<'a> {
    fn check(&self, user: &User) -> CheckResult {
        if user.id % 2 == 0 {
            match &self.next {
                Some(v) => v.check(&user),
                None => Ok(()),
            }
        } else {
            Err("User id is wrong")
        }
    }
}

pub struct CheckName<'a> {
    pub next: Option<Box<&'a dyn CheckUser>>,
}

impl<'a> CheckUser for CheckName<'a> {
    fn check(&self, user: &User) -> CheckResult {
        if !user.name.is_empty() {
            match &self.next {
                Some(v) => v.check(&user),
                None => Ok(()),
            }
        } else {
            Err("User name is empty")
        }
    }
}

pub struct CheckEmail<'a> {
    pub next: Option<Box<&'a dyn CheckUser>>,
}

impl<'a> CheckUser for CheckEmail<'a> {
    fn check(&self, user: &User) -> CheckResult {
        let email_regex = Regex::new(
            r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
        )
        .unwrap();
        if email_regex.is_match(&user.email) {
            match &self.next {
                Some(v) => v.check(&user),
                None => Ok(()),
            }
        } else {
            Err("User email is malformed")
        }
    }
}
