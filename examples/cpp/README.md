# Praeda C++ Example

This example demonstrates how to use the Praeda loot generation library from C++ through its FFI (Foreign Function Interface).

## Prerequisites

- C++17 or later
- CMake 3.10+
- Rust toolchain (to build the Praeda library)

## Building

### Step 1: Build the Praeda Rust Library

First, build the Praeda library in release mode:

```bash
cd /path/to/praeda
cargo build --release
```

This creates the shared library at `target/release/libpraeda.so` (Linux) or `libpraeda.dylib` (macOS).

### Step 2: Build the C++ Example

Navigate to the example directory and build with CMake:

```bash
cd examples/cpp
mkdir -p build
cd build
cmake ..
make
```

The CMakeLists.txt automatically:
- Finds the praeda library in `../../target/release`
- Includes headers from `../../include`
- Sets up proper library paths for runtime execution

## Running

Execute the compiled test:

```bash
cd examples/cpp/build
./test_praeda
```

Or from the examples/cpp directory:

```bash
LD_LIBRARY_PATH=../../target/release ./build/test_praeda
```

## Expected Output

The test performs 6 main test suites:

1. **Programmatic Configuration** - Sets qualities, item types, subtypes, attributes, and names
2. **Query Methods** - Demonstrates checking if qualities exist
3. **Load Configuration from TOML** - Loads configuration from a TOML string
4. **Loot Generation (Programmatic)** - Generates items using programmatic configuration
5. **Loot Generation (TOML)** - Generates items using TOML configuration
6. **Generator Info** - Retrieves library version information

Example output:
```
=== Praeda C++ FFI Test ===

Creating generator...
✓ Generator created successfully

--- Test 1: Programmatic Configuration ---
Setting qualities...
✓ Qualities set
...

=== All Tests Passed! ===
```

## Code Structure

### Main Components

- **praeda.hpp** - C++ wrapper header containing:
  - C FFI declarations (`extern "C"` block)
  - Native C++ wrapper classes:
    - `ItemAttribute` - Item attribute with value ranges
    - `Affix` - Prefix/suffix with attributes
    - `Item` - Generated item with all properties
    - `GenerationOptions` - Configuration for loot generation
    - `Generator` - Main loot generation engine

- **test_praeda.cpp** - Comprehensive test suite demonstrating:
  - Generator creation and configuration
  - All API methods
  - Error handling
  - Native C++ type usage throughout

## API Usage

### Creating a Generator

```cpp
auto gen = praeda::Generator::create();
```

### Configuring Qualities and Items

```cpp
gen->set_quality_data("common", 100);
gen->set_item_type("weapon", 2);
gen->set_item_subtype("weapon", "sword", 3);
gen->set_attribute("weapon", "sword", "damage", 15.0, 5.0, 30.0, true);
gen->set_item_names("weapon", "sword", {"longsword", "shortsword"});
```

### Loading from TOML

```cpp
std::string toml_content = R"(
[qualities]
common = 100
rare = 30

[items.weapon]
weight = 2

[items.weapon.subtypes.sword]
weight = 3
)";

gen->load_toml_string(toml_content);
```

### Generating Loot

```cpp
praeda::GenerationOptions options{
    .number_of_items = 5,
    .base_level = 15.0,
    .level_variance = 5.0,
    .affix_chance = 0.75,
    .linear = true,
    .scaling_factor = 1.0
};

auto items = gen->generate_loot(options);
for (const auto& item : items) {
    std::cout << "[" << item.quality << "] "
              << item.type << "/" << item.subtype
              << " - " << item.name << std::endl;
}
```

## Error Handling

The C++ wrapper uses exceptions for error handling:

```cpp
try {
    auto gen = praeda::Generator::create();
    gen->set_quality_data("common", 100);
    auto items = gen->generate_loot(options);
} catch (const praeda::Exception& e) {
    std::cerr << "Praeda error: " << e.what() << std::endl;
} catch (const std::exception& e) {
    std::cerr << "Unexpected error: " << e.what() << std::endl;
}
```

## Troubleshooting

### Library Not Found

If CMake can't find the praeda library:
1. Ensure you've built the Rust library: `cargo build --release`
2. Verify the library exists: `ls ../../target/release/libpraeda.*`
3. Check that CMakeLists.txt paths are correct

### Runtime Library Not Found

If you get `libpraeda.so: cannot open shared object file`:

```bash
# Set LD_LIBRARY_PATH before running
export LD_LIBRARY_PATH=/path/to/praeda/target/release:$LD_LIBRARY_PATH
./test_praeda
```

### Compilation Errors

Ensure your compiler supports C++17:

```bash
cmake -DCMAKE_CXX_STANDARD=17 ..
```

## Performance Notes

- Release builds are significantly faster than debug builds
- The library uses weighted random selection for quality and item generation
- TOML parsing happens during configuration, not during loot generation
- Thread safety: Each `Generator` instance is independent; use one per thread

## Building for Different Platforms

### Linux

```bash
cmake -DCMAKE_BUILD_TYPE=Release ..
make
```

### macOS

```bash
cmake -DCMAKE_BUILD_TYPE=Release ..
make
```

### Windows (MinGW)

```bash
cmake -G "MinGW Makefiles" -DCMAKE_BUILD_TYPE=Release ..
mingw32-make
```

## License

This example is part of the Praeda project.
