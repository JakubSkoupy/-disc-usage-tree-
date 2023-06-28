use clap:: {Args, Parser, Subcommand};

struct Options {
    block_size: bool,
    percent: bool,
    name_sort: bool,
    device: bool,
    depth: u32,
}

