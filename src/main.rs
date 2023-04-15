//! # Point Reader
//! 
//! This is a simple CLI application that can take a VSE export JSON file with the points enabled and 
//! display them in a table and optionally to a CSV file.
//! 
//! Installation
//! 
//! To install the tool run the following command
//! 
//! Example: 
//! 
//! ```
//! cargo install point_reader
//! ```
//! You can get a list of the available commands by using the --help flag.
//! 
//! Example:
//! ```
//! point_reader --help
//! ```
//! 
//! To run the tool without outputting a CSV run the following
//! 
//! Example:
//! 
//! ```
//! point_reader --file VSE_Exports.json
//! ```
//! 
//! To run the tool and output a CSV run the following
//! 
//! Example:
//! 
//! ```
//! point_reader --file VSE_Exports.json --save-file points_data.csv
//! ```
//! 
//! The .csv is optional and will be added if not provided
//! 
//! To run the tool and only show a specific tier(s) run the following
//!
//! Example:
//! 
//! ```
//! point_reader --file VSE_Exports.json --tiers pc // Only show performance and capacity tiers
//! ```
//! 
//! The tiers flag can be any combination of the following:
//! 
//! * p - Performance Tier
//! * c - Capacity Tier
//! * a - Archive Tier
//! 
//! The tiers flag is case insensitive
//! 
//! If you know the workload name you can run non-interactively by using the --workload flag
//! 
//! Example:
//! 
//! ```
//! point_reader --file <path to file> --workload <workload name>
//! ```
//! 

mod models;

