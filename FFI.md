# Praeda Foreign Function Interface (FFI)

This document describes how to use Praeda from C++, C#, and other languages via the Foreign Function Interface (FFI).

## Overview

Praeda exposes a C-compatible FFI layer that allows usage from any language that can call C functions. All data is exchanged through **C-compatible structs and simple status codes**

## Building for FFI

### Prerequisites

- Rust 1.70+
- For C++: C++17 compiler
- For C#: .NET 6.0+

### Build Steps

1. **Build the dynamic library:**

```bash
cargo build --release
```

This creates:
- Linux: `target/release/libpraeda.so`
- macOS: `target/release/libpraeda.dylib`
- Windows: `target/release/praeda.dll`

2. **Copy to your project:**

```bash
# C++ project
cp target/release/libpraeda.so /path/to/cpp/project/lib/

# C# project
cp target/release/praeda.dll /path/to/csharp/project/

# Or on macOS/Linux, set library path:
export LD_LIBRARY_PATH="/path/to/praeda/target/release:$LD_LIBRARY_PATH"
```

## C++ Usage

### Setup

1. Copy the header:
```bash
cp include/praeda.hpp /path/to/your/project/include/
```

2. Link the library in your CMakeLists.txt or build system:
```cmake
find_library(PRAEDA_LIB praeda PATHS ${CMAKE_SOURCE_DIR}/lib)
target_link_libraries(your_target ${PRAEDA_LIB})
```

### Basic Example

```cpp
#include "praeda.hpp"
#include <iostream>

int main() {
    try {
        // Create generator
        auto gen = praeda::Generator::create();

        // Load configuration from TOML
        gen->load_toml_string(R"(
            [quality_data]
            common = 100
            uncommon = 60
            rare = 30

            [[item_types]]
            item_type = "weapon"
            weight = 1
            [item_types.subtypes]
            sword = 1
        )");

        // Generate items using native C++ types
        praeda::GenerationOptions options{
            .number_of_items = 10,
            .base_level = 15.0,
            .level_variance = 5.0,
            .affix_chance = 0.75,
            .linear = true,
            .scaling_factor = 1.0
        };

        auto items = gen->generate_loot(options);

        std::cout << "Generated " << items.size() << " items:" << std::endl;
        for (const auto& item : items) {
            std::cout << "[" << item.quality << "] "
                      << item.type << "/" << item.subtype
                      << " - " << item.name << std::endl;
        }

    } catch (const praeda::Exception& e) {
        std::cerr << "Praeda error: " << e.what() << std::endl;
        return 1;
    }
    return 0;
}
```

### Programmatic Configuration Example (No TOML Required)

You can also configure the generator programmatically without needing TOML files:

```cpp
#include "praeda.hpp"
#include <iostream>

int main() {
    try {
        // Create generator
        auto gen = praeda::Generator::create();

        // Configure programmatically (no TOML needed!)

        // Set qualities
        gen->set_quality_data("common", 100);
        gen->set_quality_data("uncommon", 60);
        gen->set_quality_data("rare", 30);

        // Set item types
        gen->set_item_type("weapon", 2);
        gen->set_item_type("armor", 1);

        // Set subtypes
        gen->set_item_subtype("weapon", "sword", 3);
        gen->set_item_subtype("weapon", "axe", 2);
        gen->set_item_subtype("armor", "chest", 1);

        // Set attributes
        gen->set_attribute("weapon", "", "damage", 15.0, 5.0, 30.0, true);
        gen->set_attribute("armor", "", "defense", 10.0, 2.0, 20.0, true);

        // Set item names
        gen->set_item_names("weapon", "sword", {"longsword", "shortsword"});
        gen->set_item_names("weapon", "axe", {"battleaxe"});
        gen->set_item_names("armor", "chest", {"plate_armor", "leather_armor"});

        // Generate items using native C++ types
        praeda::GenerationOptions options{
            .number_of_items = 10,
            .base_level = 15.0,
            .level_variance = 5.0,
            .affix_chance = 0.75,
            .linear = true,
            .scaling_factor = 1.0
        };

        auto items = gen->generate_loot(options);

        std::cout << "Generated " << items.size() << " items:" << std::endl;
        for (const auto& item : items) {
            std::cout << "[" << item.quality << "] "
                      << item.type << "/" << item.subtype
                      << " - " << item.name << std::endl;
        }

    } catch (const praeda::Exception& e) {
        std::cerr << "Praeda error: " << e.what() << std::endl;
        return 1;
    }
    return 0;
}
```

