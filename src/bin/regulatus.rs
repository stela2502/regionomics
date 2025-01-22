use clap::{Parser};

use regionomics::{BedFile};


use std::io::{Write, BufWriter};
use std::fs::{File, create_dir_all};
use std::path::Path;
use quantify_bam::gtf::GTF;


/// regulatus - A tool identifying potentially regulated genes in a gtf based on distance to regions in a bed like file.
#[derive(Parser)]
#[clap(version = "0.1.0", author = "Stefan L. <stefan.lang@med.lu.se>")]
struct Args {
    /// Path to the input BED file containing genomic regions
    #[clap(short, long)]
    bed: String,

    /// Path to the input GTF file containing gene annotations
    #[clap(short, long)]
    gtf: String,

    /// Path to the output file to save the results (default is stdout)
    #[clap(short, long, default_value_t = String::from("stdout"))]
    outfile: String,

    /// The distance in base pairs to search for genes within the region (default is 1000)
    #[clap(short, long, default_value_t = 1000)]
    distance: usize,

    /*/// Verbosity level of the output (default is 'info')
    #[clap(long, default_value_t = String::from("info"))]
    verbosity: String,*/

    /*/// Enable additional debugging information
    #[clap(long)]
    debug: bool,*/

}

fn main() {
    let args = Args::parse();


    let mut gtf = GTF::new(None);
    gtf.parse_gtf(&args.gtf).unwrap();

    let bed_file = BedFile::new( &args.bed).unwrap();

    let path = Path::new( &args.outfile );
    let dir = path.parent().expect("Failed to get parent directory");

    // Create the directory if it doesn't exist
    if !dir.exists() {
        create_dir_all(dir).expect("Failed to create output directory");
    }
    let file = File::create(path).expect("Unable to outfile");

    let mut writer = BufWriter::new(file);
    let line = format!("bam_region\tgene_name\tdistance_to_bed_center\tstart\tend\n");
    writer.write_all(line.as_bytes()).expect("Failed to write to file");
    
    
    let mut detected_genes = 0;
    for entry in bed_file.into_iter() {
        match &gtf.slice_gtf( &entry.chr, entry.start.saturating_sub( args.distance ), entry.end + args.distance ){
            Ok( slice ) => {
                println!("Here I got {} genes", slice.len() );
                let center = ((entry.start + entry.end) / 2) as i32;
                for gene in slice{
                    detected_genes += 1;
                    let line = format!("{}\t{}\t{}\t{}\t{}\n", entry.as_string(), gene.gene_name, center - args.distance as i32, gene.start, gene.end);
                    writer.write_all(line.as_bytes()).expect("Failed to write to outfile");
                }
            },
            Err(e) => {
                eprintln!("Error while slicing the gtf: {e}");
                continue;
            }

        };
        
    }

    println!("Detected {detected_genes} genes potentially linked to the bed entries:" );
    println!("written to {}", args.outfile );

}