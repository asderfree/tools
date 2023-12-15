use clap::Parser;

#[derive(Parser, Debug)]
#[command(name="rtree", author, version, about, long_about=None)]
pub struct Cmd {
    #[arg(short, long, required = false)]
    path: Option<String>,
    #[arg(short, long, required = false)]
    depth: Option<u32>,
}

impl Cmd {
    pub fn get_path(&self) -> &str {
        self.path.as_deref().unwrap_or(".")
    }
    pub fn get_depth(&self) -> u32 {
        self.depth.unwrap_or(2)
    }
}

pub fn get_cli() -> Cmd {
    Cmd::parse()
}
