use std::path::PathBuf;

use crate::fileUtils::*;
use crate::tUtils::*;

pub fn openDir(selected: Option<usize>, dirList: &mut DirList) {
   
    if let Some(selected) = dirList.items.state.selected() {
        let item = dirList.items.items.get(selected).unwrap().clone();

        if !item.path.is_dir() {
            return;
        }

        if let Ok(contents) = getContents(&item.path){
            if let Some(parent) = item.path.parent() {
                *dirList = DirList::new(parent.to_path_buf(), contents);
            }
        }
    }
}

pub fn exitDir(dirList: &mut DirList) {
    let contents = getContents(&dirList.parent);

    if let Ok(contents) = getContents(&dirList.parent){
        if let Some(parent) = dirList.parent.parent() {
            *dirList = DirList::new(parent.to_path_buf(), contents);
        }
    }
}

use std::fs::File;
use std::io::Write;

fn debugOut(debugDat: String) {
    let mut debug = File::create("debug.txt").expect("creation failed");
    debug.write(debugDat.as_bytes()).expect("write failed");
}