### API Reference

#### `Generator::create()`
Create a new generator instance.
- Returns: `unique_ptr<Generator>`
- Throws: `praeda::Exception` on failure

#### Configuration Methods

##### `gen->set_quality_data(const std::string& quality, int weight)`
Set a quality tier with weight.
- Parameters: quality name, weight (higher = more likely)
- Throws: `praeda::Exception` on error

##### `gen->set_item_type(const std::string& type, int weight)`
Set an item type with weight.
- Parameters: type name, weight
- Throws: `praeda::Exception` on error

##### `gen->set_item_subtype(const std::string& type, const std::string& subtype, int weight)`
Set an item subtype with weight.
- Parameters: type name, subtype name, weight
- Throws: `praeda::Exception` on error

##### `gen->set_attribute(const std::string& type, const std::string& subtype, const ItemAttribute&)`
Set an attribute for a type/subtype.
- Parameters: type name, subtype name (empty string "" for type-wide), attribute
- Throws: `praeda::Exception` on error

##### `gen->set_item_names(const std::string& type, const std::string& subtype, const std::vector<std::string>& names)`
Set item names for a type/subtype.
- Parameters: type name, subtype name, vector of names
- Throws: `praeda::Exception` on error

#### `gen->load_toml_string(const std::string&)`
Load configuration from TOML string (alternative to programmatic methods).
- Throws: `praeda::Exception` on parse error

#### `gen->generate_loot(const GenerationOptions& options)`
Generate loot items.
- Parameter: Generation options struct (number_of_items, base_level, etc.)
- Returns: `std::vector<Item>` of generated items
- Throws: `praeda::Exception` on error

#### `gen->has_quality(const std::string&)`
Check if a quality exists.
- Returns: `bool`

#### `gen->info()`
Get generator information (version string).
- Returns: Version string (e.g., "0.1.5")

#### `praeda::version()`
Get library version.
- Returns: Version string (e.g., "0.1.5")

## C# Usage

### Setup

1. Copy the wrapper:
```bash
cp examples/csharp/PraedaGenerator.cs /path/to/your/project/
```

2. Copy the shared library and set library path:
```bash
# Set LD_LIBRARY_PATH before running
export LD_LIBRARY_PATH=/path/to/praeda/target/release:$LD_LIBRARY_PATH

# Or on macOS
export DYLD_LIBRARY_PATH=/path/to/praeda/target/release:$DYLD_LIBRARY_PATH
```

3. The library name is automatically detected:
```csharp
// In PraedaGenerator.cs:
private const string DllName = "praeda";  // Automatically loads correct library per platform
// Windows: praeda.dll
// Linux: libpraeda.so
// macOS: libpraeda.dylib
```

### Basic Example

```csharp
using System;
using Praeda;

class Program {
    static void Main() {
        try {
            // Create generator
            using var gen = new PraedaGenerator();

            // Load configuration from TOML
            gen.LoadTomlString(@"
                [quality_data]
                common = 100
                uncommon = 60
                rare = 30

                [[item_types]]
                item_type = ""weapon""
                weight = 1
                [item_types.subtypes]
                sword = 1
            ");

            // Generate items using native C# types
            var options = new GenerationOptions {
                NumberOfItems = 10,
                BaseLevel = 15.0,
                LevelVariance = 5.0,
                AffixChance = 0.75,
                Linear = true,
                ScalingFactor = 1.0
            };

            var items = gen.GenerateLoot(options);

            Console.WriteLine($"Generated {items.Count} items:");
            foreach (var item in items) {
                Console.WriteLine($"[{item.Quality}] {item.Type}/{item.Subtype} - {item.Name}");
            }

        } catch (Exception ex) {
            Console.Error.WriteLine($"Error: {ex.Message}");
            return 1;
        }
        return 0;
    }
}
```

