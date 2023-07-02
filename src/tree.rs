// use crate::Options;
use std::fs::read_dir;
use std::os::unix::fs::FileTypeExt;
use std::os::linux::fs::MetadataExt;
use crate::Options;
use std::path::{Path, PathBuf};

pub struct FileNode {
    pub name: String,
    pub size: u64,
    pub children: Vec<Box<FileNode>>,
}

fn directory_items<'a>(
    path: Box<PathBuf>,
    items: &'a mut Vec<Box<PathBuf>>,
) -> Result<(), std::io::Error> {
    // NOT A DIRECTORY
    if !path.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Not a directory",
        ));
    }

    let dir = read_dir(*path)?;
    for item in dir {
        let item = item?;
        let item_path = Box::new(item.path());
        items.push(item_path);
    }
    Ok(())
}

impl FileNode {
    fn build_subtree(path: Box<PathBuf>, options: &Options) -> Option<FileNode> {
        // FILE EXISTS
        if !path.exists() {
            // eprintln!("Invalid Path {}", path.display());
            return None;
        }

        if let Some(parent) = path.parent() {
            if parent == Path::new("/proc"){
                return None;
            }
        }

        // FILE NAME
        let path_string = path.to_string_lossy();
        let name = match path_string.rsplit_once('/') {
            None => &path_string,
            Some((_, name)) => name,
        };

        // NODE
        let mut node = FileNode {
            name: name.to_string(),
            size: 0,
            children: Vec::new(),
        };

        // METADATA
        let metadata = match path.metadata() {
            Err(_) => {
                return Some(node);
            }
            Ok(data) => data,
        };

        // DIRECTORY FILES
        let ftype = metadata.file_type();
        if ftype.is_char_device() || path.is_symlink() || ftype.is_fifo() || ftype.is_socket() || ftype.is_block_device()  {
            return Some(node);
        }

        node.size = if options.block_size {metadata.st_blksize()} else {metadata.st_size()};
        let mut items = Vec::new();
        if let Err(_) = directory_items(path.clone(), &mut items) {
        } // TODO VOID RETURN

        // SUB TREES
        for item in items.iter() {
            let child = FileNode::build_subtree(item.clone(), options);
            let child = match child {
                Some(ch) => Box::new(ch),
                None => continue,
            };

            node.size += child.size;
            node.children.push(child);
        }
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
