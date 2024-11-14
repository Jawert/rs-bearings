mod bearing_line;
mod geo_helpers;
mod point;
mod safe_math;

use bearing_line::BearingLine;
use csv::ReaderBuilder;
use geo_helpers::get_intersection;
use std::env;
use std::error::Error;
use std::fs::File;
use std::process;
use std::time::Instant;

fn read_bearing_lines_from_csv(file_path: &str) -> Result<Vec<BearingLine>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(File::open(file_path)?);
    let mut bearing_lines = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let latitude: f64 = record[0].parse()?;
        let longitude: f64 = record[1].parse()?;
        let bearing: f64 = record[2].parse()?;
        bearing_lines.push(BearingLine::new(latitude, longitude, bearing)?);
    }

    Ok(bearing_lines)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Get the CSV file path from the command line argument
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <csv_file_path>", args[0]);
        process::exit(1);
    }
    let csv_file_path = &args[1];

    // Read the bearing lines from the CSV file
    let bearing_lines = read_bearing_lines_from_csv(csv_file_path)?;

    // Start the timer
    let start_time = Instant::now();

    let mut intersection_count = 0;
    let total_pairs = (bearing_lines.len() * (bearing_lines.len() - 1)) / 2;

    // Iterate over each pair of bearing lines
    for i in 0..bearing_lines.len() {
        for j in i + 1..bearing_lines.len() {
            match get_intersection(&bearing_lines[i], &bearing_lines[j]) {
                Ok(intersection) => {
                    intersection_count += 1;
                    println!(
                        "Intersection of ({}, {}) with ({}, {}) is: latitude: {}, longitude: {}",
                        bearing_lines[i].latitude(),
                        bearing_lines[i].longitude(),
                        bearing_lines[j].latitude(),
                        bearing_lines[j].longitude(),
                        intersection.latitude,
                        intersection.longitude
                    );
                }
                Err(e) => {
                    println!(
                        "Error calculating intersection of ({}, {}) with ({}, {}): {}",
                        bearing_lines[i].latitude(),
                        bearing_lines[i].longitude(),
                        bearing_lines[j].latitude(),
                        bearing_lines[j].longitude(),
                        e
                    );
                }
            }
        }
    }

    // Print the number of intersections found and time taken
    let duration = start_time.elapsed();
    println!("\nProcessed {} pairs of bearing lines.", total_pairs);
    println!(
        "Found {} intersections in {:?}.",
        intersection_count, duration
    );

    Ok(())
}
