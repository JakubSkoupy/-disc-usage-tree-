// DISC USAGE TREE
use clap::Parser;
use tree::FileTree;
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
}

fn main() {
    let options = Options::parse();
    let _opt_depthless = options.clone();

    let tree = FileTree::build("/".to_string());
    let mut _prefix: Vec<&str> = Vec::new();
    match tree {
        Err(_) => eprintln!("Mas to napicu"),
        Ok(_tree) => _tree.print(options, &mut _prefix),
    }
}
