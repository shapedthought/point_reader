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
use colored::*;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{modifiers::UTF8_SOLID_INNER_BORDERS, Table};
use comfy_table::{Cell, Color, ContentArrangement};
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use std::{fs, path::PathBuf};

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

    let file = fs::read_to_string(cli.file.clone())?;

    let vse_data: models::Root = serde_json::from_str(&file)?;

    let workload_names = vse_data
        .raw_out
        .sites
        .iter()
        .flat_map(|sites| sites.storage.iter())
        .flat_map(|storage| storage.points.primary_backup.iter())
        .map(|primary_backup| primary_backup.workload_name.clone())
        .filter(|name| name.contains("showPoints"))
        .collect::<Vec<String>>();

    let workload_selected = if let Some(workload) = &cli.workload {
        workload_names.iter().position(|x| x == workload).unwrap()
    } else {
        if workload_names.len() == 1 {
            println!("Only one workload found, selecting {} by default", workload_names[0].green().bold());
            0 as usize
        } else {
            Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a workload")
            .default(0)
            .items(&workload_names[..])
            .interact()
            .unwrap()
        }
    };

    let workload_name = workload_names[workload_selected].clone();

    let workload = vse_data
        .raw_out
        .sites
        .iter()
        .flat_map(|sites| sites.storage.iter())
        .flat_map(|storage| storage.points.primary_backup.iter())
        .find(|backup| backup.workload_name == workload_name)
        .unwrap();

    let mut table = Table::new();
    let mut table_totals = Table::new();

    let mut tier_data: Vec<Vec<String>> = Vec::new();

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
        
    add_data(&cli, &workload.archive_tier_result, &mut tier_data, "a");
    add_data(&cli, &workload.capacity_tier_result, &mut tier_data, "c");
    add_data(&cli, &workload.performance_tier_result, &mut tier_data, "p");

    for row in tier_data.iter() {
        table.add_row(row);
    }

    let workload_name = &workload.workload_name;

    if workload_names.len() > 1 {
        println!("Workload Selected: {}", workload_name.green().bold());
    }
    
    println!("{}", table);

    table_totals
    .load_preset(UTF8_FULL)
    .apply_modifier(UTF8_ROUND_CORNERS)
    .apply_modifier(UTF8_SOLID_INNER_BORDERS)
    .set_content_arrangement(ContentArrangement::Dynamic)
    .set_header(vec![
        Cell::new("Tier").fg(Color::Green),
        Cell::new("Capacity TiB"),
        Cell::new("Point Count")
    ]);

    let perf_total = format!("{:.2}", workload.total_performance);
    let cap_total = format!("{:.2}", workload.total_capacity);
    let arch_total = format!("{:.2}", workload.total_archive);
    let perf_points = format!("{:.2}", workload.perf_tier_point_count);
    let cap_points = format!("{:.2}", workload.cap_tier_point_count);
    let arch_points = format!("{:.2}", workload.arch_tier_point_count);

    table_totals.add_row(vec!["Archive Tier".to_string(), arch_total, arch_points]);
    table_totals.add_row(vec!["Capacity Tier".to_string(), cap_total, cap_points]);
    table_totals.add_row(vec!["Performance Tier".to_string(), perf_total, perf_points]);

    println!("{}", table_totals);

    if let Some(file_name) = cli.save_file {
        let file_name = if !file_name.ends_with(".csv") {
            format!("{}.csv", file_name)
        } else {
            file_name
        };

        let mut wtr = csv::Writer::from_path(file_name)?;

        wtr.write_record([
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

        for row in tier_data {
            wtr.write_record(row)?;
        }

        wtr.flush()?;
    }

    Ok(())
}

fn add_data(cli: &Cli, tier_result: &Vec<models::TierResult>, tier_data: &mut Vec<Vec<String>>, tier: &str) {
    if cli.tiers.to_lowercase().contains(tier) {
        for point in tier_result {
            tier_data.push(vec![
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
