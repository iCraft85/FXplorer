use chrono::{DateTime, Datelike, Timelike, TimeZone, Utc};
use std::fs;
use std::os::unix::prelude::MetadataExt;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use std::os::unix::prelude::PermissionsExt;
use std::io;
use std::env;

pub struct DirItem {
    pub name: String,
    pub path: PathBuf,
    pub perm: String,
}

pub fn getContents(dir: &PathBuf) -> io::Result<Vec<DirItem>> {
    let mut entries: Vec<DirItem> = vec![];

    for entry in fs::read_dir(dir)?{
        match entry {
            Ok(entry) => {
                let mut dirItem: DirItem;

                let mut name = entry.file_name().to_string_lossy().to_string();
                if entry.path().is_dir() {
                    name.push('/');
                }

                if let Ok(meta) = entry.path().metadata() {
                    let mode = meta.permissions().mode();
                    dirItem = DirItem {
                        name: name,
                        path: entry.path(),
                        perm: getPerm(mode)
                    };
                }
                else {
                    dirItem = DirItem {
                        name: name,
                        path: entry.path(),
                        perm: "ERR".to_owned(),
                    };
                }

                entries.push(dirItem);
            },
            Err(err) => eprintln!("Error: {}", err),

        }
    }

    // entries.sort();

    Ok(entries)
}

struct permissions {
    // Owner Permissions 
    ownerRead: bool,
    ownerWrite: bool,
    ownerExecute: bool,

    // Group Permissions
    groupRead: bool,
    groupWrite: bool,
    groupExecute: bool,
    
    // Other Permissions
    otherRead: bool,
    otherWrite: bool,
    otherExecute: bool,
}

pub fn getPerm(bits: u32) -> String {
    let mut s: String = String::new();

    rwx(&mut s, bits >> 6);
    rwx(&mut s, bits >> 3);
    rwx(&mut s, bits);

    return s;
}

fn rwx(s: &mut String, bits: u32) {
    s.push(if bits & 0b100 != 0 { 'r' } else { '-' });
    s.push(if bits & 0b010 != 0 { 'w' } else { '-' });
    s.push(if bits & 0b001 != 0 { 'x' } else { '-' });
}

pub fn getSize(path: &PathBuf) -> Result<String, std::io::Error>{
    let units = ["B", "kB", "MB", "GB", "TB", "PB"];
    let mut index = 0;

    let metadata = path.metadata()?;
    let mut size = metadata.len() as f64;
    
    while size > 1024.0 && index < units.len() - 1{
       size /= 1024.0; 
       index += 1;
    }

    size = (size * 100.0).round() / 100.0;

    if size.fract() == 0.0 {
        return Ok(format!("{:.0} {}", size, units[index]))
    }
    else {
        return Ok(format!("{:.2} {}", size, units[index]))
    }
}

pub fn getModified(path: &PathBuf) -> Result<String, std::io::Error>{
    let metadata = path.metadata()?;

    if let Ok(time) = metadata.modified() {
        return Ok(formatTime(time));
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Not supported on this platform",
        ));
    }
}

fn formatTime(system_time: SystemTime) -> String {
    let dt: DateTime<Utc> = system_time.into();
    let weekday = dt.format("%a").to_string();
    let month = dt.format("%b").to_string();
    let day = dt.day();
    let hour = dt.hour();
    let minute = dt.minute();

    format!("{} {} {:02} {:02}:{:02}", weekday, month, day, hour, minute)
}


// fn formatTime(time: SystemTime) -> String {
//
// }
