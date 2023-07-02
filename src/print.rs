use crate::tree::FileNode;
use crate::tree::FileTree;
use crate::Options;

const BRANCH: &str = "├── ";
const END: &str = "└── ";
const PIPE: &str = "│   ";
const SPACE: &str = "    ";

const UNITS: [&str; 7] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB"];
const UNITS_DEC: [&str; 7] = ["B", "KB", "MB", "GB", "TB", "PB", "EB"];

const DIVISOR: u64 = 1024;
const DIVISOR_DEC: u64 = 1000;

impl FileNode {
    fn print(&self, options: &Options, prefix: &Vec<&str>, flags: (bool, bool)) -> () {
        let (end, root) = flags;
        FileNode::print_size(options, self.size);
        print!("{}", prefix.join(""));

        if !root {
            if end {
                print!("{}", END)
            } else {
                print!("{}", BRANCH)
            };
        }
        println!("{}", self.name);
    }

    pub fn print_subtree(
        &self,
        options: Options,
        prefix: &mut Vec<&str>,
        flags: (bool, bool),
    ) -> () {
        let mut depth_limit = false;
        let mut depth = 0;

        // THIS IS FUCKING RETARDED AND UGLY
        if let Some(depth_) = options.depth {
            depth_limit = true;
            if depth_ <= 0 {
                return ();
            }
            depth = depth_;
        }

        let (end, root) = flags;

        if !root {
            &self.print(&options, &prefix, flags);
            prefix.push(if end { SPACE } else { PIPE });
        }

        let children = self.children.len();
        for (index, child) in self.children.iter().enumerate() {
            let mut options_next = options.clone();
            if depth_limit {
                options_next.depth = Some(depth - 1);
            }

            let end = index == children - 1;
            child.print_subtree(options_next, prefix, (end, false));
        }

        prefix.pop();
    }

    fn print_size(options: &Options, mut size: u64) {
        let mut unit = 0;
        while size > DIVISOR {
            size /= DIVISOR;
            unit += 1;
        }
        print!("{:1$}", size, 4);
        print!(" {:>1$}  ", UNITS[unit], 3);
    }
}

impl FileTree {
    pub fn print(&self, options: Options, prefix: &mut Vec<&str>) -> () {
        match &self.root {
            None => return (),
            Some(root) => {
                root.print(&options, &prefix, (false, true));
                root.print_subtree(options, prefix, (false, true));
            }
        }
    }
}
