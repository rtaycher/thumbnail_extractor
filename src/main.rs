#![feature(convert)]
extern crate zip;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::io::Error;
use std::fmt;
use std::path::Path;
use std::path::PathBuf;

extern crate clap;
use clap::App;

fn main() {
    match try_main() {
        Ok(()) => {}
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

fn false_to_error<'a, F>(expr: F, error_message: &'a str) -> Result<(), StrError<'a>>
    where F: Fn() -> bool
{
    if expr() {
        Ok(())
    } else {
        Err(StrError::from(error_message))
    }
}

fn get_input_filename_parent_dir(input_filename: &str) -> PathBuf {
    if Path::new(input_filename).is_absolute() {
        PathBuf::from(input_filename)
            .parent()
            .expect("input file should have a parent")
            .to_path_buf()
    } else {
        let cur_dir = std::env::current_dir().expect("current dir should probably exist");
        let full_input_file = cur_dir.join(input_filename);
        let parent_dir_result = full_input_file.parent();

        parent_dir_result.expect("input file should have a parent").to_path_buf()

    }
}

fn try_main() -> Result<(), Box<std::error::Error>> {
    let matches = App::new("thumnail_extractor")
                      .version("0.1.0")
                      .author("Roman A. Taycher <rtaycher1987@gmail.com>")
                      .about("Extracts thumbnails embeded thumbnails from ora and odt files")
                      .args_from_usage("<INPUT> 'Sets the input ora or odt file to extract the \
                                        thumbnail from'
                          				    \
                                        [OUTPUT] 'Set where to write the output file (defaults \
                                        to thumbnail.png if not provided)'")
                      .get_matches();

    let input_filename = matches.value_of("INPUT").unwrap().clone();
    let input_dir_path = get_input_filename_parent_dir(input_filename);

    let output_filename = matches.value_of("OUTPUT")
                                 .map(PathBuf::from)
                                 .unwrap_or(input_dir_path.join("thumbnail.png"));


    try!(false_to_error(|| input_filename.ends_with(".odt") || input_filename.ends_with(".ora"),
                        "This program only supports creating thumbnails from odt or ora files."));
    let f = try!(File::open(input_filename));

    let mut odt = try!(zip::ZipArchive::new(f));
    let thumb_name = r"Thumbnails/thumbnail.png";
    let mut thumb_file_in_zip = try!(odt.by_name(thumb_name)
                                        .or(Err(StrError::from("could not find thumbnail zip \
                                                                file"))));

    let mut fw = try!(File::create(output_filename));
    let mut buffer: Vec<u8> = Vec::new();
    try!(thumb_file_in_zip.read_to_end(&mut buffer));
    try!(fw.write_all(buffer.as_slice()));
    Ok(())
}
