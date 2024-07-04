use std::fs::{self};
use std::os::unix::fs::PermissionsExt;
use chrono::DateTime;
use chrono::offset::Utc;

const FOLDER_ICON: char = '\u{1F4C1}';
const FILE_ICON: char = '\u{1F5CE}';

pub fn ls(){
    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        if let Ok(dir_entry) = path {
            let metadata: fs::Metadata = dir_entry.metadata().unwrap();

            // path creation date time
            let created: DateTime<Utc> = metadata.created().unwrap().into();
            let formatted_created = created.format("%d/%m/%Y %T"); // TODO: parse from utc to local

            // path name
            let name = &dir_entry.path().display().to_string()[2..];

            let path_type = if metadata.is_dir()  {FOLDER_ICON} else {FILE_ICON} ;

            // path owner

            //path permissions
            let permissions = get_permissions(metadata);

            println!("{}  {} {}  {}", path_type, permissions, formatted_created, name);
        }
    }
}

fn get_permissions(meta: fs::Metadata) -> String{
    let mode: u32 = meta.permissions().mode();
    let permissions = meta.permissions();
    let mut result = String::new();
    let file_type = mode & 0o170000; // top 4 bits represent the file type

    if file_type == 0o100000 {
        result.push(' ');
    }

    result.push(match file_type {
        0o100000 => '-', // regular file
        0o120000 => 'l', // symlink
        0o040000 => 'd', // directory
        0o060000 => 'b', // block device
        0o020000 => 'c', // character device
        0o10000 => 'p',  // named pipe (FIFO)
        0o140000 => 's', // socket
        _ => '?',        // unknown
    });

    // Owner permissions
    result.push(if permissions.mode() & 0o400 != 0 { 'r' } else { '-' });
    result.push(if permissions.mode() & 0o200 != 0 { 'w' } else { '-' });
    result.push(if permissions.mode() & 0o100 != 0 {
        if permissions.mode() & 0o4000 != 0 {
            's'
        } else {
            'x'
        }
    } else if permissions.mode() & 0o4000 != 0 {
        'S'
    } else {
        '-'
    });

    // Group permissions
    result.push(if permissions.mode() & 0o40 != 0 { 'r' } else { '-' });
    result.push(if permissions.mode() & 0o20 != 0 { 'w' } else { '-' });
    result.push(if permissions.mode() & 0o10 != 0 {
        if permissions.mode() & 0o2000 != 0 {
            's'
        } else {
            'x'
        }
    } else if permissions.mode() & 0o2000 != 0 {
        'S'
    } else {
        '-'
    });

    // Others permissions
    result.push(if permissions.mode() & 0o4 != 0 { 'r' } else { '-' });
    result.push(if permissions.mode() & 0o2 != 0 { 'w' } else { '-' });
    result.push(if permissions.mode() & 0o1 != 0 {
        if permissions.mode() & 0o1000 != 0 {
            't'
        } else {
            'x'
        }
    } else if permissions.mode() & 0o1000 != 0 {
        'T'
    } else {
        '-'
    });

    // TODO: does not work correctly add implementation for symlink
    /* 
    if meta.file_type().is_symlink() {
        result.push('@');
    } else {
        result.push(' ');
    }*/

    result
}
