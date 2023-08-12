#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused)]

use clap::Parser;
use std::env;
use std::fs;
use std::io;
use std::os::unix::prelude::PermissionsExt;
use std::path::{Path, PathBuf};

mod tui;
mod permissions;
mod tUtils;


// TODO: Handle Symbols For file types

#[derive(Parser, Debug)]
#[command(name = "fx", author, version, about, long_about = None)]
struct Args {
    #[clap(value_name="DirPath", value_delimiter=' ')]
    dir: Option<PathBuf>,
}

pub struct DirItem {
    name: String,
    path: PathBuf,
    perm: String,
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

fn getContents(dir: &PathBuf) -> io::Result<Vec<DirItem>> {
    let mut entries: Vec<DirItem> = vec![];

    for entry in fs::read_dir(dir)?{
        match entry {
            Ok(entry) => {
                let mut mode = entry.path().metadata().unwrap().permissions().mode();

                let mut name = entry.file_name().to_string_lossy().to_string();
                if entry.path().is_dir() {
                    name.push('/');
                }

                let mut dirItem = DirItem {
                    name: name,
                    path: entry.path(),
                    perm: permissions::mode(mode)
                };

                // println!("Name: {:?} Perms: {:?}", dirItem.name, dirItem.perm);
                entries.push(dirItem);
            },
            Err(err) => eprintln!("Error: {}", err),

        }
    }

    // entries.sort();

    Ok(entries)
}


fn main() {
    let args = Args::parse();
    let dir = getDirectory(&args);

    match dir {
        Ok(dir) => {
            let contents = getContents(&dir).unwrap();
            // for item in contents{
            //     let meta = fs::metadata(item).unwrap().permissions();
            //     println!("{:?}", meta)
            // }

            let dirList = tUtils::DirList::new(contents);
            tui::uiMain(dirList);
        },
        Err(err) => eprintln!("Error: {}", err),
    }
}






