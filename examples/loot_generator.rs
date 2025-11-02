use praeda::*;
use std::fs;
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage: loot_generator <config.toml> <output.json> <num_items>");
        eprintln!("");
        eprintln!("Example:");
        eprintln!("  cargo run --example loot_generator -- examples/test_data.toml loot.json 10");
        eprintln!("");
        eprintln!("This will:");
        eprintln!("  1. Load generator configuration from config.toml");
        eprintln!("  2. Generate the specified number of items");
        eprintln!("  3. Save the generated items to output.json");
        std::process::exit(1);
    }

    let config_path = &args[1];
    let output_path = &args[2];
    let num_items: u32 = args[3].parse()
        .expect("num_items must be a number");

    // Load generator configuration from TOML
    eprintln!("Loading configuration from {}...", config_path);
    let mut gen = PraedaGenerator::new();
    gen.load_data_toml_from_file(config_path)?;

    // All configuration is loaded from TOML (item_types, attributes, names, affixes)

    // Generate loot
    eprintln!("Generating {} items...", num_items);
    let options = GeneratorOptions {
        number_of_items: num_items,
        base_level: 10.0,
        level_variance: 5.0,
        affix_chance: 0.75,  // Higher chance to see affixes in generated items
        linear: true,
        scaling_factor: 1.0,
    };

    let items = gen.generate_loot(&options, &GeneratorOverrides::empty(), "cli")?;

    // Save output to JSON
    eprintln!("Saving {} items to {}...", items.len(), output_path);
    let output_json = serde_json::to_string_pretty(&items)?;
    fs::write(output_path, output_json)
        .expect("Failed to write output file");

    println!("✅ Successfully generated {} items and saved to {}", items.len(), output_path);
    Ok(())
}
