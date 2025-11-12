# Praeda Foreign Function Interface (FFI)

This document describes how to use Praeda from C++, C#, and other languages via the Foreign Function Interface (FFI).

## Overview

Praeda exposes a C-compatible FFI layer that allows usage from any language that can call C functions. All data is exchanged through JSON strings for simplicity and language independence.

```
Praeda Rust Library
    ↓
[C FFI Layer] (src/ffi.rs)
    ↙              ↘
C++ Wrapper     C# Wrapper
(include/praeda.hpp) (bindings/PraedaGenerator.cs)
```

## Building for FFI

### Prerequisites

- Rust 1.70+
- For C++: C++17 compiler, nlohmann/json
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
#include <nlohmann/json.hpp>
#include <iostream>

using json = nlohmann::json;

int main() {
    try {
        // Create generator
        auto gen = praeda::Generator::create();

        // Load configuration
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

        // Generate items
        json options = {
            {"number_of_items", 10},
            {"base_level", 15.0},
            {"level_variance", 5.0},
            {"affix_chance", 0.75},
            {"linear", true},
            {"scaling_factor", 1.0}
        };

        std::string items_json = gen->generate_loot(options.dump());
        json items = json::parse(items_json);

        std::cout << "Generated " << items.size() << " items:" << std::endl;
        std::cout << items.dump(2) << std::endl;

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
#include <nlohmann/json.hpp>
#include <iostream>

using json = nlohmann::json;

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
        gen->set_attribute("weapon", "",
            praeda::ItemAttribute("damage", 15.0, 5.0, 30.0, true));
        gen->set_attribute("armor", "",
            praeda::ItemAttribute("defense", 10.0, 2.0, 20.0, true));

        // Set item names
        gen->set_item_names("weapon", "sword", {"longsword", "shortsword"});
        gen->set_item_names("weapon", "axe", {"battleaxe"});
        gen->set_item_names("armor", "chest", {"plate_armor", "leather_armor"});

        // Generate items
        json options = {
            {"number_of_items", 10},
            {"base_level", 15.0},
            {"level_variance", 5.0},
            {"affix_chance", 0.75},
            {"linear", true},
            {"scaling_factor", 1.0}
        };

        std::string items_json = gen->generate_loot(options.dump());
        json items = json::parse(items_json);

        std::cout << "Generated " << items.size() << " items:" << std::endl;
        std::cout << items.dump(2) << std::endl;

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

#### `gen->generate_loot(const std::string& options_json)`
Generate loot items.
- Parameter: JSON string with generation options
- Returns: JSON array string of items
- Throws: `praeda::Exception` on error

#### `gen->get_quality_data()`
Get all quality data as JSON.
- Returns: JSON object mapping quality names to weights

#### `gen->has_quality(const std::string&)`
Check if a quality exists.
- Returns: `bool`

#### `gen->info()`
Get generator information.
- Returns: JSON object with generator stats

#### `praeda::version()`
Get library version.
- Returns: Version string (e.g., "0.1.5")

## C# Usage

### Setup

1. Copy the wrapper:
```bash
cp bindings/PraedaGenerator.cs /path/to/your/project/
```

2. Copy the shared library:
```bash
# Windows
cp target/release/praeda.dll /path/to/your/project/bin/Debug/

# Linux
cp target/release/libpraeda.so /path/to/your/project/bin/Debug/

# macOS
cp target/release/libpraeda.dylib /path/to/your/project/bin/Debug/
```

3. Update library name if needed:
```csharp
// In PraedaGenerator.cs, adjust:
private const string DLL_NAME = "praeda";  // Windows adds .dll automatically
// On Linux: "praeda" loads libpraeda.so
// On macOS: "praeda" loads libpraeda.dylib
```

### Basic Example

```csharp
using System;
using System.Text.Json;
using Praeda;

class Program {
    static void Main() {
        try {
            // Create generator
            using var gen = new PraedaGenerator();

            // Load configuration
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

            // Generate items
            var options = new GenerationOptions {
                NumberOfItems = 10,
                BaseLevel = 15.0,
                LevelVariance = 5.0,
                AffixChance = 0.75,
                Linear = true,
                ScalingFactor = 1.0
            };

            string itemsJson = PraedaHelper.GenerateLoot(gen, options);
            var items = JsonDocument.Parse(itemsJson);

            Console.WriteLine($"Generated {items.RootElement.GetArrayLength()} items:");
            Console.WriteLine(JsonSerializer.Serialize(items,
                new JsonSerializerOptions { WriteIndented = true }));

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
using System.Text.Json;
using System.Collections.Generic;
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

            // Generate items
            var options = new GenerationOptions {
                NumberOfItems = 10,
                BaseLevel = 15.0,
                LevelVariance = 5.0,
                AffixChance = 0.75,
                Linear = true,
                ScalingFactor = 1.0
            };

            string itemsJson = PraedaHelper.GenerateLoot(gen, options);
            var items = JsonDocument.Parse(itemsJson);

            Console.WriteLine($"Generated {items.RootElement.GetArrayLength()} items:");
            Console.WriteLine(JsonSerializer.Serialize(items,
                new JsonSerializerOptions { WriteIndented = true }));

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

#### `gen.LoadTomlFile(string filePath)`
Load configuration from a file.
- Throws: `System.IO.FileNotFoundException` if file not found
- Throws: `InvalidOperationException` on parse error

#### `gen.GenerateLoot(string optionsJson)`
Generate loot items.
- Parameter: JSON string with generation options
- Returns: JSON array string
- Throws: `InvalidOperationException` on error

#### `gen.GetQualityData()`
Get quality data as JSON.
- Returns: JSON object string

#### `gen.HasQuality(string quality)`
Check if a quality exists.
- Returns: `bool`
- Throws: `InvalidOperationException` on error

#### `gen.GetInfo()`
Get generator information.
- Returns: JSON object string

#### `PraedaGenerator.GetVersion()`
Get library version.
- Returns: Version string

#### `PraedaHelper.GenerateLoot(gen, options)`
Convenience method for strongly-typed options.

## Generation Options

Both C++ and C# expect generation options as JSON:

```json
{
  "number_of_items": 10,
  "base_level": 15.0,
  "level_variance": 5.0,
  "affix_chance": 0.75,
  "linear": true,
  "scaling_factor": 1.0
}
```

### Option Descriptions

- **number_of_items** (uint): How many items to generate (default: 1)
- **base_level** (float): Average item level (default: 10.0)
- **level_variance** (float): Range around base level (default: 5.0)
- **affix_chance** (float): Probability of affixes 0.0-1.0 (default: 0.75)
- **linear** (bool): Use linear scaling if true, exponential if false (default: true)
- **scaling_factor** (float): Attribute scaling multiplier (default: 1.0)

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

### JSON Parsing Error
- Ensure JSON is valid before passing to Praeda
- Use `JsonSerializer.Serialize()` in C# for safe conversion
- Use `json::dump()` in C++ after parsing with nlohmann

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