use anyhow::Result;
use clap::Parser;
use comfy_table::{Color, Cell, ContentArrangement};
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{modifiers::UTF8_SOLID_INNER_BORDERS, Table};
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use std::{fs, path::PathBuf};
use colored::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to file
    #[clap(short, long)]
    file: PathBuf,

    /// Tiers to show
    #[arg(short, long, default_value_t = String::from("pca"))]
    tiers: String,

    /// Print results to csv
    #[arg(short, long)]
    save_file: Option<String>,

    /// Select the workload by name
    #[arg(short, long)]
    workload: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let file = fs::read_to_string(cli.file)?;

    let vse_data: models::Root = serde_json::from_str(&file)?;

    let workload_names = vse_data
        .raw_out
        .sites
        .iter()
        .map(|sites| sites.storage.iter())
        .flatten()
        .map(|storage| storage.points.primary_backup.iter())
        .flatten()
        .map(|primary_backup| primary_backup.workload_name.clone())
        .collect::<Vec<String>>();

    let workload_selected = if let Some(workload) = cli.workload {
        workload_names
            .iter()
            .position(|x| x == &workload)
            .unwrap()
    } else {
        Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a workload")
            .default(0)
            .items(&workload_names[..])
            .interact()
            .unwrap()
    };

    // let workload_selected = Select::with_theme(&ColorfulTheme::default())
    //     .with_prompt("Select a workload")
    //     .default(0)
    //     .items(&workload_names[..])
    //     .interact()
    //     .unwrap();

    let workloads: Vec<_> = vse_data
        .raw_out
        .sites
        .iter()
        .map(|sites| sites.storage.iter())
        .flatten()
        .map(|storage| storage.points.primary_backup.clone())
        .collect();

    let selected_workload = workloads[workload_selected].clone();

    let mut table = Table::new();

    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .apply_modifier(UTF8_SOLID_INNER_BORDERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Point Type").fg(Color::Green),
            Cell::new("backupSize (TiB)"),
            Cell::new("day"),
            Cell::new("isFull"),
            Cell::new("isGFS"),
            Cell::new("daily"),
            Cell::new("weekly"),
            Cell::new("monthly"),
            Cell::new("yearly"),
        ]);

    for workload in selected_workload.iter() {
        if cli.tiers.to_lowercase().contains("a") {
            for point in &workload.archive_tier_result {
                table.add_row(vec![
                    point.point_type.clone(),
                    format!("{:.2}", point.backup_size),
                    point.point.day.to_string(),
                    point.point.is_full.to_string(),
                    point.point.is_gfs.to_string(),
                    point.point.flags.daily.to_string(),
                    point.point.flags.weekly.to_string(),
                    point.point.flags.monthly.to_string(),
                    point.point.flags.yearly.to_string(),
                ]);
            }
        }

        if cli.tiers.to_lowercase().contains("c") {
            for point in &workload.capacity_tier_result {
                table.add_row(vec![
                    point.point_type.clone(),
                    format!("{:.2}", point.backup_size),
                    point.point.day.to_string(),
                    point.point.is_full.to_string(),
                    point.point.is_gfs.to_string(),
                    point.point.flags.daily.to_string(),
                    point.point.flags.weekly.to_string(),
                    point.point.flags.monthly.to_string(),
                    point.point.flags.yearly.to_string(),
                ]);
            }
        }

        if cli.tiers.to_lowercase().contains("p") {
            for point in &workload.performance_tier_result {
                table.add_row(vec![
                    point.point_type.clone(),
                    format!("{:.2}", point.backup_size),
                    point.point.day.to_string(),
                    point.point.is_full.to_string(),
                    point.point.is_gfs.to_string(),
                    point.point.flags.daily.to_string(),
                    point.point.flags.weekly.to_string(),
                    point.point.flags.monthly.to_string(),
                    point.point.flags.yearly.to_string(),
                ]);
            }
        }
    }

    let workload_name = &selected_workload[0].workload_name;

    println!("Workload Selected: {}", workload_name.green().bold());
    println!("{}", table);

    if let Some(file_name) = cli.save_file {
        let file_name = if !file_name.ends_with(".csv") {
            format!("{}.csv", file_name)
        } else {
            file_name
        };

        let mut wtr = csv::Writer::from_path(file_name)?;

        wtr.write_record(&[
            "Workload Name",
            "Point Type",
            "backupSize",
            "day",
            "isFull",
            "isGFS",
            "daily",
            "weekly",
            "monthly",
            "yearly",
        ])?;

        for workload in selected_workload {
            if cli.tiers.to_lowercase().contains("a") {
                for point in workload.archive_tier_result {
                    wtr.write_record(&[
                        workload.workload_name.clone(),
                        point.point_type.clone(),
                        format!("{:.2}", point.backup_size),
                        point.point.day.to_string(),
                        point.point.is_full.to_string(),
                        point.point.is_gfs.to_string(),
                        point.point.flags.daily.to_string(),
                        point.point.flags.weekly.to_string(),
                        point.point.flags.monthly.to_string(),
                        point.point.flags.yearly.to_string(),
                    ])?;
                }
            }

            if cli.tiers.to_lowercase().contains("c") {
                for point in workload.capacity_tier_result {
                    wtr.write_record(&[
                        workload.workload_name.clone(),
                        point.point_type.clone(),
                        format!("{:.2}", point.backup_size),
                        point.point.day.to_string(),
                        point.point.is_full.to_string(),
                        point.point.is_gfs.to_string(),
                        point.point.flags.daily.to_string(),
                        point.point.flags.weekly.to_string(),
                        point.point.flags.monthly.to_string(),
                        point.point.flags.yearly.to_string(),
                    ])?;
                }
            }

            if cli.tiers.to_lowercase().contains("p") {
                for point in workload.performance_tier_result {
                    wtr.write_record(&[
                        workload.workload_name.clone(),
                        point.point_type.clone(),
                        format!("{:.2}", point.backup_size),
                        point.point.day.to_string(),
                        point.point.is_full.to_string(),
                        point.point.is_gfs.to_string(),
                        point.point.flags.daily.to_string(),
                        point.point.flags.weekly.to_string(),
                        point.point.flags.monthly.to_string(),
                        point.point.flags.yearly.to_string(),
                    ])?;
                }
            }
        }

        wtr.flush()?;
    }

    Ok(())
}
