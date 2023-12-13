use clap::{Parser};
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug, Clone, Copy)]
pub enum CmdType {
    Add,
    Del,
    Mod,
    Show,
    Help,
    None,
}

impl FromStr for CmdType {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cmd = s.parse::<u32>()?;
        match cmd {
            1 => Ok(Self::Add),
            2 => Ok(Self::Del),
            3 => Ok(Self::Mod),
            4 => Ok(Self::Show),
            5 => Ok(Self::Help),
            _ => Ok(Self::None),
        }
    }
}

#[derive(Parser, Debug)]
#[command(name = "pwdmng", author, version, about, long_about = None)]
pub struct Cmd {
    #[arg(short, long, required = false)]
    cmd: Option<CmdType>,
    #[arg(short, long, required = false)]
    id: Option<u32>,
    #[arg(short, long, required = false)]
    name: Option<String>,
    #[arg(short, long, required = false)]
    password: Option<String>,
    #[arg(short, long, required = false)]
    email: Option<String>,
    #[arg(short, long, required = false)]
    telephone: Option<String>,
    #[arg(short, long, required = false)]
    url: Option<String>,
    #[arg(short, long, required = false)]
    web: Option<String>,
}

impl Cmd {
    pub fn get_cmd(&self) -> CmdType {
        self.cmd.unwrap_or(CmdType::None)
    }

    pub fn get_id(&self) -> i32 {
        match self.id {
            Some(c) => c as i32,
            None => -1
        }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_deref().unwrap_or("")
    }

    pub fn get_password(&self) -> &str {
        self.password.as_deref().unwrap_or("")
    }

    pub fn get_email(&self) -> &str {
        self.email.as_deref().unwrap_or("")
    }

    pub fn get_telephone(&self) -> &str {
        self.telephone.as_deref().unwrap_or("")
    }

    pub fn get_url(&self) -> &str {
        self.url.as_deref().unwrap_or("")
    }

    pub fn get_web(&self) -> &str {
        self.web.as_deref().unwrap_or("")
    }
    pub fn check_args(&self) -> bool {       
         match self.get_cmd() {
            //if the cmd type is to add new user, then check all args.
            CmdType::Add => {
             // if the cmd type is to add new user, then check all args.
             return self.get_name() != "" && self.get_password() != "" && self.get_email() != "" && self.get_telephone() != "" && self.get_url() != "" && self.get_web() != "";
            },
            CmdType::Del => {
             return self.get_id() > 0;
            },
            CmdType::Mod => {
             return self.get_id() > 0 && (self.get_name() != "" || self.get_password() != "" || self.get_email() != "" || self.get_telephone() != "" || self.get_url() != "" || self.get_web() != "");
            },
            CmdType::Show => {
             // if the cmd type is to show and the id is not set, then show all user.
             // other wise: just show the user with the id.
             return true;
            },
            CmdType::Help => {
             return true;
            },
            _ => {
             return false;
            },
        }
    }
}

pub fn get_cli() -> Cmd {
    Cmd::parse()
    //match Cmd::try_parse() {
    //    Ok(c) => c,
    //    Err(c) => panic!("{}", c),
    //}
}

// cmdType id name pwd web url phone email
//pub fn get_cmd() ->Cmd {
//
//}
