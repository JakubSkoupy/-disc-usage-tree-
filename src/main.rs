// DISC USAGE TREE
use clap::Parser;
use std::path;
use tree::FileTree;
mod data_functions;
mod options;
mod print;
mod tree;

fn main() {
    let parsing_options = options::ParsingOptions::parse();
    let options = options::Options::compile(&parsing_options);

    let mut pathbuf = path::PathBuf::new();
    let path = path::Path::new(&parsing_options.path);
    pathbuf.push(path);

    let tree = FileTree::build(&pathbuf, &options);
    let mut _prefix: Vec<&str> = Vec::new();

    if let Ok(filetree) = tree {
        filetree.print(options, &mut _prefix);
    }
}
