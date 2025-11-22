# Praeda Godot Example

This example demonstrates how to use the Praeda loot generator in Godot 4.x.

## Prerequisites

- Rust (latest stable)
- Godot 4.2 or later

## Setup

1. Build the GDExtension library:
   ```bash
   cargo build
   ```
   This will create the shared library in `target/debug/`.

2. Open the project in Godot:
   - Launch Godot
   - Import this folder (`examples/godot`)
   - Run the `main.tscn` scene

## Structure

- `praeda.gdextension`: Configuration file linking the compiled Rust library to Godot.
- `test_praeda.gd`: GDScript example usage.
- `main.tscn`: Main scene that runs the test script.
