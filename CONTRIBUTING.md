# Contributing to Horizon Browser

Thank you for your interest in contributing to Horizon Browser! This document provides guidelines and instructions for contributing.

## Code of Conduct

We are committed to providing a welcoming and inclusive environment. Please be respectful and professional in all interactions.

## How to Contribute

### Reporting Bugs

1. Check if the bug has already been reported in [Issues](https://github.com/GitGoneWild/Horizon-Browser/issues)
2. If not, create a new issue with:
   - Clear, descriptive title
   - Steps to reproduce
   - Expected vs actual behavior
   - Environment details (OS, Rust version)
   - Screenshots if applicable

### Suggesting Features

1. Check existing [Issues](https://github.com/GitGoneWild/Horizon-Browser/issues) for similar suggestions
2. Create a new issue with:
   - Clear description of the feature
   - Use cases and benefits
   - Possible implementation approach

### Pull Requests

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Make your changes following our coding standards
4. Write or update tests
5. Update documentation
6. Run tests and linting
7. Commit with clear messages
8. Push to your fork
9. Create a pull request

## Development Process

### Setting Up

```bash
# Fork and clone
git clone https://github.com/YOUR_USERNAME/Horizon-Browser.git
cd Horizon-Browser

# Add upstream remote
git remote add upstream https://github.com/GitGoneWild/Horizon-Browser.git

# Create feature branch
git checkout -b feature/my-feature
```

### Making Changes

1. Follow the [Development Guide](docs/DEVELOPMENT.md)
2. Write tests for new functionality
3. Update documentation
4. Format code: `cargo fmt --all`
5. Run linter: `cargo clippy --workspace --all-targets -- -D warnings`
6. Run tests: `cargo test --workspace`

### Commit Messages

Follow conventional commits format:

```
type(scope): subject

body

footer
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `style`: Code style (formatting)
- `refactor`: Code refactoring
- `test`: Adding tests
- `chore`: Maintenance

**Examples:**
```
feat(engine): add WebGPU rendering backend

Implement WebGPU-based rendering for improved performance
on modern GPUs.

Closes #123
```

```
fix(networking): handle timeout errors correctly

Fixed issue where timeout errors were not properly propagated,
causing the browser to hang.

Fixes #456
```

### Pull Request Guidelines

1. **Title**: Clear, descriptive title
2. **Description**: 
   - What changes were made
   - Why the changes were needed
   - How to test the changes
3. **Link Issues**: Reference related issues
4. **Tests**: Include test coverage
5. **Documentation**: Update relevant docs
6. **CI**: Ensure all checks pass

### Code Review Process

1. Maintainers will review your PR
2. Address feedback and comments
3. Make requested changes
4. Once approved, PR will be merged

## Coding Standards

### Rust Style

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` for formatting
- Use `clippy` for linting
- Maximum line length: 100 characters

### Documentation

- Document all public APIs
- Include examples in doc comments
- Update README.md for user-facing changes
- Add inline comments for complex logic

### Testing

- Write unit tests for new functions
- Write integration tests for features
- Maintain test coverage above 80%
- Test edge cases and error conditions

### Performance

- Profile performance-critical code
- Avoid unnecessary allocations
- Use appropriate data structures
- Document performance characteristics

## Project Organization

### Crate Guidelines

Each crate should:
- Have a single, clear responsibility
- Expose a clean public API
- Include comprehensive tests
- Be well-documented

### Module Organization

```rust
// Good: Clear module structure
mod renderer {
    mod software;
    mod hardware;
    
    pub use software::SoftwareRenderer;
    pub use hardware::HardwareRenderer;
}

// Bad: Everything in one file
mod renderer { /* 1000+ lines */ }
```

## Getting Help

- **Discord**: [Join our Discord](https://discord.gg/horizon) (placeholder)
- **Discussions**: [GitHub Discussions](https://github.com/GitGoneWild/Horizon-Browser/discussions)
- **Issues**: [GitHub Issues](https://github.com/GitGoneWild/Horizon-Browser/issues)

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

## Recognition

Contributors will be recognized in:
- README.md contributors section
- Release notes
- CONTRIBUTORS.md file

## Questions?

Feel free to ask questions in:
- GitHub Discussions
- Pull request comments
- Issues

Thank you for contributing to Horizon Browser! 🚀
