use crate::options::{Options, Size};
use crate::tree::FileNode;
use std::cmp;
use std::fs::Metadata;
use std::os::linux::fs::MetadataExt;

pub fn compare_size(item_b: &FileNode, item_a: &FileNode) -> cmp::Ordering {
        // REVERSED FOR DESCENDING ORDER
        item_a.size.cmp(&item_b.size)
}

pub fn compare_name(item_a: &FileNode, item_b: &FileNode) -> cmp::Ordering {
        item_a.name.cmp(&item_b.name)
}

pub fn file_size(file: &Metadata, options: &Options) -> u64 {
        match options.size {
                (Size::BlockSize, _) => return file.st_blksize(),
                (Size::Length, _) => return file.st_size(),
                (Size::Blocks, _) => return file.st_blocks(),
        }
}

pub fn sort_items(items: &mut Vec<FileNode>, options: &Options) -> () {
        match options.sort {
                None => return,
                Some(comparison_function) => items.sort_by(comparison_function),
        }
}
