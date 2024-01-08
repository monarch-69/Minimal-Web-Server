// This file is responsible for handling the URI or the paths

use std::fs::File; 
use std::io::Read;

pub fn uri_parser() -> Vec<String>
{
    let mut path_string = String::new();
    File::open("path.conf").unwrap().read_to_string(&mut path_string).unwrap();
    let path_vector: Vec<_> = path_string.lines().map(|path| path.to_string()).collect();
    
    path_vector
}