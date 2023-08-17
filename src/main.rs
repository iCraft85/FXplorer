#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused)]

use clap::Parser;
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

mod tui;
mod fileUtils;
mod tUtils;
mod tNav;

use fileUtils::*;

// TODO: Handle Symbols For file types

#[derive(Parser, Debug)]
#[command(name = "fx", author, version, about, long_about = None)]
pub struct Args {
    #[clap(value_name="DirPath", value_delimiter=' ')]
    dir: Option<PathBuf>,
}

fn getDirectory(args: &Args) -> Result<PathBuf, io::Error>{
    // Check to see if dir was parsed into command
    if let Some(dir) = &args.dir {
        if dir.exists(){
            Ok(dir.clone())
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Specified directory does not exist: {:?}", dir),
            ))
        }
    } else {
        Ok(env::current_dir().unwrap())
    }
}

fn main() {
    let args = Args::parse();
    let dir = getDirectory(&args);

    match dir {
        Ok(dir) => {
            let contents = fileUtils::getContents(&dir).unwrap();

            // TODO: Handle not having a parent

            let mut dirList: tUtils::DirList;
            if let Some(parent) = dir.parent() {
                dirList = tUtils::DirList::new(parent.to_path_buf(), contents);
            }
            else {
                dirList = tUtils::DirList::new(dir, contents);
            }
            tui::uiMain(dirList);
        },
        Err(err) => eprintln!("Error: {}", err),
    }
}