### Programmatic Configuration Example (No TOML Required)

You can also configure the generator programmatically without needing TOML files:

```csharp
using System;
using Praeda;

class Program {
    static void Main() {
        try {
            // Create generator
            using var gen = new PraedaGenerator();

            // Configure programmatically (no TOML needed!)

            // Set qualities
            gen.SetQualityData("common", 100);
            gen.SetQualityData("uncommon", 60);
            gen.SetQualityData("rare", 30);

            // Set item types
            gen.SetItemType("weapon", 2);
            gen.SetItemType("armor", 1);

            // Set subtypes
            gen.SetItemSubtype("weapon", "sword", 3);
            gen.SetItemSubtype("weapon", "axe", 2);
            gen.SetItemSubtype("armor", "chest", 1);

            // Set attributes
            gen.SetAttribute("weapon", "", "damage", 15.0, 5.0, 30.0, true);
            gen.SetAttribute("armor", "", "defense", 10.0, 2.0, 20.0, true);

            // Set item names
            gen.SetItemNames("weapon", "sword", new[] { "longsword", "shortsword" });
            gen.SetItemNames("weapon", "axe", new[] { "battleaxe" });
            gen.SetItemNames("armor", "chest", new[] { "plate_armor", "leather_armor" });

            // Generate items using native C# types
            var options = new GenerationOptions {
                NumberOfItems = 10,
                BaseLevel = 15.0,
                LevelVariance = 5.0,
                AffixChance = 0.75,
                Linear = true,
                ScalingFactor = 1.0
            };

            var items = gen.GenerateLoot(options);

            Console.WriteLine($"Generated {items.Count} items:");
            foreach (var item in items) {
                Console.WriteLine($"[{item.Quality}] {item.Type}/{item.Subtype} - {item.Name}");
            }

        } catch (Exception ex) {
            Console.Error.WriteLine($"Error: {ex.Message}");
            return 1;
        }
        return 0;
    }
}
```

### API Reference

#### `new PraedaGenerator()`
Create a new generator instance.
- Throws: `InvalidOperationException` on failure
- Note: Use `using` statement for automatic cleanup

#### `gen.LoadTomlString(string tomlContent)`
Load configuration from TOML string.
- Throws: `InvalidOperationException` on error

#### `gen.GenerateLoot(GenerationOptions options)`
Generate loot items.
- Parameter: Generation options struct (NumberOfItems, BaseLevel, etc.)
- Returns: `List<Item>` of generated items
- Throws: `InvalidOperationException` on error

#### `gen.HasQuality(string quality)`
Check if a quality exists.
- Returns: `bool`
- Throws: `InvalidOperationException` on error

#### `gen.GetInfo()`
Get generator information (version string).
- Returns: Version string

## Generation Options

Both C++ and C# use strongly-typed `GenerationOptions` structs:

### C++
```cpp
praeda::GenerationOptions options{
    .number_of_items = 10,
    .base_level = 15.0,
    .level_variance = 5.0,
    .affix_chance = 0.75,
    .linear = true,
    .scaling_factor = 1.0
};
```

### C#
```csharp
var options = new GenerationOptions {
    NumberOfItems = 10,
    BaseLevel = 15.0,
    LevelVariance = 5.0,
    AffixChance = 0.75,
    Linear = true,
    ScalingFactor = 1.0
};
```

### Option Descriptions

