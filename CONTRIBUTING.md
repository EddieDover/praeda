# Contributing to Praeda

Thank you for your interest in contributing to Praeda! This guide will help you get started with development.

## Quick Start

### 1. Clone and Setup

```bash
git clone https://github.com/EddieDover/praeda.git
cd praeda
./scripts/setup-hooks.sh
```

The setup script configures git hooks that:
- Enforce Conventional Commits format for commit messages
- Run clippy and tests before each commit

### 2. Build and Test

```bash
# Build the library
cargo build --lib

# Run all tests
cargo test
```

### 3. Code Quality Checks

```bash
# Run clippy (linter)
cargo clippy --all-targets --all-features -- -D warnings
# or
cargo strict

# Generate coverage report
cargo coverage          # Full HTML/XML report
cargo coverage-check    # Quick percentage
cargo coverage-lcov     # LCOV format
```

Or use the helper script:
```bash
./scripts/coverage.sh           # Default 85% threshold
./scripts/coverage.sh 75        # Custom threshold
```

## Commit Standards

This project follows [Conventional Commits](https://www.conventionalcommits.org/).

### Format

```
type(scope): description
```

### Allowed Types

- **feat** - A new feature
- **fix** - A bug fix
- **refactor** - Code refactoring without feature or bug fix
- **docs** - Documentation changes
- **test** - Adding or updating tests
- **chore** - Maintenance tasks (dependencies, build, etc.)
- **perf** - Performance improvements
- **style** - Code style changes (formatting, etc.)
- **ci** - CI/CD configuration changes
- **build** - Build system changes
- **revert** - Reverting a previous commit

### Examples

```bash
git commit -m "feat: add new loot generation feature"
git commit -m "fix(cli): handle invalid config file paths"
git commit -m "docs: update README with examples"
git commit -m "chore(deps): bump clap to version 4.5"
git commit -m "refactor: simplify metadata handling"
git commit -m "test: add unit tests for attributes"
```

## Pre-commit Hooks

The hooks are automatically set up when you run `./scripts/setup-hooks.sh`.

### commit-msg Hook
Validates that your commit message follows Conventional Commits format. If your message is invalid, the commit will be rejected with helpful guidance.

### pre-commit Hook
Runs before each commit:
1. **Clippy** - Rust linter (must pass with no warnings)
2. **Tests** - All library tests (must pass)

If either check fails, the commit is rejected and you'll see the errors to fix.

### Bypassing Hooks (Emergency Only)

```bash
git commit --no-verify
```

**Note:** This is discouraged and should only be used in exceptional circumstances.

## Git Hooks Details

See [.githooks/README.md](.githooks/README.md) for detailed information about the hooks.

## Code Coverage

Praeda maintains an 85% code coverage threshold. All pull requests must meet this standard.

### Running Coverage Locally

```bash
# Quick check
cargo coverage-check

# Full report with HTML
cargo coverage

# Using the helper script
./scripts/coverage.sh

# Custom threshold
./scripts/coverage.sh 75
```

The HTML report (`coverage/tarpaulin-report.html`) shows:
- Red lines: uncovered code
- Green lines: covered code
- Yellow lines: partially covered

### Improving Coverage

1. Open `coverage/tarpaulin-report.html` in your browser
2. Look for red-highlighted lines (uncovered code)
3. Add tests for those code paths
4. Run `cargo test` to verify

## Pull Request Process

1. **Fork and branch** - Create a feature branch from `master`
2. **Commit with standards** - Use Conventional Commits format
3. **Keep hooks enabled** - Don't bypass pre-commit checks
4. **Ensure coverage** - New code should maintain 85%+ coverage
5. **Run tests locally** - `cargo test`
6. **Push and create PR** - GitHub will run CI checks

### What CI Checks

GitHub Actions will:
- Build the project
- Run all tests
- Run clippy with strict warnings
- Generate and check coverage (must meet 85%)
- Post coverage report as PR comment

## Development Tips

### Common Commands

```bash
# Build
cargo build
cargo build --release

# Test
cargo test
cargo test --lib
cargo test -- --test-threads=1

# Documentation
cargo doc --open

# Format check
cargo fmt --check

# Clippy
cargo clippy
cargo clippy --all-targets --all-features -- -D warnings
# or
cargo strict
```

### Directory Structure

```
praeda/
├── bindings/                      # Language bindings
│   └── praeda-godot/              # Godot 4.x GDExtension bindings
├── src/                           # Library source code
│   ├── lib.rs                     # Library entry point
│   ├── models.rs                  # Data structures (Item, ItemAttribute, Affix, etc.)
│   ├── generator.rs               # Loot generation logic (PraedaGenerator)
│   ├── ffi.rs                     # C++ and C# FFI bindings
│   └── error.rs                   # Error types
├── tests/                         # Test suite
│   ├── integration_test.rs        # Main integration tests
│   └── ffi_struct_test.rs         # FFI structure tests
├── examples/                      # Example programs
│   ├── loot_generator.rs          # CLI loot generator with --no-toml support
│   ├── test_data.toml             # Example TOML configuration
│   ├── cpp/                       # C++ FFI examples
│   │   ├── README.md
│   │   ├── CMakeLists.txt
│   │   └── test_praeda.cpp
│   ├── csharp/                    # C# FFI examples
│   │   ├── README.md
│   │   ├── PraedaGenerator.cs
│   │   └── PraedaTest.cs
│   └── godot/                     # Godot 4.x example project
│       ├── project.godot
│       └── test_praeda.gd
├── .github/workflows/             # CI/CD pipelines
│   ├── rust.yml                   # Rust build, test, and coverage
│   ├── build-libraries.yml        # FFI library builds for C++ and C#
│   └── release-plz.yml            # Automated release management
├── .githooks/                     # Git hooks
│   ├── README.md
│   ├── commit-msg                 # Conventional Commits validation
│   └── pre-commit                 # Clippy and test checks
├── scripts/                       # Helper scripts
│   ├── setup-hooks.sh             # Git hooks setup
│   └── coverage.sh                # Code coverage reporting
├── Praeda.sln                     # Visual Studio solution for C# bindings
├── .cargo/config.toml             # Cargo aliases and configuration
├── Cargo.toml                     # Package manifest
├── Cargo.lock                     # Dependency lock file
├── README.md                      # Project overview and quick start
├── CONTRIBUTING.md                # This file
└── LICENSE                        # LGPL-3.0-or-later
```

## Useful Cargo Aliases

From `.cargo/config.toml`:

```bash
cargo coverage          # Full coverage with HTML/XML
cargo coverage-check    # Quick coverage percentage
cargo coverage-lcov     # LCOV format coverage
```

## Running Examples

### Loot Generator CLI

The loot generator example demonstrates using the Praeda library programmatically and with TOML configuration:

```bash
# Generate loot from TOML configuration
cargo run --example loot_generator -- \
  --input examples/test_data.toml \
  --output output.json \
  --num-items 10 \
  --base-level 15.0 \
  --level-variance 5.0 \
  --affix-chance 0.8

# Generate loot using programmatic configuration (no TOML needed)
cargo run --example loot_generator -- \
  --no-toml \
  --output output.json \
  --num-items 10 \
  --base-level 15.0 \
  --level-variance 5.0 \
  --affix-chance 0.8
```

The `--no-toml` flag sets up all item types, subtypes, attributes, and affixes programmatically, useful for testing without external configuration files.

## Questions?

- Check [README.md](README.md) for usage documentation
- Look at [examples/test_data.toml](examples/test_data.toml) for TOML configuration examples
- Review [examples/loot_generator.rs](examples/loot_generator.rs) for programmatic API usage
- Check FFI examples in [examples/cpp/](examples/cpp/) and [examples/csharp/](examples/csharp/)
- Review existing tests in [tests/](tests/) for test patterns
- Open an issue for questions or suggestions

## Code Style

- Follow Rust conventions (enforced by `cargo fmt`)
- Use meaningful variable and function names
- Add doc comments for public items
- Keep functions focused and testable

## Testing Guidelines

- Write tests for new features
- Update tests when changing behavior
- Aim for >85% code coverage
- Use descriptive test names like `test_descriptive_behavior()`
- Consider edge cases and error paths

Thank you for contributing to Praeda!