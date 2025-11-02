# Praeda

This library provides a flexible system for randomly generating items with affixes, attributes, and quality tiers.

## Quick Start

Add to your `Cargo.toml`:
```toml
[dependencies]
praeda = { git = "https://www.github.com/EddieDover/praeda.git", branch = "master" }
```

Build and test:
```bash
# Build the library
cargo build --lib

# Run tests
cargo test
```

## Features

- ✅ **Weighted Random Selection** - Quality tiers, item types, subtypes, affixes
- ✅ **Attribute System** - Attributes scale with item level (linear/exponential)
- ✅ **Affix System** - Prefixes and suffixes with configurable attributes
- ✅ **TOML Configuration** - Human-readable TOML input files (easy to edit manually)
- ✅ **JSON Output** - Generated items serialize to clean JSON
- ✅ **Error Handling** - Proper Result<T> error types
- ✅ **Thread-Safe** - Uses rand::thread_rng() for safety

## Usage Examples

### Using as a Rust Library

Here's how to use Praeda in your Rust project:

```rust
use praeda::*;
use std::collections::HashMap;

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
        "attack_damage".to_string(),
        5.0, 1.0, 10.0, true, HashMap::new(),
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
- `number_of_items`: How many items to generate
- `base_level`: Average item level
- `level_variance`: Range around base level
- `affix_chance`: Probability of applying optional attributes (0.0-1.0)
- `linear`: Whether scaling is linear or exponential
- `scaling_factor`: Multiplier for attribute scaling

### GeneratorOverrides
Override specific generation choices:
- `quality_override`: Force specific quality
- `type_override`: Force specific type
- `subtype_override`: Force specific subtype

## API Methods

### Configuration
- `set_quality_data(quality, weight)` - Add quality tier
- `set_item_type(type_name, weight)` - Add item type
- `set_item_subtype(type_name, subtype, weight)` - Add subtype
- `set_attribute(...)` - Add attribute to items
- `set_item(type_name, subtype, names)` - Set item names
- `set_affix(type_name, subtype)` - Create affix slot
- `set_affix_attribute(...)` - Add attribute to prefix/suffix

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
- `has_quality(quality)` -> bool
- `has_item_type(type)` -> bool
- `has_item_subtype(type, subtype)` -> bool

## Data Structure

The generator maintains internal state:
- `quality_data: HashMap<String, i32>` - Quality tiers and weights
- `item_types: Vec<ItemType>` - Item types with subtypes
- `item_list: HashMap<(String, String), Vec<String>>` - Names per type/subtype
- `item_attributes: HashMap<(String, String), Vec<ItemAttribute>>` - Attributes per type/subtype
- `item_affixes: HashMap<(String, String), (Vec<Affix>, Vec<Affix>)>` - Prefixes/suffixes
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