- **NumberOfItems** (uint): How many items to generate (default: 1)
- **BaseLevel** (float): Average item level (default: 10.0)
- **LevelVariance** (float): Range around base level (default: 5.0)
- **AffixChance** (float): Probability of affixes 0.0-1.0 (default: 0.75)
- **Linear** (bool): Use linear scaling if true, exponential if false (default: true)
- **ScalingFactor** (float): Attribute scaling multiplier (default: 1.0)

## Configuration Format

Configuration is provided as TOML. See [examples/test_data.toml](examples/test_data.toml) for a complete example.

Basic structure:
```toml
[quality_data]
common = 100
uncommon = 60
rare = 30

[[item_types]]
item_type = "weapon"
weight = 1
[item_types.subtypes]
sword = 1
axe = 1

[[item_attributes]]
item_type = "weapon"
subtype = ""
[[item_attributes.attributes]]
name = "attack_damage"
initial_value = 5.0
min = 1.0
max = 10.0
required = true
```

## Error Handling

### C++

All operations that can fail throw `praeda::Exception`:

```cpp
try {
    gen->load_toml_string(config);
} catch (const praeda::Exception& e) {
    std::cerr << "Error: " << e.what() << std::endl;
}
```

### C#

Operations throw `InvalidOperationException` with descriptive messages:

```csharp
try {
    gen.LoadTomlString(config);
} catch (InvalidOperationException ex) {
    Console.Error.WriteLine($"Error: {ex.Message}");
}
```

## Performance Considerations

- **JSON Serialization**: ~1-5ms overhead per generation
- **Memory**: Each generator instance holds configuration in memory
- **Thread Safety**: Each thread should have its own generator instance
- **Optimization**: For batch operations, reuse the same generator instance

## Platform-Specific Notes

### Linux

The library is built as `libpraeda.so`. Set `LD_LIBRARY_PATH`:

```bash
export LD_LIBRARY_PATH="/path/to/praeda/target/release:$LD_LIBRARY_PATH"
```

### macOS

The library is built as `libpraeda.dylib`. You may need to:

```bash
install_name_tool -id @loader_path/libpraeda.dylib libpraeda.dylib
```

### Windows

The library is built as `praeda.dll`. Place it in the same directory as your executable or on PATH.

## Testing the FFI

### C++ Test

```bash
cd /path/to/praeda/bindings/cpp
cmake .
make
./test_praeda
```

### C# Test

```bash
cd /path/to/praeda/bindings/csharp
dotnet new console -n PraedaTest
cp ../../target/release/praeda.dll .
dotnet add reference PraedaGenerator.cs
dotnet run
```

## Troubleshooting

### "Cannot find library"
- Ensure the shared library is in the system library path
- Check the library file exists in `target/release/`
- On Linux: set `LD_LIBRARY_PATH`
- On macOS: use `install_name_tool` or `@rpath`
- On Windows: place DLL in the executable directory

### "Undefined symbol"
- Ensure you built with `cargo build --release`
- Check the FFI header matches the library version
- Try rebuilding: `cargo clean && cargo build --release`

### Type Errors or Crashes
- Ensure struct fields are properly initialized
- Check that all required configuration is set before generating loot

### Memory Leaks
- Ensure generators are properly freed (use RAII/using patterns)
- All string pointers from FFI must be freed by the library
- Don't manually free strings; use wrapper destructors

## Contributing FFI Improvements

To extend the FFI:

1. Add new functions to `src/ffi.rs`
2. Mark with `#[no_mangle]` and `extern "C"`
3. Update C++ wrapper in `include/praeda.hpp`
4. Update C# wrapper in `bindings/PraedaGenerator.cs`
5. Add documentation and examples
6. Test on all platforms

## See Also

- [README.md](README.md) - Library usage overview
- [CONTRIBUTING.md](CONTRIBUTING.md) - Development guidelines
- [examples/test_data.toml](examples/test_data.toml) - Configuration example
- [examples/loot_generator.rs](examples/loot_generator.rs) - Rust example
