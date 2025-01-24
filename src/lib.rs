use std::path::{Path, PathBuf};

use std::io::{BufReader, BufRead, BufWriter, Write, Read};
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use flate2::Compression;
use std::fs::{File, create_dir_all};


// Helper function to detect gzip format by inspecting magic bytes
fn is_gzipped(file_path: &str) -> Result<bool, String> {
    let mut file = File::open(file_path).map_err(|e| e.to_string())?;
    let mut magic_bytes = [0; 2];
    file.read_exact(&mut magic_bytes).map_err(|e| e.to_string())?;
    Ok(magic_bytes == [0x1F, 0x8B]) // gzip magic bytes
}

/// Determines the appropriate reader (plain text or gzip) based on the file extension or content.
/// 
/// # Arguments
/// * `file_path` - The path to the file to be opened.
///
/// # Returns
/// A `BufReader` wrapped around the appropriate reader.
///
/// # Errors
/// Returns an error if the file cannot be opened or if there's an issue determining the file type.
pub fn create_reader(file_path: &str) -> Result<BufReader<Box<dyn Read>>, String> {
    let reader: Box<dyn Read> = if file_path.ends_with(".gz") || is_gzipped(file_path)? {
        let file = File::open(&file_path).map_err(|e| e.to_string())?;
        Box::new(GzDecoder::new(file))
    } else {
        let file = File::open(&file_path).map_err(|e| e.to_string())?;
        Box::new(file)
    };

    Ok(BufReader::new(reader))
}

/// Determines the appropriate writer (plain text or gzip) based on the file extension.
/// 
/// # Arguments
/// * `file_path` - The path to the file to be opened.
///
/// # Returns
/// A `BufWriter` wrapped around the appropriate writer.
///
/// # Errors
/// Returns an error if the file cannot be opened or if there's an issue determining the file type.
pub fn create_writer(file_path: &str) -> Result<Box<dyn Write>, String> {
    // Check if the parent directory exists, and create it if it doesn't
    if let Some(parent) = Path::new(file_path).parent() {
        if !parent.exists() {
            create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
        }
    }

    let writer: Box<dyn Write> = if file_path.ends_with(".gz") {
        // If the file ends with ".gz", use GzEncoder to write compressed data
        let file = File::create(file_path).map_err(|e| e.to_string())?;
        Box::new(GzEncoder::new(file, Compression::default()))
    } else {
        // If not a ".gz" file, write to a plain file
        let file = File::create(file_path).map_err(|e| e.to_string())?;
        Box::new(file)
    };

    Ok(writer)
}


/// Parses a delimited text file into a `Vec<Vec<String>>`.
///
/// # Arguments
/// * `reader` - A buffered reader for the file.
/// * `delimiter` - The delimiter to use for splitting the lines.
///
/// # Returns
/// A `Vec<Vec<String>>` where each inner vector represents a row of the file.
pub fn parse_delimited_file(reader: BufReader<Box<dyn Read>>, delimiter: char) -> Result<Vec<Vec<String>>, String> {
    let mut result = Vec::new();
    for line in reader.lines() {
        let line = line.map_err(|e| e.to_string())?;
        let row: Vec<String> = line.split(delimiter).map(|s| s.to_string()).collect();
        result.push(row);
    }
    Ok(result)
}

/// Checks if `value` can be parsed into a number. 
/// If it can, returns `Some(usize)`; otherwise, returns `None` to indicate it's a string.
///
/// # Arguments
/// * `gene_col` - A string reference representing the gene column value.
///
/// # Returns
/// `Result<usize, &str>`: A number if it can be parsed, or the original string otherwise.
pub fn parse_value(value: &str) -> Result<usize, &str> {
    match value.parse::<usize>() {
        Ok(num) => Ok(num), // Successfully parsed as a number
        Err(_) => Err(value), // Cannot be parsed; treat it as a string
    }
}


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
        let reader = create_reader(file_path)?;

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