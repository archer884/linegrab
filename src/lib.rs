use std::fs::File;
use std::io::{ BufRead, BufReader, Read };

/// Returns input from either a file or stdin as an Iterator
///
/// The actual behavior of this function skips bad lines, whatever those might happen to be, so
/// don't consume it if, for whatever reason, you're trying to read input that isn't a valid UTF8
/// string.
pub fn linegrab<T>(optional: Option<String>, fallback: T) -> Box<Iterator<Item=String>>
    where T: Read + 'static
{
    // I prefer to use box syntax for this kind of thing, but I want this usable on stable
    match optional.and_then(|path| File::open(&path).ok()) {
        None => Box::new(
            BufReader::new(fallback).lines().filter_map(|line| line.ok())
        ),
        
        Some(reader) => Box::new(
            BufReader::new(reader).lines().filter_map(|line| line.ok())
        ),
    }
}
