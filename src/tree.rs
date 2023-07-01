use crate::Options;
use std::os::linux::fs::MetadataExt;
use std::path::Path;
use std::fs;



pub struct FileNode {
    pub name: String,
    pub size: u64,
    pub children: Vec<Box<FileNode>>,
}

impl FileNode {
    fn new(name: String) -> Self {
        FileNode {
            name,
            size: 0,
            children: Vec::new(),
        }
    }

    fn build_subtree(&mut self, options: Options) -> () {
        // DEPTH LIMIT
        let mut depth_limit = false;
        let mut depth = 0;

        // THIS IS FUCKING RETARDED AND UGLY
        if let Some(depth_) = options.depth {
            depth_limit = true;
            if depth_ == 0 {
                return ();
            }
            depth = depth_;
        }
        // UP TO HERE

        let path = self.name.clone();
        let dir = match fs::read_dir(path) {
            Err(_) => return (), // TODO ERROR TYPES
            Ok(dir_) => dir_,
        };

        for item in dir {
            let ( file_data, path ) = match item {
                Err(_) => continue,
                Ok(f) => match f.metadata() {
                    Err(_) => continue,
                    Ok(data) => ( data, f.path() ),
                },
            };

            let name = path.to_string_lossy();
            println!("file: {}", name);
            let mut file_node = Box::new(FileNode::new(name.to_string()));
            file_node.size = file_data.st_blksize();
            if file_data.is_dir() && !file_data.is_symlink() {
                // DIRECTORY
                let mut subtree_options = options.clone();
                subtree_options.depth = if depth_limit {Some(depth - 1)} else {None};
                file_node.build_subtree(subtree_options);
                self.size += file_node.size;
            }
            self.children.push(file_node);
        }
        // self.children.sort_by()
    }
}

pub struct FileTree {
    pub root: FileNode,
}

impl FileTree {
    pub fn new() -> Self {
        let root = FileNode::new("/".to_string());
        FileTree { root }
    }

    pub fn build(directory_path: &Path, options: Options) -> Self {
        let mut tree = FileTree::new();
        let data = directory_path.metadata();
        let data = match data {
            Err(_) => return tree,
            Ok(d) => d,
        };

        if !data.is_dir() {
            return tree;
        }

        tree.root.build_subtree(options);
        tree
    }
}
