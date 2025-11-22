# Praeda

A procedural loot generator library written in Rust with FFI bindings (and examples) for C++, C#, and the [Godot Engine](https://godotengine.org/) v4.2+.

## Features

- Configurable quality tiers with weighted probabilities
- Multiple item types and subtypes
- Attribute scaling based on item level
- Named item generation with customizable prefixes and suffixes
- Affix system for dynamic item modifiers
- Foreign Function Interface (FFI) bindings for C++ and C#

## Quick Start

### Rust

```rust
use praeda::{PraedaGenerator, GeneratorOptions, GeneratorOverrides, ItemAttribute};

let mut generator = PraedaGenerator::new();

// Define quality tiers
generator.set_quality_data("common", 100);
generator.set_quality_data("rare", 30);

// Define item types
generator.set_item_type("weapon", 1);
generator.set_item_subtype("weapon", "sword", 1);
generator.set_item("weapon", "sword",
                   vec!["longsword", "shortsword"]);

// Define attributes
generator.set_attribute("weapon", "",
                       ItemAttribute::new("damage", 10.0, 5.0, 20.0, true));

let options = GeneratorOptions {
    number_of_items: 5,
    base_level: 15.0,
    level_variance: 5.0,
    affix_chance: 0.75,
    linear: true,
    scaling_factor: 1.0,
};

let items = generator.generate_loot(&options, &GeneratorOverrides::empty(), "loot")?;

for item in items {
    println!("{}: {}", item.quality, item.name);
}
```

### Advanced Configuration (Rust)

```rust
use praeda::{PraedaGenerator, GeneratorOptions, GeneratorOverrides, ItemAttribute};

let mut generator = PraedaGenerator::new();

// Define quality tiers
generator.set_quality_data("common", 100);
generator.set_quality_data("uncommon", 60);
generator.set_quality_data("rare", 30);
generator.set_quality_data("legendary", 5);

// Define item types with weights
generator.set_item_type("weapon", 70);
generator.set_item_type("armor", 30);

// Define item subtypes for weapons
generator.set_item_subtype("weapon", "sword", 40);
generator.set_item_subtype("weapon", "axe", 30);
generator.set_item_subtype("weapon", "bow", 30);

// Define item subtypes for armor
generator.set_item_subtype("armor", "chest", 50);
generator.set_item_subtype("armor", "helm", 50);

// Set item names for weapons
generator.set_item("weapon", "sword",
                   vec!["longsword", "shortsword", "greatsword"]);
generator.set_item("weapon", "axe",
                   vec!["battleaxe", "handaxe", "greataxe"]);
generator.set_item("weapon", "bow",
                   vec!["longbow", "shortbow", "recurve bow"]);

// Set item names for armor
generator.set_item("armor", "chest",
                   vec!["plate armor", "leather armor", "chain mail"]);
generator.set_item("armor", "helm",
                   vec!["iron helm", "leather helm", "great helm"]);

// Define attributes for weapons
generator.set_attribute("weapon", "",
                       ItemAttribute::new("damage", 15.0, 5.0, 30.0, true));
generator.set_attribute("weapon", "",
                       ItemAttribute::new("attack_speed", 1.0, 0.3, 2.0, true));

// Define attributes for armor
generator.set_attribute("armor", "",
                       ItemAttribute::new("defense", 10.0, 2.0, 20.0, true));
generator.set_attribute("armor", "",
                       ItemAttribute::new("durability", 100.0, 30.0, 150.0, true));

// Add prefix attributes
generator.set_prefix_attribute("weapon", "sword", "sharp",
                             ItemAttribute::new("damage", 5.0, 0.0, 15.0, true));

// Add suffix attributes
generator.set_suffix_attribute("weapon", "sword", "of awesomeness",
                             ItemAttribute::new("attack_speed", 0.5, 0.0, 1.5, true));

// Generate items with affixes
let options = GeneratorOptions {
    number_of_items: 10,
    base_level: 20.0,
    level_variance: 5.0,
    affix_chance: 0.8,
    linear: true,
    scaling_factor: 1.5,
};

let items = generator.generate_loot(&options, &GeneratorOverrides::empty(), "weapons")?;

for item in items {
    let level = item.attributes
        .get("level")
        .map(|attr| attr.initial_value as i32)
        .unwrap_or(0);
    println!(
        "[{}] {} {} (Level: {}) - Attributes: {:?}",
        item.quality,
        item.name,
        item.subtype,
        level,
        item.attributes
    );
}
```

### C++

```cpp
#include "praeda.hpp"
#include <iostream>

int main() {
    auto gen = praeda::Generator::create();

    // Define quality tiers
    gen->set_quality_data("common", 100);
    gen->set_quality_data("rare", 30);

    // Define item types
    gen->set_item_type("weapon", 1);

    // Define item subtypes
    gen->set_item_subtype("weapon", "sword", 1);

    // Define item names
    gen->set_item_names("weapon", "sword", {"longsword", "shortsword"});

    // Define attributes
    praeda::ItemAttribute damage("damage", 10.0, 5.0, 20.0, true);
    gen->set_attribute("weapon", "", damage);

    // Generate loot
    praeda::GenerationOptions options;
    options.number_of_items = 5;
    options.base_level = 15.0;
    options.level_variance = 5.0;
    options.affix_chance = 0.75;
    options.linear = true;
    options.scaling_factor = 1.0;

    auto items = gen->generate_loot(options);

    for (const auto& item : items) {
        std::cout << item.quality << ": " << item.name << std::endl;
    }

    return 0;
}
```

### C#

```csharp
using System;
using Praeda;

class Program {
    static void Main() {
        using var gen = new PraedaGenerator();

        // Define quality tiers
        gen.SetQualityData("common", 100);
        gen.SetQualityData("rare", 30);

        // Define item types
        gen.SetItemType("weapon", 1);

        // Define item subtypes
        gen.SetItemSubtype("weapon", "sword", 1);

        // Define item names
        gen.SetItemNames("weapon", "sword", new[] { "longsword", "shortsword" });

        // Define attributes
        gen.SetAttribute("weapon", "", "damage", 10.0, 5.0, 20.0, true);

        // Generate loot
        var options = new GenerationOptions {
            NumberOfItems = 5,
            BaseLevel = 15.0,
            LevelVariance = 5.0,
            AffixChance = 0.75,
            Linear = true,
            ScalingFactor = 1.0
        };

        var items = gen.GenerateLoot(options);

        foreach (var item in items) {
            Console.WriteLine($"{item.Quality}: {item.Name}");
        }
    }
}
```

### Godot

See `examples/godot`.

## Building

### Rust Library

```bash
cargo build --release
```

### C++ Examples

```bash
cd examples/cpp
mkdir build && cd build
cmake ..
make
./test_praeda
```

### C# Examples

```bash
cd examples/csharp
dotnet build
dotnet run
```


### Godot Examples

The Godot binding crate is built when the primary rust library is built. See `examples/godot/README.md`.

## License

This project is licensed under the LGPL-3.0-or-later license. See LICENSE file for details.

## Contributing

Contributions are welcome! Please ensure all tests pass before submitting pull requests:

```bash
cargo test
```

Test coverage must meet a minimum of 80%.

