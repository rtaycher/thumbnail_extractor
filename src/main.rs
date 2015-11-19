#![feature(convert)]
extern crate zip;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::io::{Error};
use std::fmt;

fn main() {
    match try_main() {
    	Ok(()) => {},
    	Err(e) => {
    		println!("thumbnail extractor failed with {:?}", e.description());
			std::process::exit(1);
			}
    }
    
}

// We derive `Debug` because all types should probably derive `Debug`.
// This gives us a reasonable human readable description of `CliError` values.
#[derive(Debug)]
pub struct StrError<'a> {
    error: &'a str,
}


impl <'a> fmt::Display for StrError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         write!(f, "error: {}", self.error)
    }
}

impl <'a> std::error::Error for StrError<'a> {
    fn description(&self) -> &str {
        self.error
    }
}

impl <'a> From<&'a str> for StrError<'a> {
    fn from(s: &'a str) -> Self {
    	StrError { error: s }
    }
}

fn false_to_error<'a, F>(expr: F,error_message :&'a str) -> Result<(), StrError<'a>> 
		where F : Fn() -> bool {
	if expr() {
		Ok(())
	} else {
		Err(StrError::from(error_message))
	}
}

fn try_main() -> Result<(), Box<std::error::Error>> { 
	let args : Vec<_> = std::env::args_os().collect();
	let filename = args.get(1).expect("needs one argument") ;
	println!("filename: {:?}", filename);
	

	try!(false_to_error(|| { filename.to_string_lossy().ends_with(".odf") || filename.to_string_lossy().ends_with(".ora") }, 
			"This program only supports creating thumbnails from odf or ora files."));
	let f = try!(File::open(filename));
	
    let mut odf = try!(zip::ZipArchive::new(f));
  	let thumb_name = r"Thumbnails/thumbnail.png";
  	let mut thumb_file_in_zip = try!(odf.by_name(thumb_name).or(
  			Err(StrError::from("could not find thumbnail zip file"))));
  	
  	let mut fw = try!(File::create(r"thumbnail.png"));
  	let mut buffer : Vec<u8> = Vec::new();
	try!(thumb_file_in_zip.read_to_end(& mut buffer));
	try!(fw.write_all(buffer.as_slice()));
	Ok(())
}