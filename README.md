# Praeda

This library provides a flexible system for randomly generating items/loot with affixes, attributes, and quality tiers.

## Quick Start

Choose your usage path below:

### A) Using Praeda as a Rust Library

Add to your `Cargo.toml`:
```toml
[dependencies]
praeda = { git = "https://www.github.com/EddieDover/praeda.git", branch = "master" }
```

Then see the **[Rust Library Usage](#using-as-a-rust-library)** section for detailed examples.

### B) Using Praeda from C++

1. Build the shared library:
```bash
cargo build --release
```

2. See the complete working example in **[examples/cpp/](examples/cpp/)** directory which includes:
   - `test_praeda.cpp` - Full test program demonstrating programmatic and TOML-based configuration
   - `praeda.hpp` - Header file to copy to your project
   - `CMakeLists.txt` - Build configuration

3. For detailed C++ integration guide, see **[FFI.md](FFI.md)**.

### C) Using Praeda from C#

1. Build the shared library:
```bash
cargo build --release
```

2. See the complete working example in **[examples/csharp/](examples/csharp/)** directory which includes:
   - `PraedaTest.cs` - Full test program demonstrating programmatic and TOML-based configuration
   - `PraedaGenerator.cs` - Wrapper class to use in your projects
   - `PraedaTest.csproj` - Project configuration

3. For detailed C# integration guide, see **[FFI.md](FFI.md)**.

### D) Using the CLI Tool

```bash
cargo run --example loot_generator -- \
  --input examples/test_data.toml \
  --output loot.json \
  --num-items 10
```

See **[Loot Generator CLI](#example-loot-generator-cli)** section for detailed examples.

### Development Setup

For detailed development instructions, see [CONTRIBUTING.md](CONTRIBUTING.md) which includes:
- Setting up git hooks for code quality
- Conventional Commits standards
- Code coverage requirements (85% threshold)
- Pull request process
- Development tips and useful commands

## Example: Loot Generator CLI

Praeda includes a complete command-line example that demonstrates generating items from a TOML configuration file with full control over generation parameters.

### Basic Usage

```bash
# Generate 10 items with defaults
cargo run --example loot_generator -- --input examples/test_data.toml --output output.json --num-items 10
```

Or using short flags:

```bash
cargo run --example loot_generator -- -i examples/test_data.toml -o output.json -n 10
```

### Advanced Usage

You can customize all generation parameters:

```bash
# Generate 20 items at level 15 with exponential scaling
cargo run --example loot_generator -- \
  --input examples/test_data.toml \
  --output output.json \
  --num-items 20 \
  --base-level 15.0 \
  --level-variance 8.0 \
  --affix-chance 0.5 \
  --exponential \
  --scaling-factor 1.2
```

### Command-Line Options

**Required:**
- `-i, --input <INPUT>` - Path to TOML configuration file
- `-o, --output <OUTPUT>` - Path where generated items will be saved
- `-n, --num-items <NUM_ITEMS>` - Number of items to generate

**Optional (with defaults):**
- `-b, --base-level <BASE_LEVEL>` - Average item level (default: 10.0)
- `-v, --level-variance <LEVEL_VARIANCE>` - Range around base level (default: 5.0)
- `-a, --affix-chance <AFFIX_CHANCE>` - Probability of affixes 0.0-1.0 (default: 0.75)
- `--exponential` - Use exponential scaling instead of linear (default: linear)
- `-s, --scaling-factor <SCALING_FACTOR>` - Attribute scaling multiplier (default: 1.0)

View all options with `--help`:

```bash
cargo run --example loot_generator -- --help
```

See `examples/test_data.toml` for a complete, working configuration example with multiple item types, attributes, and affixes.

## Features

- **Weighted Random Selection** - Quality tiers, item types, subtypes, affixes
- **Attribute System** - Attributes scale with item level (linear/exponential)
- **Affix System** - Prefixes and suffixes with configurable attributes
- **TOML Configuration** - Human-readable TOML input files (easy to edit manually)
- **JSON Output** - Generated items serialize to clean JSON

## Usage Examples

### Using as a Rust Library

#### Option 1: Load Configuration from TOML File

The simplest approach - load a TOML configuration file:

```rust
use praeda::*;

fn main() -> Result<()> {
    let mut gen = PraedaGenerator::new();

    // Load configuration from TOML file
    gen.load_toml_file("examples/test_data.toml")?;

    // Generate loot with options
    let options = GeneratorOptions {
        number_of_items: 5,
        base_level: 10.0,
        level_variance: 5.0,
        affix_chance: 0.25,
        linear: true,
        scaling_factor: 1.0,
    };

    let items = gen.generate_loot(
        &options,
        &GeneratorOverrides::empty(),
        "main"
    )?;

    // Use the generated items
    for item in items {
        println!("{} [{}] {}", item.get_quality(), item.get_subtype(), item.get_name());
    }

    Ok(())
}
```

#### Option 2: Programmatic Configuration

For dynamic configuration without TOML files:

```rust
use praeda::*;

fn main() -> Result<()> {
    let mut gen = PraedaGenerator::new();

    // Configure qualities (with weights for random selection)
    gen.set_quality_data("common".to_string(), 100);
    gen.set_quality_data("uncommon".to_string(), 60);
    gen.set_quality_data("rare".to_string(), 30);

    // Configure item types
    gen.set_item_type("weapon".to_string(), 1);
    gen.set_item_subtype("weapon".to_string(), "sword".to_string(), 1);
    gen.set_item_subtype("weapon".to_string(), "axe".to_string(), 1);

    // Configure attributes
    gen.set_attribute(
        "weapon".to_string(),
        "".to_string(),
        ItemAttribute::new(
            "attack_damage".to_string(),
            5.0,
            1.0,
            10.0,
            true,
        ),
    );

    // Set item names for this type/subtype
    gen.set_item(
        "weapon".to_string(),
        "sword".to_string(),
        vec!["longsword".to_string(), "shortsword".to_string()],
    );

    // Generate loot with options
    let options = GeneratorOptions {
        number_of_items: 5,
        base_level: 10.0,
        level_variance: 5.0,
        affix_chance: 0.25,
        linear: true,
        scaling_factor: 1.0,
    };

    let items = gen.generate_loot(
        &options,
        &GeneratorOverrides::empty(),
        "main"
    )?;

    // Use the generated items
    for item in items {
        println!("{} [{}] {}", item.get_quality(), item.get_subtype(), item.get_name());
    }

    Ok(())
}
```

### Configuration File Format

Praeda uses TOML files for configuration, which are human-readable and easy to edit. The `config.toml` file defines all item types, attributes, affixes, and quality tiers.

**Basic structure:**

```toml
# Quality tiers with their weights (frequency)
[quality_data]
common = 100
uncommon = 60
rare = 30
epic = 9
legendary = 1

# Item types and their subtypes
[[item_types]]
item_type = "weapon"
weight = 1
[item_types.subtypes]
one-handed = 1
two-handed = 1

[[item_types]]
item_type = "armor"
weight = 1
[item_types.subtypes]
chest = 1
head = 1
legs = 1
feet = 1
hands = 1
shoulders = 1

# Item attributes by type/subtype
[[item_attributes]]
item_type = "weapon"
subtype = ""
[[item_attributes.attributes]]
name = "attack_damage"
initial_value = 1.0
min = 1.0
max = 5.0
required = true

# Item names by type and subtype
[[item_list]]
item_type = "weapon"
subtype = "one-handed"
names = ["sword", "axe", "mace", "dagger"]

# Affixes (prefixes and suffixes)
[[item_affixes]]
item_type = "weapon"
subtype = ""
[[item_affixes.prefixes]]
name = "sharp"
[[item_affixes.prefixes.attributes]]
name = "attack_damage"
initial_value = 10.0

[[item_affixes.suffixes]]
name = "of the bear"
[[item_affixes.suffixes.attributes]]
name = "strength_requirement"
initial_value = 10.0
```

See `examples/test_data.toml` for a complete, working example with all item types, attributes, and affixes configured.

## Core Types

### Item
Complete generated item with quality, type, subtype, affixes, and attributes.

### ItemAttribute
Individual attribute with value, min/max, and optional metadata. Scales based on level.

### Affix
Prefix or suffix with a collection of attributes that apply to items.

### GeneratorOptions
Configuration for generation:
- `number_of_items`: How many items to generate (default: 1)
- `base_level`: Average item level (default: 1.0)
- `level_variance`: Range around base level (default: 1.0)
- `affix_chance`: Probability of applying optional attributes 0.0-1.0 (default: 0.25)
- `linear`: Whether scaling is linear or exponential (default: true)
- `scaling_factor`: Multiplier for attribute scaling (default: 1.0)

You can create options using `GeneratorOptions::default()` and customize fields:

```rust
let options = GeneratorOptions {
    number_of_items: 5,
    ..Default::default()
};
```

Or use the default configuration directly:

```rust
let items = gen.generate_loot(&GeneratorOptions::default(), &GeneratorOverrides::empty(), "main")?;
```

### GeneratorOverrides
Override specific generation choices:
- `quality_override`: Force specific quality
- `type_override`: Force specific type
- `subtype_override`: Force specific subtype

## Metadata System

Praeda supports metadata at two levels to extend item properties beyond standard attributes:

### Subtype-Level Metadata
Store arbitrary key-value data that applies to all items of a specific type/subtype combination. Useful for category-wide properties:

```rust
gen.set_subtype_metadata(
    "weapon".to_string(),
    "sword".to_string(),
    "armor_penetration".to_string(),
    json!(0.15),
);

// Later, retrieve it
if let Some(metadata) = gen.get_subtype_metadata("weapon", "sword") {
    if let Some(penetration) = metadata.get("armor_penetration") {
        println!("Armor Penetration: {}", penetration);
    }
}
```

### Per-Item Metadata
Store metadata for specific item instances. Useful for item-specific traits or properties:

```rust
gen.set_item_metadata(
    "weapon".to_string(),
    "sword".to_string(),
    "longsword".to_string(),
    "legendary_rarity".to_string(),
    json!(true),
);

// Later, retrieve it
if let Some(metadata) = gen.get_item_metadata("weapon", "sword", "longsword") {
    if let Some(is_legendary) = metadata.get("legendary_rarity") {
        println!("Is Legendary: {}", is_legendary);
    }
}
```

Both metadata types support any JSON-serializable value, giving you full flexibility to extend the item system with custom properties.


## CLI Example

Praeda includes a command-line tool that demonstrates the library:

```bash
# View help
cargo run --example loot_generator -- --help

# Generate items with defaults
cargo run --example loot_generator -- \
  --input examples/test_data.toml \
  --output loot.json \
  --num-items 10

# Custom parameters
cargo run --example loot_generator -- \
  --input examples/test_data.toml \
  --output loot.json \
  --num-items 20 \
  --base-level 15.0 \
  --level-variance 8.0 \
  --affix-chance 0.5 \
  --exponential \
  --scaling-factor 1.2
```

## API Methods

### Configuration
- `set_quality_data(quality, weight)` - Add quality tier
- `set_item_type(type_name, weight)` - Add item type
- `set_item_subtype(type_name, subtype, weight)` - Add subtype
- `set_attribute(type_name, subtype, attribute: ItemAttribute)` - Add attribute to item type/subtype
- `set_item(type_name, subtype, names)` - Set item names
- `set_affix(type_name, subtype)` - Create affix slot
- `set_affix_attribute(type_name, subtype, is_prefix, affix_name, attribute: ItemAttribute)` - Add attribute to prefix/suffix
- `set_subtype_metadata(type_name, subtype, key, value)` - Add metadata at subtype level
- `set_item_metadata(type_name, subtype, item_name, key, value)` - Add metadata for specific item

### Generation
- `generate_loot(options, overrides, key)` -> Result<Vec<Item>>
- `generate_loot_json(...)` -> Result<String>

### Persistence
- `save_data()` -> Result<String>
- `save_data_to_file(path)` -> Result<()>
- `load_data(json_str)` -> Result<()>
- `load_data_from_file(path)` -> Result<()>
- `load_data_toml(toml_str)` -> Result<()>` - Load configuration from TOML string
- `load_data_toml_from_file(path)` -> Result<()>` - Load configuration from TOML file

### Queries
- `get_loot(key)` -> Vec<Item>
- `get_loot_json(key)` -> Result<String>
- `get_prefixes(type, subtype)` -> Vec<Affix>
- `get_suffixes(type, subtype)` -> Vec<Affix>
- `get_subtype_metadata(type, subtype)` -> Option<HashMap<String, Value>>
- `get_item_metadata(type, subtype, item_name)` -> Option<HashMap<String, Value>>
- `has_quality(quality)` -> bool
- `has_item_type(type)` -> bool
- `has_item_subtype(type, subtype)` -> bool
- `has_attribute(type, subtype, attr_name)` -> bool

## Data Structure

The generator maintains internal state:
- `quality_data: HashMap<String, i32>` - Quality tiers and weights
- `item_types: Vec<ItemType>` - Item types with subtypes
- `item_list: HashMap<(String, String), Vec<String>>` - Names per type/subtype
- `item_attributes: HashMap<(String, String), Vec<ItemAttribute>>` - Attributes per type/subtype
- `item_affixes: HashMap<(String, String), (Vec<Affix>, Vec<Affix>)>` - Prefixes/suffixes
- `subtype_metadata: HashMap<(String, String), HashMap<String, Value>>` - Metadata per type/subtype
- `item_name_metadata: HashMap<(String, String, String), HashMap<String, Value>>` - Metadata per specific item
- `loot_list: HashMap<String, Vec<Item>>` - Generated loot history

## Serialization Format

Items serialize to JSON with full attribute information:

```json
{
  "name": "sword",
  "quality": "rare",
  "type": "weapon",
  "subtype": "one-handed",
  "prefix": {
    "name": "sharp",
    "attributes": [
      {
        "name": "attack_damage",
        "initial_value": 10.0,
        "min": 0.0,
        "max": 0.0,
        "required": false,
        "meta_data": {}
      }
    ]
  },
  "suffix": {...},
  "attributes": {...}
}
```

## Dependencies

- `serde` - Serialization framework
- `serde_json` - JSON support
- `toml` - TOML support
- `rand` - Random number generation
- `thiserror` - Error handling
