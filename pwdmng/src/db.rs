
use sqlx::{SqlitePool, Error, Row};
use std::fs::{self, File};
use crate::user;

#[derive(Debug)]
pub struct Sdb {
    active: bool,
    address: String,
    pool: Option<SqlitePool>, // the connection pool. ensure the Sdb is the owner of the pool.
}

fn error_print(func: &str, line: u32, msg: &str) {
    println!("\x1b[91m{}-{}:\x1b[0m {}", func, line, msg)
}

pub fn check_or_create_file(f: &str) {
    if fs::metadata(f).is_ok(){
        return
    }
    let _file = match File::create(f) {
        Ok(_) => (),
        Err(e) => panic!("create file failed:{}", e),
    };
}

impl Sdb {
    pub fn new(addr: String) -> Sdb {
        Sdb {
            active: false,
            address: addr,
            pool: None,
        }
    }

    // init will ensure the database is active. and has create the connection pool.
    pub async fn init(&mut self) {
        self.pool = match SqlitePool::connect(&self.address).await {
            Ok(pool) => Some(pool),
            Err(_) => {
                let err_msg = "Error connecting to database: ".to_owned() + &self.address;
                error_print("init", 1, &err_msg);
                return;
            }
        };
        self.active = true;
        if !self.check_table_exist().await {
            if !self.init_db().await {
                error_print("init", 2, "Error init database.");
                return;
            }
        }
    }
 

    pub fn check_active(&self) -> bool {
        self.active
    }

    // check if the table user is exist in the database.
    async fn check_table_exist(&self) -> bool {
        if !self.check_active() {
            error_print("check_table_exist", 1, "Error: database is not active.");
            //println!("\x1b[91mError: database is not active.\x1b[0m");
            return false;
        }
        let sql = String::from("select name from sqlite_master where type='table' and name='user';");

        let result = sqlx::query(sql.as_str())
            .fetch_one(self.pool.as_ref().unwrap())
            .await;
        match result {
            Ok(res) => {
                let name: String = res.get("name");
                println!("name is: {}", name);
                if name != "" {
                    return true;
                }
                error_print("check_table_exist", 2, "table user is exist.");
                false
            },
            Err(_) => false,
        }
    }

    // create the table user in the database.
    async fn init_db(&self) -> bool {
        if !self.check_active() {
            error_print("init_db", 1, "Error: database is not active.");
            //println!("\x1b[91mError: database is not active.\x1b[0m");
            return false;
        }
        let sql = String::from("create table user (
            id integer primary key autoincrement,
            username text not null,
            password text not null,
            email text not null,
            phone text not null,
            url text not null,
            web text not null
        );");
        let result = sqlx::query(sql.as_str())
            .execute(self.pool.as_ref().unwrap())
            .await;
        match result {
            Ok(_) => true,
            Err(err) => {
                // if the table create failed, just panic the program.
                panic!("{}",err);
                //false
            },
        }
    }

    // get web pwd by username.
    pub async fn get_with_web_user(&self, web: String, username: String) -> Result<user::User, Error> {
        if !self.check_active() {
            error_print("get_with_web_user", 1, "Error: database is not active.");
            //println!("\x1b[91mError: database is not active.\x1b[0m");
            return Err(Error::Io(std::io::Error::new(std::io::ErrorKind::Other, "database is not active.")));
        }
        let sql = String::from("select * from user where web=? and username=?;");
        let user = sqlx::query_as::<_, user::User>(sql.as_str())
            .bind(web)
            .bind(username)
            .fetch_one(self.pool.as_ref().unwrap())
            .await;
        match user {
            Ok(user) => Ok(user),
            Err(err) => Err(err),
        }
    }

    pub async fn get_all_user(&self) -> Vec<user::User> {
        if !self.check_active() {
            error_print("get_all_user", 1, "Error: database is not active.");
            //println!("\x1b[91mError: database is not active.\x1b[0m");
            return Vec::new();
        }
        let sql = String::from("select * from user;");
        let users = sqlx::query_as::<_, user::User>(sql.as_str())
            .fetch_all(self.pool.as_ref().unwrap())
            .await;
        match users {
            Ok(users) => users,
            Err(_) => Vec::new(),
        }
    }

    pub async fn update_password(&self, id: u32, pwd: String) -> bool {
        if !self.check_active() {
            error_print("udpate_password", 1, "Error: database is not active.");
            //println!("\x1b[91mError: database is not active.\x1b[0m");
            return false;
        }
        let sql = String::from("update user set password=? where id=?;");
        let result = sqlx::query(sql.as_str())
            .bind(pwd)
            .bind(id)
            .execute(self.pool.as_ref().unwrap())
            .await;
        match result {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub async fn get_id(&self, username: String, web: String, url: String) -> u32 {
        if !self.check_active() {
            error_print("get_id", 1, "Error: database is not active.");
            //println!("\x1b[91mError: database is not active.\x1b[0m");
            return 0;
        }
        let sql = String::from("select id from user where username=? and web=? and url=?;");
        let result = sqlx::query(sql.as_str())
            .bind(username)
            .bind(web)
            .bind(url)
            .fetch_one(self.pool.as_ref().unwrap())
            .await;
        match result {
            Ok(row) => {
                let id: u32 = row.get(0);
                id
            }
            Err(_) => 0,
        }
    }
    // delete a user.
    pub async fn delete_user(&self, id: i32) -> u32 {
        if !self.check_active() {
            error_print("delete_user", 1,"Error: database is not active");
            return 0;
        }
        let sql = String::from("delete from user where id = ?;");
        let result = sqlx::query(sql.as_str())
            .bind(id)
            .execute(self.pool.as_ref().unwrap())
            .await;
        match result {
            Ok(row) =>  1,
            Err(_) => 0,
        }
    }

    // add a new user.
    pub async fn add_user(&self, u: user::User) -> u32 {
        if !self.check_active() {
            error_print("add_user", 1, "Error: database is not active.");
            //println!("\x1b[91mError: database is not active.\x1b[0m");
            return 0;
        }

        let sql = String::from("insert into user (username, password, email, phone, url, web) values (?, ?, ?, ?, ?, ?);");
        let result = sqlx::query(sql.as_str())
            .bind(u.username)
            .bind(u.password)
            .bind(u.email)
            .bind(u.phone)
            .bind(u.url)
            .bind(u.web)
            .execute(self.pool.as_ref().unwrap())
            .await;
        match result {
            Ok(row) => row.last_insert_rowid() as u32,
            Err(_e) => {
                error_print("add_user", 2, "insert user into database failed!");
                //panic!("{}", e);
                0
            },
        }
    }
}

