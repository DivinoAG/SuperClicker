# Coding Standards

## General Principles
- **Safety:** Leverage Rust's ownership and type system to ensure memory safety.
- **Clarity:** Write code that is easy to read and understand. Prefer explicit over implicit.
- **Conciseness:** Avoid unnecessary boilerplate.

## Formatting & Style
- **Formatter:** Use `rustfmt` with default settings.
- **Naming Conventions:**
  - Variables/Functions: `snake_case`
  - Types/Traits/Structs: `UpperCamelCase`
  - Constants: `SCREAMING_SNAKE_CASE`
- **Indentation:** 4 spaces.

## Error Handling
- Use `Result<T, E>` for recoverable errors.
- Use `Option<T>` for optional values.
- Avoid `unwrap()` in production code; use `expect()` with a meaningful message or handle errors gracefully.

## Testing
- Unit tests should be co-located with the code in a `tests` module.
- Integration tests reside in the `tests/` directory.
- Run tests with `cargo test`.

## Documentation
- Public APIs must be documented using `///` comments.
- Modules should have top-level documentation using `//!`.
