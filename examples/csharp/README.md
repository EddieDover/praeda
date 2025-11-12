# Praeda C# Example

This example demonstrates how to use the Praeda loot generation library from C# through its FFI (Foreign Function Interface) using P/Invoke.

## Prerequisites

- .NET 9.0 or later (or any supported .NET version)
- Rust toolchain (to build the Praeda library)
- Linux, macOS, or Windows with appropriate C library bindings

## Building

### Step 1: Build the Praeda Rust Library

First, build the Praeda library in release mode:

```bash
cd /path/to/praeda
cargo build --release
```

This creates the shared library at `target/release/libpraeda.so` (Linux) or `libpraeda.dylib` (macOS).

### Step 2: Build the C# Project

Navigate to the example directory and restore/build with dotnet:

```bash
cd examples/csharp
dotnet build -c Release
```

Or for development/debug builds:

```bash
dotnet build
```

## Running

Execute the test:

```bash
cd examples/csharp
LD_LIBRARY_PATH=../../target/release dotnet run --configuration Release
```

Or if you've already built:

```bash
cd examples/csharp
LD_LIBRARY_PATH=../../target/release dotnet bin/Release/net9.0/PraedaTest.dll
```

### On Linux

```bash
export LD_LIBRARY_PATH=/path/to/praeda/target/release:$LD_LIBRARY_PATH
dotnet run --configuration Release
```

### On macOS

```bash
export DYLD_LIBRARY_PATH=/path/to/praeda/target/release:$DYLD_LIBRARY_PATH
dotnet run --configuration Release
```

### On Windows

The DLL will be searched in standard locations. You can set `PATH` if needed:

```cmd
set PATH=C:\path\to\praeda\target\release;%PATH%
dotnet run --configuration Release
```

## Expected Output

The test performs 4 main test suites:

1. **Programmatic Configuration** - Sets qualities, item types, subtypes, attributes, and names
2. **Query Methods** - Demonstrates checking if qualities exist
3. **Loot Generation** - Generates items using programmatic configuration
4. **Generator Info** - Retrieves library version information

Example output:
```
=== Praeda C# FFI Test ===

Creating generator...
✓ Generator created successfully

--- Test 1: Programmatic Configuration ---
Setting qualities...
✓ Qualities set
Setting item types...
✓ Item types set
...
--- Test 3: Loot Generation ---
Generating 5 items...
✓ Generated 5 items:
  1. [rare] armor / chest - plate_armor
  2. [common] weapon / axe - battleaxe
...

=== All Tests Passed! ===
```

## Code Structure

### Main Components

- **PraedaGenerator.cs** - Contains everything needed to use Praeda from C#:
  - **NativeMethods** - P/Invoke declarations for all FFI functions
  - **C# Data Structs** - C-compatible structs for FFI marshaling:
    - `CItemAttribute` - C struct for item attributes
    - `CAffix` - C struct for affixes
    - `CItem` - C struct for items
  - **Native C# Classes** - User-facing types:
    - `ItemAttribute` - Item attribute with value ranges
    - `Affix` - Prefix/suffix with attributes
    - `Item` - Generated item with all properties
    - `GenerationOptions` - Configuration for loot generation
    - `PraedaGenerator` - Main loot generation engine

- **PraedaTest.cs** - Comprehensive test suite demonstrating:
  - Generator creation and configuration
  - All API methods
  - Error handling
  - Native C# type usage throughout

## API Usage

### Creating a Generator

```csharp
using var gen = new PraedaGenerator();
```

The `using` statement ensures proper cleanup via `IDisposable`.

### Configuring Qualities and Items

```csharp
gen.SetQualityData("common", 100);
gen.SetQualityData("rare", 30);

gen.SetItemType("weapon", 2);
gen.SetItemSubtype("weapon", "sword", 3);

gen.SetAttribute("weapon", "", "damage", 15.0, 5.0, 30.0, true);
gen.SetItemNames("weapon", "sword", new[] { "longsword", "shortsword" });
```

### Loading from TOML

