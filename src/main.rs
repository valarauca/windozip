extern crate clap;
extern crate zip;
extern crate regex;

mod cli;
use cli::{Mode,match_regex,cli,claptrap};

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
                    Ok(x) => {
                        let name = x.name();
                        if match_regex(name) {
                            println!("{}",name);
                        }
                    }
                };
            }
        },
        Mode::Extract(_) => { },
        Mode::Create(_) => { },
    };        
}
