extern crate clap;
extern crate zip;
extern crate regex;

mod zip_reader;
use zip_reader::{display_zip_file, read_zip, get_index};

mod cli;
use cli::{Mode,cli,claptrap,match_regex};

use zip::read::ZipArchive;

use std::io::prelude::*;
use std::io;
use std::fs::{OpenOptions,create_dir_all};
use std::path::{Path,PathBuf};


fn main() {
    //get args
    let args = claptrap();
    //get cli args
    let mode = cli(&args);

    match mode {
        Mode::View(x) => {
            //get path to archive
            let archive = x;
            //get zip reader
            let mut reader = match OpenOptions::new()
                .write(false).read(true).open(x) {
                Ok(x) => match ZipArchive::new(x) {
                    Ok(y) => y,
                    Err(e) => panic!("\n\nCould not open file\n{}\n{:?}",&archive,e)
                },
                Err(e) => panic!("\n\nCould not open file\n{}\n{:?}",&archive,e)
            };
            //loop over all files
            for i in 0..reader.len() {
                let rv = get_index(&mut reader, i);
                display_zip_file(&rv);
            }
        },
        Mode::Extract(x) => {
            //allocate a 2MiB page to extract into
            let mut v = Vec::<u8>::with_capacity(2097152);
            //get path to archive
            let mut base_path = PathBuf::new();
            base_path.push(x);
            //get zip reader
            let mut reader = match OpenOptions::new()
                .write(false).read(true).open(x) {
                Ok(y) => match ZipArchive::new(y) {
                    Ok(z) => z,
                    Err(e) => panic!("\n\nCould not open file\n{}\n{:?}",x,e)
                },
                Err(e) => panic!("\n\nCould not open file\n{}\n{:?}",x,e)
            };
            //loop over all files
            for i in 0..reader.len() {
                //get the zip file we're working with
                let mut z = get_index(&mut reader, i);
                //get output path
                let mut p = base_path.clone();
                p.push(Path::new(z.name()));
                //ignore absolute paths
                if ! p.is_relative() {
                    continue;
                }
                //get the file's name (skip invalid names)
                let file_name = match p.file_name() {
                    Option::None => continue,
                    Option::Some(x) => match x.to_str() {
                        Option::None => continue,
                        Option::Some(y) => y
                    }
                };
                //check if file name matches the regex
                if match_regex(file_name) {
                    match p.parent() {
                        Option::None => continue,
                        Option::Some(x) => match create_dir_all(x) {
                            Ok(_) => { },
                            Err(e) => panic!("\n\nCould not unzip. Err while making file\n{}\n{:?}\n\n",z.name(),e)
                        }
                    };
                    let mut f = match OpenOptions::new()
                        .write(true).read(false).create(true).open(&p) {
                        Ok(x) => x,
                        Err(e) => panic!("\n\nCould not unzip. Err while making file\n{}\n{:?}\n\n",z.name(),e)
                    };
                    match f.set_len(z.size()) {
                        Ok(_) => { },
                        Err(e) => panic!("\n\nCould not unzip. Error while making file\n{}\n{:?}\n\n",z.name(),e)
                    };
                    read_zip(&mut z, &mut v);
                    match f.write_all(v.as_slice()) {
                        Ok(_) => { },
                        Err(e) => panic!("\n\nCould not unzip. Error while writing file\n{}\n{:?}\n\n",z.name(),e)
                    };
                }
            }
        },
        Mode::Create(_) => { },
    };        
}
