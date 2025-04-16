# Contributing to This Rust Project

Thank you for your interest in contributing! This document outlines the standards and expectations for contributions to this project.

---

## Reporting Bugs

If you encounter a bug, please help us by submitting an issue with the following details:

- A clear and descriptive title  
- Steps to reproduce the issue  
- Expected vs. actual behavior  
- Relevant environment info (e.g., OS, Rust version)  
- A minimal code sample, if possible  

---

## How to Contribute

1. Fork the repository and clone your fork.
2. Create a new branch for your changes.
3. Make your changes—keep them focused and minimal.
4. Run the required checks (see below).
5. Open a pull request with a clear description of your changes.

---

## Unsafe Code Policy

- Unsafe code **must be avoided** unless absolutely necessary.
- If `unsafe` is used, it **must be justifiable and its safety must be verifiable at a glance**:
  - Keep the unsafe block small and isolated.
  - Clearly document why the usage is safe.

---

## Code Quality Requirements

Before submitting a pull request, please ensure your code meets **all** of the following:

- **All tests must pass**: Run `cargo test`
- **No Clippy warnings**: Run `cargo clippy` and resolve all warnings
- **Proper formatting**: Run `cargo fmt` to ensure consistent formatting
- **Tests are required**: Include tests for new features or bug fixes

Pull requests that do not meet these requirements may be rejected or asked to be revised.

---

## Thank You

Your contributions help make this project better! Whether it’s fixing bugs, improving documentation, or adding new features—every bit helps.
