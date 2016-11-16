use super::zip::read::{ZipFile,ZipArchive};
use super::cli::{match_regex,display_size};
use std::io;
use std::io::prelude::*;
use std::ops::RangeTo;

///Display a Zip File
///
///Handles interacting with flags
pub fn display_zip_file(z: &ZipFile) {
    let name = z.name();
    if match_regex(name) {
        println!("{}",name);
        if display_size() {
            let size = z.size();
            let comp = z.compressed_size();
            let ratio: f64 = comp as f64/size as f64;
            let ratio = ratio * 100f64;
            let ratio = ratio.round() / 100f64;
            println!("\torg: {:?} comp: {:?} ratio: {:?}%",size,comp,ratio);
    
        }
    }
}






///Borrow both the zip file, and 
pub fn read_zip(z: &mut ZipFile, buff: &mut Vec<u8>) {
    match z.read_to_end(buff) {
        Ok(_) => { },
        Err(e) => panic!("\n\nCould not read file\n{:?}\n\n",e)
    };
}
