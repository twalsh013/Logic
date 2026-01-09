# CI/CD Pipeline Documentation

## Overview

This project uses GitHub Actions for continuous integration and continuous deployment. The pipeline automatically runs on every commit to ensure code quality and build reliability.

## Current Pipeline Stages

### 1. Test Stage
**Triggers:** Every push and pull request to any branch

**Steps:**
- Code formatting check with `cargo fmt`
- Linting with `cargo clippy` (treating warnings as errors)
- Unit test execution with `cargo test`

**Purpose:** Ensures code quality and validates that all unit tests pass before proceeding to build.

### 2. Build Stage
**Triggers:** Only runs if test stage passes successfully

**Steps:**
- Builds release binary with optimizations
- Stores executable as a GitHub Actions artifact
- Artifacts are retained for 30 days

**Artifact naming:** `logic-<commit-sha>`

**Purpose:** Creates production-ready executable binaries for every successful build.

## Performance Optimizations

The pipeline includes caching for:
- Cargo registry (dependencies)
- Cargo git index
- Build artifacts (target directory)

This significantly reduces build times for subsequent runs.

## Future Enhancements

### Integration Testing (Planned)

Currently, the pipeline runs unit tests only. A future enhancement will include:

**End-to-End Testing Stage:**
- Run the built binary against predefined test circuit files
- Validate output against expected results
- Test various circuit configurations (combinational logic gates, multi-input scenarios)
- Performance benchmarking

**Implementation Status:**
🚧 **Not yet implemented** - The program architecture is still evolving and does not yet have stable test input files or expected output definitions. This will be added once:
1. Standard test circuit files are defined
2. Expected output format is stabilized
3. Test oracle (expected results) is created

**Placeholder for future workflow:**
```yaml
integration-test:
  name: Integration Tests
  runs-on: ubuntu-latest
  needs: build

  steps:
  - name: Download binary artifact
    uses: actions/download-artifact@v3

  - name: Run test circuits
    run: |
      chmod +x logic
      ./logic test_circuits/simple_and.txt --inputvec 0,0
      ./logic test_circuits/simple_and.txt --inputvec 0,1
      # ... more test cases

  - name: Validate outputs
    run: |
      # Compare actual vs expected outputs
      # Report any discrepancies
```

## Artifact Access

Build artifacts can be downloaded from:
1. The Actions tab in GitHub
2. Via GitHub CLI: `gh run download <run-id>`
3. Through the GitHub REST API

Artifacts are automatically cleaned up after 30 days.

## Local Testing

To replicate the CI pipeline locally:

```bash
# Format check
cargo fmt --all -- --check

# Linting
cargo clippy --all-targets --all-features -- -D warnings

# Run tests
cargo test --verbose

# Build release
cargo build --release
```

## Troubleshooting

**If tests fail:**
- Review test output in the Actions log
- Run `cargo test --verbose` locally to reproduce
- Check for environment-specific issues

**If build fails:**
- Verify `Cargo.toml` dependencies
- Check for platform-specific code issues
- Ensure all dependencies are available in the build environment
