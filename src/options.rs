use std::os::unix::prelude::MetadataExt;
use std::path::Path;

use crate::tree::FileNode;
use crate::{data_functions, print};
use clap::Parser;

#[derive(PartialEq, Clone)]
pub enum Size {
        Length,
        BlockSize,
        Blocks,
}

#[derive(Parser)]
#[clap(version = "1.0", author = "Jakub Skoupy")]
pub struct ParsingOptions {
        #[clap(short = 'b', long = "blocksize")]
        block_size: bool,

        #[clap(long = "blocks")]
        blocks: bool,

        #[clap(short = 'v', long = "verbose")]
        verbose: bool,

        #[clap(short = 'i', long = "indent")]
        indent: bool,

        #[clap(short = 'q', long = "quiet")]
        quiet: bool,

        #[clap(short = 'p')]
        percent: bool,

        #[clap(short = 'l', long = "depth")]
        depth: Option<u64>,

        #[clap(short = 's', long = "sort")]
        sort: Option<String>,

        #[clap(long = "decimal")]
        decimal: bool,

        #[clap(short = 'd', long = "device")]
        dev: bool,

        #[clap(short = 'c', long = "nocolor")]
        nocolor: bool,

        #[clap()]
        pub path: String,
}

#[derive(Clone)]
pub struct Options {
        pub size: (Size, bool),
        pub verbosity: u8,
        pub sort: Option<fn(&FileNode, &FileNode) -> std::cmp::Ordering>,
        pub depth: Option<u64>,
        pub units: (&'static [&'static str; 7], u64),
        pub indent_size: bool,
        pub colors: bool,
        pub dev: Option<u64>,
}

impl Options {
        fn default() -> Self {
                Options {
                        size: (Size::Length, false),
                        verbosity: 1,
                        sort: None,
                        depth: None,
                        units: (&print::UNITS, print::DIVISOR),
                        indent_size: false,
                        colors: true,
                        dev: None,
                }
        }

        pub fn compile(input: &ParsingOptions) -> Self {
                let mut options = Options::default();
                if input.dev {
                        let path = Path::new(&input.path);
                        options.dev = Some(path.metadata().unwrap().dev());
                }

                // size
                if input.percent {
                        let (size, _) = options.size;
                        options.size = (size, true);
                }

                let (mut size, percent) = options.size;
                if input.block_size {
                        size = Size::BlockSize
                };
                if input.blocks {
                        size = Size::Blocks
                };
                options.size = (size, percent);

                if let Some(sort_method) = &input.sort {
                        options.sort = match sort_method.as_str() {
                                "name" => Some(data_functions::compare_name),
                                "size" => Some(data_functions::compare_size),
                                _ => None,
                        }
                }

                // verbosity
                if input.verbose {
                        options.verbosity = 2
                };
                if input.quiet {
                        options.verbosity = 0
                };

                // display
                if input.decimal {
                        options.units = (&print::UNITS_DEC, print::DIVISOR_DEC)
                };
                options.colors = !input.nocolor;

                // depth
                options.depth = match input.depth {
                        None => None,
                        Some(depth) => Some(depth + 1),
                };

                options.indent_size = input.indent;

                options
        }
}
