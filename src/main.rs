// DISC USAGE TREE
use clap::Parser;
use tree::FileTree;
mod tree;
mod print;

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
    let path = std::path::Path::new("/home");

    let opt_depthless = options.clone();

    let tree = FileTree::build(&path, opt_depthless);
    let mut prefix = Vec::new();
    tree.root.print(options, &mut prefix);
}
