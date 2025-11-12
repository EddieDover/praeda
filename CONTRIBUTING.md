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
- ✅ Build the project
- ✅ Run all tests
- ✅ Run clippy with strict warnings
- ✅ Generate and check coverage (must meet 85%)
- ✅ Post coverage report as PR comment

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
```

### Directory Structure

```
praeda/
├── src/                    # Library source code
│   ├── lib.rs             # Library entry point
│   ├── models.rs          # Data structures
│   ├── generator.rs       # Loot generation logic
│   └── error.rs           # Error types
├── tests/                 # Integration tests
│   └── integration_test.rs
├── examples/              # Example programs
│   ├── loot_generator.rs  # CLI loot generator
│   └── test_data.toml     # Example configuration
├── .github/workflows/     # CI/CD pipelines
│   └── rust.yml
├── .githooks/             # Git hooks
│   ├── commit-msg
│   └── pre-commit
├── scripts/               # Helper scripts
│   ├── setup-hooks.sh
│   └── coverage.sh
└── .cargo/config.toml     # Cargo aliases
```

## Useful Cargo Aliases

From `.cargo/config.toml`:

```bash
cargo coverage          # Full coverage with HTML/XML
cargo coverage-check    # Quick coverage percentage
cargo coverage-lcov     # LCOV format coverage
```

## Questions?

- Check [README.md](README.md) for usage documentation
- Look at [examples/test_data.toml](examples/test_data.toml) for configuration examples
- Review existing tests in [tests/](tests/) for patterns
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

Thank you for contributing to Praeda! 🚀