extern crate regex;
extern crate reqwest;
extern crate select;
extern crate zip;


use zip;
use regex::Regex;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};
use std::fs::File;
use std::io::prelude::*;

struct Comic {
    comicSrc: String,
    comicLink: String,
}

impl Iterator for Comic {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let reg = Regex::new(r"BluePageBanner").unwrap();
        let res = reqwest::get(&self.comicLink).unwrap();
        assert!(res.status().is_success());
        let doc = Document::from_read(res).unwrap();

        self.comicLink = String::from(doc.find(Class("nav-next").descendant(Name("a")))
            .filter_map(|n| n.attr("href"))
            .next()?);

        self.comicSrc = String::from(doc.find(Name("img"))
            .filter_map(|n| n.attr("src"))
            .filter(|n| !reg.is_match(n))
            .next()?);

        match self.comicLink.is_empty() {
            true => None,
            false => return Some(self.comicSrc.clone()),
        }
    }
}

fn getComic() -> Comic {
    Comic {
        comicSrc: String::new(),
        comicLink: String::from("https://mollybeans.com/comic/eggs/"),
    }
}

/*
fn doit(zip: zip::ZipWriter, filename: File) -> zip::result::ZipResult<()>
{
    let path = std::path::Path::new("/home/noah/comic/mollybeans.cbz");
	let file = std::fs::File::create(&path).unwrap();
    let mut zip = zip::ZipWriter::new(file);

    try!(zip.add_directory(dir, FileOptions::default()));

    let options = FileOptions::default().compression_method(zip::CompressionMethod::Stored).unix_permissions(0o755);
    try!(zip.start_file(, options));
    try!(zip.write_all());

    try!(zip.finish());
    Ok(())
}
*/

fn main() {
	let args: Vec<_> = std::env::args().collect();
    let mut count = 1;
    let str_init = &args[1];
//    let path = std::path::Path::new("/home/noah/comic/mollybeans.cbz");
//   	let file = std::fs::File::create(&path).unwrap();
//   	let mut zip = zip::ZipWriter::new(file);
    
    for i in getComic() {
        let mut s = String::from(str_init);
        s.push_str(&count.to_string());
        s.push_str(".png");
        let mut f = File::create(s).unwrap();
        reqwest::get(&i).unwrap().copy_to(&mut f).unwrap();
        println!("Comic # {} is done!", count);
        count += 1;
    }
}
