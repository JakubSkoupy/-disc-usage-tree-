use std::os::unix::prelude::FileTypeExt;

use crate::options::Options;
use crate::tree::FileNode;
use crate::tree::FileTree;
use colored::Colorize;

const BRANCH: &str = "├── ";
const END: &str = "└── ";
const PIPE: &str = "│   ";
const SPACE: &str = "    ";

pub const UNITS: [&str; 7] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB"];
pub const UNITS_DEC: [&str; 7] = ["B", "KB", "MB", "GB", "TB", "PB", "EB"];

pub const DIVISOR: u64 = 1024;
pub const DIVISOR_DEC: u64 = 1000;

pub fn error_display(message: &str, options: &Options, verbosity: u8) {
    if options.verbosity >= verbosity {
        eprintln!("{}", message);
    }
}

impl FileNode {
    fn print(&self, options: &Options, prefix: &Vec<&str>, flags: (bool, bool)) -> () {
        let (end, root) = flags;

        let size_string = FileNode::size_string(options, self.size);
        print!("{:>13}{}", size_string, &prefix.join(""));

        if !root {
            if end {
                print!("{}", END)
            } else {
                print!("{}", BRANCH)
            };
        }

        let mut colored_name = self.name.white();
        colored_name = match self.filetype {
            None => colored_name.bright_red().underline(),
            Some(x) if x.is_dir() => colored_name.bold().cyan(),
            Some(x) if x.is_symlink() => colored_name.bright_cyan(),
            Some(x) if x.is_char_device() || x.is_block_device() => colored_name.green(),
            Some(x) if x.is_fifo() => colored_name.bold().purple(),
            _ => colored_name,
        };

        println!("{}", colored_name);
    }

    pub fn print_subtree(
        &self,
        options: &Options,
        prefix: &mut Vec<&str>,
        flags: (bool, bool, Option<u64>),
    ) -> () {
        let (end, root, depth) = flags;
        let depth = match depth {
            None => None,
            Some(0) => return (),
            Some(x) => Some(x - 1),
        };

        if !root {
            self.print(&options, &prefix, (end, root));
            prefix.push(if end { SPACE } else { PIPE });
        }

        for (index, child) in self.children.iter().enumerate() {
            let end = index == self.children.len() - 1;
            child.print_subtree(&options, prefix, (end, false, depth));
        }

        prefix.pop();
    }

    fn size_string(options: &Options, mut size: u64) -> String {
        let (units, divisor) = options.units;
        let mut unit = 0;
        size *= 100;

        while size > divisor * 100 {
            size /= divisor;
            unit += 1;
        }
        format!("{:.2}", (size as f64) / 100.0) + &format!(" {}  ", units[unit])
    }
}

impl FileTree {
    pub fn print(&self, options: Options, prefix: &mut Vec<&str>) -> () {
        match &self.root {
            None => return (),
            Some(root) => {
                root.print(&options, &prefix, (false, true));
                root.print_subtree(&options, prefix, (false, true, options.depth));
            }
        }
    }
}
