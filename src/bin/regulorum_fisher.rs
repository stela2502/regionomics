use clap::{Parser};

use std::collections::HashMap;

use std::io::{Write, BufWriter};
use std::fs::{File, create_dir_all};
use std::path::Path;
use regionomics::{create_reader, parse_delimited_file, create_writer, parse_value};



/// regulorum_fisher takes an outfile from regulatos and intersects it with a statistical results text table. It only adds the statistical data to the regulatos output.
#[derive(Parser)]
#[clap(version = "0.1.0", author = "Stefan L. <stefan.lang@med.lu.se>")]
struct Args {
    /// An outfile from regulatos
    #[clap(short, long)]
    regulatos_output: String,

    /// a text (gz) file with stats values
    #[clap(short, long)]
    stats_file: String,

    /// Path to the output file to save the results (default is stdout)
    #[clap(short, long, default_value_t = String::from("stdout"))]
    outfile: String,

    /// which column to check for the gene names?
    #[clap(short, long)]
    gene_col: String,

    /// The delimiter for the stats data (default tab)
    #[clap(short, long )]
    sep: Option<String>,

}



fn main() {
    let args = Args::parse();

    let intersects_reader = create_reader( &args.regulatos_output )
        .unwrap_or_else(|e| panic!("Failed to create reader for regulatos_output: {}", e));

    let stats_reader = create_reader( &args.stats_file )
        .unwrap_or_else(|e| panic!("Failed to create reader for stats_file: {}", e));

    // we only need the stats as whole line and the gene name as a string.
    // so keeping all in a Vec::Vec::<String> should work well
    let sep = match args.sep {
        Some(string) => string.chars().next().unwrap(),
        None => '\t',
    };
    let stats_data = parse_delimited_file( stats_reader, sep )
        .unwrap_or_else(|e| panic!("Failed to parse the stats_file: {}", e));
    let mut genes = HashMap::<String, usize>::new();

    let header = match parse_value( &args.gene_col ) {
        Ok( col_id ) => {
            if let Some(header_row) = stats_data.get(0) {
                for (id, row) in stats_data.iter().enumerate().skip(1) {
                    if let Some(gene) = row.get(col_id) {
                        genes.insert(gene.clone(), id);
                    }else {
                        panic!("The stats table does not contain enough columns!");
                    }
                }
                header_row
            } else {
                panic!("Header row is missing in stats_data");
            }
        },
        Err( col_name ) => {
            // Assume the first row contains the column headers
            if let Some(header_row) = stats_data.get(0) {
                let col_id = header_row
                    .iter()
                    .enumerate()
                    .find_map(|(id, name)| if col_name == name { Some(id) } else { None })
                    .expect(&format!("Column '{}' not found in header row", col_name));


                for (id, row) in stats_data.iter().enumerate().skip(1) {
                    if let Some(gene) = row.get(col_id) {
                        genes.insert(gene.clone(), id);
                    }
                }

                header_row
            }else {
                panic!("Header row is missing in stats_data");
            }
        }
    };

    let intersects_data = parse_delimited_file( intersects_reader, '\t' )
        .unwrap_or_else(|e| panic!("Failed to parse the regulatos_output: {}", e));

    let mut writer = create_writer( &args.outfile )
        .unwrap_or_else(|e| panic!("Failed to create the outfile: {}", e));

    let path = Path::new(&args.stats_file);
    let addon = path.file_name().and_then(|name| name.to_str())
        .unwrap();
    fn addon_to(vec:&Vec<String>, add:&str, sep:&str ) -> String {
        vec.iter().map(|string| format!("{}_{}", add, string) ).collect::<Vec<String>>().join(sep)
    }
    

     if let Some(regulus_header_row) = intersects_data.get(0) {
        let header_line = regulus_header_row.join("\t") + "\t" + &addon_to(header, addon, "\t");
        writer.write_all(header_line.as_bytes()).expect("Failed to write header to outfile");
    }

    // Write data rows
    for main_row in intersects_data.iter().skip(1) {
        let gene_id = &main_row[1];  // Assuming the gene ID is in column 1

        let mut merged_line = if let Some(&gene_index) = genes.get(gene_id) {
            // If the gene is found, retrieve the corresponding row from stats_data
            if let Some(stats_row) = stats_data.get(gene_index) {
                // Merge main_row and the respective stats_row
                main_row.iter()
                    .chain(stats_row.iter())  // Append stats_row to main_row
                    .map(|s| s.clone())  // Clone the strings so we can join them
                    .collect::<Vec<String>>()
                    .join("\t")
            } else {
                // Handle the case where stats_data doesn't have a row for the gene
                main_row.iter()
                    .chain(vec!["na".to_string(); stats_data[0].len()].iter())  // Concatenate "na" for missing stats
                    .map(|s| s.clone())
                    .collect::<Vec<String>>()
                    .join("\t")
            }
        } else {
            // If gene_id is not in genes map, add "na" for each missing stat
            main_row.iter()
                .chain(vec!["na".to_string(); stats_data[0].len()].iter())  // Concatenate "na" for missing stats
                .map(|s| s.clone())
                .collect::<Vec<String>>()
                .join("\t")
        };
        merged_line +="\n";
        writer.write(merged_line.as_bytes()).expect("Failed to write line to outfile");
    }
    

    println!("Finished!");

}

