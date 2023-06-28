// DISC USAGE TREE
use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Jakub Skoupy")]

struct Options {
    #[clap(short = 'b')]
    block_size: bool,

    #[clap(short='p')]
    percent: bool,

    #[clap(short='n')]
    name_sort: bool,

    #[clap(short='d', default_value = "false")]
    device: bool,

    #[clap(short = 'l', long = "depth", default_value = "None")]
    depth: Option<u32>,
}


fn main() {
    let parse_options: Result<Options, clap::Error> = Ok(Options::parse());
    let options: Options;

    match parse_options {

        Ok(opts) => {
            match opts.depth {
                None => println!("No depth specified"),
                Some(num) => println!("Depth: {}", num),
            }
            if opts.block_size {
                println!("Using block size");
            }
        }

        Err(_) => {
            eprintln!("Ses asi uplne posral ne?! \
            nemuzes si proste vymyslet ze chces hloubkove omezeni \
            a pak nerict jake")
        }
    }
}
