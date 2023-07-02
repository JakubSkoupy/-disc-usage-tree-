// DISC USAGE TREE
use clap::{Parser, };
use tree::FileTree;
use std::path;
mod print;
mod tree;

#[derive(Parser)]
#[clap(version = "1.0", author = "Jakub Skoupy")]
#[derive(Clone)]
pub struct Options {
    #[clap(short = 'b')]
    block_size: bool,

    #[clap(short = 'p')]
    percent: bool,

    #[clap(short = 'n')]
    name_sort: bool,

    #[clap(short = 'd', default_value = "false")]
    device: bool,

    #[clap(short = 'l', long = "depth")]
    depth: Option<u32>,

    #[clap(long = "decimal")]
    decimal: bool,

    #[clap()]
    path: String,
}

fn main() {
    let options = Options::parse();
    let _opt_depthless = options.clone();

    let path = path::Path::new(&options.path);
    let mut pathbuf = path::PathBuf::new();
    pathbuf.push(path);

    let tree = FileTree::build(&pathbuf, &options);
    let mut _prefix: Vec<&str> = Vec::new();
    match tree {
        Err(_) => eprintln!("Mas to napicu"),
        Ok(_tree) => _tree.print(options, &mut _prefix),
    }
}
