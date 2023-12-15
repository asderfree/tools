use std::fs;
use std::path::{Path, PathBuf};
mod cmd;

// print_dir will just print the tree of the dir: path, this is a recursive function.
// @param path: the path the function will show.
// @param prefix: the prefix of the father dir.
// @param last: if the dir or file is the last file in the parent dir.
// @param count: to record how much files has show.
// @param max_depth: the max depth to recursive. which will represent the depth of the tree.
fn print_dir(
    path: &Path,
    prefix: &str,
    last: bool,
    count: &mut u64,
    max_depth: u32,
) -> std::io::Result<()> {
    let stem = if last { "└── " } else { "├── " };
    let content_prefix = if last { "    " } else { "│   " };
    if path.is_dir() {
        let mut entries: Vec<PathBuf> = fs::read_dir(path)?
            .filter_map(|e| e.ok().map(|e| e.path()))
            .collect();
        entries.sort();
        for (i, e) in entries.iter().enumerate() {
            let last_in_group = i == entries.len() - 1;
            println!(
                "{}{}{}",
                prefix,
                stem,
                e.file_name().unwrap().to_string_lossy()
            );
            if e.is_dir() && max_depth > 1 {
                print_dir(
                    e,
                    &format!("{}{}", prefix, content_prefix),
                    last_in_group,
                    count,
                    max_depth - 1,
                )?;
            }
            *count += 1;
        }
    }
    Ok(())
}

// check if the src is match the ma.
//fn check_match(ma: &str, src: &str) -> bool {
//    println!("the match is: {}, and the src is: {}", ma, src);
//    false
//}

fn main() {
    let c = cmd::get_cli();
    // println!("{:?}", c);

    let mut count: u64 = 0;
    let root = c.get_path();
    let path = Path::new(root);
    if path.is_dir() {
        println!("{}", path.display());
        print_dir(&path, "", true, &mut count, c.get_depth())
            .unwrap_or_else(|e| eprintln!("Error: {}", e));
        println!(
            "total {} files found in dir {}",
            count,
            path.display().to_string()
        );
    } else {
        eprintln!("Provided path is not a directory.");
    }
}
