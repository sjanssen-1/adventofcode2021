use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

const FILE_PATH: &str = "src/input.txt";

fn main() -> Result<(), Error> {
    println!("In file {}", FILE_PATH);

    let inputs = read(FILE_PATH)?;

    let mut increases: i64 = 0;
    let mut previous: i64 = 0;

    for input in inputs {
        if previous != 0 && input > previous
        {
            increases += 1;
        }
        previous = input;
    }
    println!("{}", increases);
    Ok(())
}

fn read(path: &str) -> Result<Vec<i64>, Error> {
    let file = File::open(path)?; // open file by given path
    // wrap file into generic buffered reader, it will use 4 KB buffer internally
    // to reduce number of syscalls, thus improving performance
    let br = BufReader::new(file);
    // create an empty vector, type of the stored elements will be inferred
    let mut v = Vec::new();
    // br.lines() creates an iterator over lines in the reader
    // see: https://doc.rust-lang.org/std/io/trait.BufRead.html#method.lines
    for line in br.lines() {
        // IO operations generally can return error, we check if got
        // an error,in which case we return this error as the function result
        let line = line?;
        let n = line
            .trim() // trim "whitespaces"
            .parse() // call `str::parse::<i64>(&self)` method on the trimmed line, which parses integer
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?; // parse() can return error (e.g. for string "abc"), here if we got it, we convert it to `std::io::Error` type and return it as function result
        v.push(n); // push acquired integer to the vector
    }
    Ok(v) // everything is Ok, return vector
}