use rayon::prelude::*;

// use crate::Options;
use crate::{
        data_functions::{self, sort_items},
        options::Options,
        print::error_display,
};
use std::os::unix::fs::FileTypeExt;
use std::path::{Path, PathBuf};
use std::{
        fs::{read_dir, FileType},
        os::unix::prelude::MetadataExt,
};

pub struct FileNode {
        pub name: String,
        pub size: u64,
        pub children: Vec<FileNode>,
        pub error: bool,
        pub filetype: Option<FileType>,
}

fn directory_items<'a>(path: PathBuf) -> (Vec<PathBuf>, bool) {
        let mut items: Vec<PathBuf> = Vec::new();
        if !path.is_dir() {
                return (items, false);
        };

        let dir = match read_dir(path) {
                Err(_) => return (items, true),
                Ok(directory) => directory,
        };

        let mut error = false;

        for item in dir {
                let item = match item {
                        Ok(item) => item,
                        Err(_) => {
                                error = true;
                                continue;
                        }
                };

                let item_path = item.path();
                items.push(item_path);
        }
        (items, error)
}

fn parse_name(path: &PathBuf) -> String {
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

        fn build_subtree(path: PathBuf, options: &Options) -> Option<FileNode> {
                // FILE EXISTS
                if !path.exists() {
                        let error = format!("Invalid Path {}", path.display());
                        error_display(&error, options, 1);
                        return None;
                }

                if let Some(parent) = path.parent() {
                        if parent == Path::new("/proc") || parent == Path::new("/media") {
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
                if let Some(dev) = options.dev {
                        if metadata.dev() != dev {
                                return None;
                        }
                }

                // SUB TREES
                node.size = data_functions::file_size(&metadata, &options);

                let (mut items, error) = directory_items(path);
                node.error |= error;

                let children = items
                        .par_iter_mut()
                        .filter_map(|item| FileNode::build_subtree(item.clone(), options))
                        .collect::<Vec<_>>();

                node.children.reserve(items.len());
                for child in children {
                        node.size += child.size;
                        node.error = node.error || child.error;
                        node.children.push(child);
                }

                sort_items(&mut node.children, options);
                Some(node)
        }
}

pub struct FileTree {
        pub root: Option<FileNode>,
}

impl FileTree {
        pub fn build(path: PathBuf, options: &Options) -> Result<FileTree, std::io::Error> {
                let root = FileNode::build_subtree(path, options);
                Ok(FileTree { root })
        }
}
