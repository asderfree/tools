use std::fmt;
mod db;
mod user;
mod cmd;

const DB_URL: &str = "sqlite://.pwd.db";

fn show_line(id: u32, name: String, pwd: String, web: String, url: String, email: String, phone: String) {
    println!("{}", format_args!(
        "|{:^5}|{:^10}|{:^15}|{:^15}|{:^15}|{:^15}|{:^15}|",
        id, name, pwd, web, url, email, phone
        ));
}

fn show_border(c: &str){
    println!("{}", c.repeat(98));
}

fn show_header(){
    show_border("=");
    println!("{}", format_args!(
        "|{:^5}|{:^10}|{:^15}|{:^15}|{:^15}|{:^15}|{:^15}|",
        "id", "name", "password", "web", "url", "email", "phone"
    ));
}


#[tokio::main]
async fn main() {
    db::check_or_create_file(".pwd.db");
    // init a database.
    let mut s = db::Sdb::new(DB_URL.to_string());
    s.init().await;

    let c = cmd::get_cli();
    println!("{:?}", c);
    if !c.check_args() {
        println!("error args! use -h get usage");
        return;
    }
    let t: cmd::CmdType = c.get_cmd();
    match t {
        cmd::CmdType::Add => {
            let u = user::User::new(c.get_name(), c.get_password(), c.get_email(), c.get_telephone(), c.get_url(), c.get_web());
            let add_id: u32 = s.add_user(u).await;   
            println!("add user success!, the new user id is: {}", add_id);
        },
        cmd::CmdType::Del => {
            let id = s.delete_user(c.get_id()).await;
            if id == 0 {
                println!("delete user failed!");
            } else {
                println!("success delete user: {}", c.get_id());
            }
        },
        cmd::CmdType::Mod => {
            let id = c.get_id() as u32;
            let pwd = c.get_password();
            let res = s.update_password(id, pwd.to_string()).await;
            if !res {
                println!("update {}'s password to {} failed", id, pwd);
            } else {
                println!("success update {}'s password to {}", id, pwd);
            }
        },
        cmd::CmdType::Show => {
            println!("going to show the users: ");
            let users = s.get_all_user().await;
            show_header();
            for u in users.iter() {
                u.show_line();
            }
            show_border("=");
        },
        _ => {
            println!("unsupport cmd, usage: pwdmng -h");
        }
    }
    //db::check_or_create_file(".pwd.db");
    //// init a database.
    //let mut s = db::Sdb::new(DB_URL.to_string());
    //s.init().await;
    //// get all user.
    //let users = s.get_all_user().await;
    //// print all user.
    //for u in users.iter() {
    //    u.show_line();
    //}
    //let u = user::User::new("zhangsan", "1", "2", "3", "4", "5");
    //s.add_user(u).await;
}

