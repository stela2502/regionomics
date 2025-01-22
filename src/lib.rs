use std::path::{Path, PathBuf};

use std::io::{BufReader, BufRead, BufWriter, Write, Read};
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use flate2::Compression;
use std::fs::File;

#[derive(Debug, Clone)]
pub struct BedEntry{
    pub chr: String,
    pub start: usize,
    pub end: usize
}

impl BedEntry{
    // Method to create a BedEntry from a Vec<&str>
    pub fn from_vec(vec: Vec<&str>) -> Result<Self, &'static str> {
        if vec.len() < 3 {
            return Err("Input vector must contain at least 3 elements.");
        }

        let chr = vec[0].to_string();
        let start = vec[1].parse::<usize>().map_err(|_| "Invalid start value")?;
        let end = vec[2].parse::<usize>().map_err(|_| "Invalid end value")?;

        Ok(BedEntry { chr, start, end })
    }
    pub fn as_string(&self) -> String {
        format!("{}:{}-{}", self.chr, self.start, self.end)
    }

}

pub struct BedFile{
    data: Vec<BedEntry>,
    index: usize,
}

impl BedFile {
    pub fn new( file_path:&str ) -> Result<Self, String> {

        let mut data = Vec::<BedEntry>::with_capacity( 10_000 );
        let path = Path::new(file_path);
        let mut line_id = 0;
        // Determine the reader: plain text or gzip
        let reader: Box<dyn Read> = if file_path.ends_with(".gz") || Self::is_gzipped(file_path)? {
            let file = File::open(&path).map_err(|e| e.to_string())?;
            Box::new(GzDecoder::new(file))
        } else {
            let file = File::open(&path).map_err(|e| e.to_string())?;
            Box::new(file)
        };

        let reader = BufReader::new(reader);

        for line in reader.lines() {
            line_id +=1;
            let line = line.map_err(|e| e.to_string())?;
            if line.starts_with('#') {
                continue; // Skip comment lines
            }

            let fields: Vec<&str> = line.split('\t').collect();
            let entry = BedEntry::from_vec( fields )?;
            data.push(entry);
        }

        Ok( Self{ 
            data,
            index: 0,
        } )
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }

    // Helper function to detect gzip format by inspecting magic bytes
    fn is_gzipped(file_path: &str) -> Result<bool, String> {
        let mut file = File::open(file_path).map_err(|e| e.to_string())?;
        let mut magic_bytes = [0; 2];
        file.read_exact(&mut magic_bytes).map_err(|e| e.to_string())?;
        Ok(magic_bytes == [0x1F, 0x8B]) // gzip magic bytes
    }
}


// Implement the Iterator trait directly for BedFile
impl Iterator for BedFile {
    type Item = BedEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.data.len() {
            let entry = self.data[self.index].clone();
            self.index += 1;
            Some(entry)
        } else {
            None
        }
    }
}