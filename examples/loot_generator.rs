use praeda::*;
use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about = "Generate random loot items using Praeda", long_about = None)]
struct Args {
    /// Path to TOML configuration file with item types, attributes, and affixes
    #[arg(short = 'i', long)]
    input: String,

    /// Path where generated items will be saved as JSON
    #[arg(short = 'o', long)]
    output: String,

    /// Number of items to generate
    #[arg(short = 'n', long)]
    num_items: u32,

    /// Average item level (default: 10.0)
    #[arg(short = 'b', long, default_value = "10.0")]
    base_level: f64,

    /// Range around base level (default: 5.0)
    #[arg(short = 'v', long, default_value = "5.0")]
    level_variance: f64,

    /// Probability of applying affixes 0.0-1.0 (default: 0.75)
    #[arg(short = 'a', long, default_value = "0.75")]
    affix_chance: f64,

    /// Use exponential scaling instead of linear
    #[arg(long = "exponential")]
    exponential: bool,

    /// Multiplier for attribute scaling (default: 1.0)
    #[arg(short = 's', long, default_value = "1.0")]
    scaling_factor: f64,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Load generator configuration from TOML
    eprintln!("Loading configuration from {}...", args.input);
    let mut gen = PraedaGenerator::new();
    gen.load_data_toml_from_file(&args.input)?;

    // Generate loot with user-specified options
    let linear = !args.exponential;  // Default is linear unless --exponential flag is set

    eprintln!("Generating {} items...", args.num_items);
    eprintln!("  Base Level: {}", args.base_level);
    eprintln!("  Level Variance: {}", args.level_variance);
    eprintln!("  Affix Chance: {}", args.affix_chance);
    eprintln!("  Scaling Mode: {}", if linear { "linear" } else { "exponential" });
    eprintln!("  Scaling Factor: {}", args.scaling_factor);

    let options = GeneratorOptions {
        number_of_items: args.num_items,
        base_level: args.base_level,
        level_variance: args.level_variance,
        affix_chance: args.affix_chance,
        linear,
        scaling_factor: args.scaling_factor,
    };

    let items = gen.generate_loot(&options, &GeneratorOverrides::empty(), "cli")?;

    // Save output to JSON
    eprintln!("Saving {} items to {}...", items.len(), args.output);
    let output_json = serde_json::to_string_pretty(&items)?;
    fs::write(&args.output, output_json)
        .expect("Failed to write output file");

    println!("✅ Successfully generated {} items and saved to {}", items.len(), args.output);
    Ok(())
}
