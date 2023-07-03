// use crate::Options;
use crate::{
    data_functions::{self, sort_items},
    options::Options,
    print::error_display,
};
use std::fs::{read_dir, FileType};
use std::os::unix::fs::FileTypeExt;
use std::path::{Path, PathBuf};

pub struct FileNode {
    pub name: String,
    pub size: u64,
    pub children: Vec<Box<FileNode>>,
    pub error: bool,
    pub filetype: Option<FileType>,
}

fn directory_items<'a>(path: Box<PathBuf>) -> Vec<Box<PathBuf>> {
    let mut items: Vec<Box<PathBuf>> = Vec::new();
    if !path.is_dir() {
        return items;
    };

    let dir = match read_dir(*path) {
        Err(_) => return items,
        Ok(directory) => directory,
    };

    for item in dir {
        let item = item.unwrap();
        let item_path = Box::new(item.path());
        items.push(item_path);
    }
    items
}

fn parse_name(path: &Box<PathBuf>) -> String {
    let string = path.to_string_lossy();
    match string.clone().rsplit_once('/') {
        None => string.to_string(),
        Some((_, name)) => name.to_string(),
    }
}

impl FileNode {
    fn new(name: String) -> Self {
        FileNode {
            name: name.to_string(),
            size: 0,
            children: Vec::new(),
            error: false,
            filetype: None,
        }
    }

    fn build_subtree(path: Box<PathBuf>, options: &Options) -> Option<FileNode> {
        // FILE EXISTS
        if !path.exists() {
            let error = format!("Invalid Path {}", path.display());
            error_display(&error, options, 1);
            return None;
        }

        if let Some(parent) = path.parent() {
            if parent == Path::new("/proc") {
                return None;
            }
        };
        let mut node = FileNode::new(parse_name(&path));

        // METADATA
        let metadata = match path.metadata() {
            Err(_) => return Some(node),
            Ok(data) => data,
        };
        let ftype = metadata.file_type();
        node.filetype = Some(ftype);
        if ftype.is_char_device() || path.is_symlink() {
            return Some(node);
        };

        // SUB TREES
        node.size = data_functions::file_size(&metadata, &options);
        let items = directory_items(path);

        for item in items.iter() {
            if let Some(child) = FileNode::build_subtree(item.clone(), options) {
                node.size += child.size;
                node.children.push(Box::new(child));
            }
        }
        sort_items(&mut node.children, options);
        Some(node)
    }
}

pub struct FileTree {
    pub root: Option<FileNode>,
}

impl FileTree {
    pub fn build(path: &PathBuf, options: &Options) -> Result<FileTree, std::io::Error> {
        let path = Box::new(path.clone());

        let root = FileNode::build_subtree(path, options);
        Ok(FileTree { root })
    }
}