```csharp
string tomlContent = @"
[qualities]
common = 100
rare = 30

[items.weapon]
weight = 2

[items.weapon.subtypes.sword]
weight = 3
";

gen.LoadTomlString(tomlContent);
```

### Generating Loot

```csharp
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
    Console.WriteLine($"[{item.Quality}] {item.Type}/{item.Subtype} - {item.Name}");
}
```

### Querying Configuration

```csharp
bool hasCommon = gen.HasQuality("common");
Console.WriteLine($"Has quality 'common': {hasCommon}");
```

### Getting Library Info

```csharp
string info = gen.GetInfo();
Console.WriteLine($"Praeda version: {info}");
```

## Error Handling

The C# wrapper uses exceptions for error handling:

```csharp
try {
    using var gen = new PraedaGenerator();
    if (!gen.SetQualityData("common", 100)) {
        throw new Exception("Failed to set quality");
    }
    var items = gen.GenerateLoot(options);
} catch (Exception ex) {
    Console.Error.WriteLine($"Error: {ex.Message}");
}
```

Configuration methods return `bool` - `true` for success, `false` for failure. High-level methods like `GenerateLoot()` throw exceptions on error.

## P/Invoke Details

The C# implementation uses P/Invoke with careful marshaling:

- **String marshaling**: `[MarshalAs(UnmanagedType.LPStr)]` for UTF-8 C strings
- **Array marshaling**: `[MarshalAs(UnmanagedType.LPArray)]` for string arrays
- **Calling convention**: `CallingConvention.Cdecl` to match Rust FFI
- **Struct layout**: `[StructLayout(LayoutKind.Sequential)]` for C struct compatibility

## Troubleshooting

### DLL/SO Not Found

If you get `DllNotFoundException`:

1. Ensure the library is built: `cargo build --release`
2. Verify the library exists at the expected location
3. Set the library path before running:
   - **Linux**: `export LD_LIBRARY_PATH=/path/to/praeda/target/release:$LD_LIBRARY_PATH`
   - **macOS**: `export DYLD_LIBRARY_PATH=/path/to/praeda/target/release:$DYLD_LIBRARY_PATH`
   - **Windows**: Add to `PATH` or use `DllImportSearchPath`

### Platform-Specific Issues

**Linux**: Most distributions include libc by default. If you encounter issues, verify glibc compatibility.

**macOS**: The library may need to be code-signed. Try:
```bash
codesign --force --deep --sign - /path/to/libpraeda.dylib
```

**Windows**: Ensure Visual C++ Redistributables are installed if using MSVC-built Rust toolchain.

## Project Configuration

The C# project uses `net9.0` as the target framework. To change it:

Edit `PraedaTest.csproj`:
```xml
<TargetFramework>net8.0</TargetFramework>
```

Supported frameworks: net6.0, net7.0, net8.0, net9.0, etc.

## Building and Deployment

### Debug Build

```bash
dotnet build
dotnet run
```

### Release Build

```bash
dotnet build -c Release
dotnet bin/Release/net9.0/PraedaTest.dll
```

### Self-Contained Deployment

To create a standalone executable:

```bash
dotnet publish -c Release -r linux-x64
```

This includes the .NET runtime; adjust `-r` for your platform:
- `linux-x64` - Linux x86-64
- `osx-x64` - macOS Intel
- `osx-arm64` - macOS Apple Silicon
- `win-x64` - Windows x86-64

## Performance Notes

- Release builds are significantly faster than debug builds
- P/Invoke calls have minimal overhead
- The library uses weighted random selection for quality and item generation
- TOML parsing happens during configuration, not during loot generation
- Thread safety: Each `PraedaGenerator` instance is independent; use one per thread

## Async Usage

The Praeda library is synchronous. For async usage, wrap it in a task:

```csharp
var items = await Task.Run(() => {
    using var gen = new PraedaGenerator();
    gen.SetQualityData("common", 100);
    return gen.GenerateLoot(options);
});
```

## License

This example is part of the Praeda project.
