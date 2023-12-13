use std::fmt;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub phone: String,
    pub url: String,    // refer url that used the password.
    pub web: String,    // the web that used the password.
}

impl User {
    pub fn new(name: &str, password: &str, email: &str, phone: &str, url: &str, web: &str) -> Self {
        User {
            id: 0,
            username: name.to_string(),
            password: password.to_string(),
            email: email.to_string(),
            phone: phone.to_string(),
            url: url.to_string(),
            web: web.to_string(),
        }
    }
    pub fn show_line(&self) {
        println!("{}", format_args!("|{:^5}|{:^10}|{:^15}|{:^15}|{:^15}|{:^15}|{:^15}|",
            self.id, self.username, self.password, self.web, self.url, self.email, self.phone
        ));
    }
}
