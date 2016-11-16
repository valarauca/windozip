extern crate clap;
extern crate zip;
extern crate regex;

mod zip_reader;
use zip_reader::{display_zip_file, read_zip};

mod cli;
use cli::{Mode,cli,claptrap};

use zip::read::ZipArchive;

use std::io::prelude::*;
use std::io;
use std::fs::OpenOptions;

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
                match reader.by_index(i) {
                    Err(e) => println!("Could not open index {:?} {:?}",i,e),
                    Ok(ref x) => display_zip_file(x)
                };
            }
        },
        Mode::Extract(_) => { },
        Mode::Create(_) => { },
    };        
}
