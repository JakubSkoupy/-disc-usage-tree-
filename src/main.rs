// DISC USAGE TREE
use clap::Parser;
use std::path;
use tree::FileTree;

use crate::print::PrintOptions;
mod data_functions;
mod options;
mod print;
mod tree;

fn main() {
        let parsing_options = options::ParsingOptions::parse();
        let mut options = options::Options::compile(&parsing_options);

        let mut pathbuf = path::PathBuf::new();
        let path = path::Path::new(&parsing_options.path);
        pathbuf.push(path);

        let tree = FileTree::build(pathbuf, &options);
        let mut _prefix: Vec<&str> = Vec::new();

        if let Ok(filetree) = tree {
                filetree.print(options.clone(), &mut _prefix);
                let opts = PrintOptions {
                        end: false,
                        root: true,
                        root_size: None,
                        root_error: false,
                        depth: None,
                };
                let root = filetree.root.unwrap();
                let (x, _) = options.size;
                options.size = (x, false);
                let size_string = tree::FileNode::size_string(&options, root.size, &opts);
                print!("\nTOTAL SIZE: [ {} ] ", size_string);
        }
}
