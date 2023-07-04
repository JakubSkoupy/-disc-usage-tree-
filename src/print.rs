use std::os::unix::prelude::FileTypeExt;

use crate::options::Options;
use crate::options::Size;
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

#[derive(Clone)]
pub struct PrintOptions {
    end: bool,
    root: bool,
    root_size: Option<u64>,
    root_error: bool,
    depth: Option<u64>,
}

impl FileNode {
    fn print_error(&self, options: &Options) -> () {
        match self.error {
            true => {
                match options.colors {
                    true => print!("{}", "E".bright_red()),
                    false => print!("E"),
                };
            }
            false => print!(" "),
        }
    }

    fn print_name(&self, options: &Options) -> () {
        let mut colored_name = self.name.white();

        if options.colors {
            colored_name = match self.filetype {
                None => colored_name.bright_red().underline(),
                Some(x) if x.is_dir() => colored_name.bold().cyan(),
                Some(x) if x.is_symlink() => colored_name.bright_cyan(),
                Some(x) if x.is_char_device() || x.is_block_device() => colored_name.green(),
                Some(x) if x.is_fifo() => colored_name.bold().purple(),
                _ => colored_name,
            };
        }
        println!("{}", colored_name);
    }

    fn print_prefix(
        options: &Options,
        size_string: String,
        prefix: &Vec<&str>,
        print_options: &PrintOptions,
    ) -> () {
        let mut ext = "".to_string();
        if !print_options.root {
            ext = match print_options.end {
                true => format!("{}", END),
                false => format!("{}", BRANCH),
            };
        }

        match options.indent_size {
            true => print!(" {}{}  {:>8}", &prefix.join(""), ext, size_string),
            false => print!("{:>13}{}{}", size_string, &prefix.join(""), ext),
        }
    }

    fn print(&self, options: &Options, prefix: &Vec<&str>, print_options: &PrintOptions) -> () {
        if print_options.root_error {
            self.print_error(options);
        }

        let size_string = Self::size_string(options, self.size, print_options);
        FileNode::print_prefix(options, size_string, prefix, print_options);

        self.print_name(options);
    }

    pub fn print_subtree(
        &self,
        options: &Options,
        prefix: &mut Vec<&str>,
        mut print_options: PrintOptions,
    ) -> () {
        print_options.depth = match print_options.depth {
            None => None,
            Some(0) => return (),
            Some(x) => Some(x - 1),
        };

        if !print_options.root {
            self.print(&options, &prefix, &print_options);
            prefix.push(if print_options.end { SPACE } else { PIPE });
        }

        for (index, child) in self.children.iter().enumerate() {
            let end = index == self.children.len() - 1;

            let mut print_options = print_options.clone();
            print_options.end = end;
            print_options.root = false;

            child.print_subtree(&options, prefix, print_options);
        }

        prefix.pop();
    }

    fn size_string(options: &Options, mut size: u64, print_options: &PrintOptions) -> String {
        let (units, divisor) = options.units;
        let mut unit = 0;
        size *= 100;

        if let (Size::Blocks, _) = options.size {
            return format!("{:5} bl ", size / 100);
        };

        let mut postfix = units[unit];
        match options.size {
            // sizes in units
            (_, false) => {
                while size > (divisor * 100) {
                    size /= divisor;
                    unit += 1;
                    postfix = units[unit];
                }
            }

            // sizes in %
            (_, true) => {
                // this branch <=> root_size != None
                size = (100 * size) / print_options.root_size.unwrap();
                postfix = "%";
            }
        }
        format!("{:.2}", (size as f64) / 100.0) + &format!(" {}  ", postfix)
    }
}

impl FileTree {
    pub fn print(&self, options: Options, prefix: &mut Vec<&str>) -> () {
        match &self.root {
            None => return (),
            Some(root) => {
                let print_options = PrintOptions {
                    end: false,
                    root: true,
                    root_size: match options.size {
                        (_, true) => Some(root.size),
                        (_, false) => None,
                    },
                    depth: options.depth,
                    root_error: root.error,
                };
                root.print(&options, &prefix, &print_options);
                root.print_subtree(&options, prefix, print_options);
            }
        }
    }
}
