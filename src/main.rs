use clap::{command, Command};
use std::env;
use std::path::PathBuf;
use std::path::Path;
use std::sync::Arc;
use std::fs::read_dir;
use std::collections::HashMap;
use env::current_dir;
use std::fs::ReadDir;
use std::error::Error;
use std::fs::DirEntry;
use std::sync::Mutex;
use lazy_static::lazy_static;
use std::string::String;
use std::fs;
use std::fmt::Display;
use ptree::*;
use std::borrow::Cow;

#[derive(Clone)]  
struct FileNode {
    name: String,
    children: Vec<FileNode>,
}

impl FileNode {
    fn new(name: String) -> Self {
        FileNode {
            name,
            children: Vec::new(),
        }
    }

    fn add_child(&mut self, child: FileNode) {
        self.children.push(child);
    }
}

impl TreeItem for FileNode {
    type Child = Self;

    fn write_self<W: std::io::Write>(&self, f: &mut W, style: &Style) -> std::io::Result<()> {
        write!(f, "{}", style.paint(&self.name))
    }

    fn children(&self) -> Cow<[Self::Child]> {
        Cow::Owned(self.children.clone())
    }
}

fn main() {
    let currentdir: String = current_dir().expect("could not read current dir")
                                    .into_os_string()
                                    .into_string()
                                    .expect("could not convert osstring into string");
    
    let mut root = FileNode::new(currentdir.clone());
    
    parsedir(currentdir.clone(), &mut root).expect("Failed to parse directory");
    print_tree(&root).expect("couldn't print tree :(");
    
    println!("{}", currentdir);
}

fn parsedir(dir: String, parent_node: &mut FileNode) -> Result<(), Box<dyn std::error::Error>> {
    let entries: Vec<DirEntry> = read_dir(dir)?
        .filter_map(|result| result.ok())
        .filter(|e| !is_hidden(e))
        .collect();
    
    for entry_result in entries {
        let entry_path = entry_result.path();
        let entry_path_string = get_path_name(&entry_result);
        let filename = get_file_name(&entry_result);
        
        let mut new_node = FileNode::new(filename.clone());
        
        if entry_path.is_dir() && !is_hidden(&entry_result) {
            parsedir(entry_path_string.clone(), &mut new_node)?;
        }
        
        parent_node.add_child(new_node);
    }
    
    Ok(())
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn get_path_name(entry: &DirEntry) -> String {
    entry
        .path()
        .to_str()
        .unwrap_or_default()
        .to_string()
}

fn get_file_name(entry: &DirEntry) -> String {
    entry
        .file_name()
        .to_str()
        .unwrap_or_default()
        .to_string()
}
