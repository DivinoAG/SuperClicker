# Source Tree Structure

## Project Root
- `super_clicker/`: Main Rust crate directory.
  - `src/`: Source code files.
    - `main.rs`: Entry point of the application.
    - `app.rs`: Main application logic and state management (Iced).
    - `ui.rs`: User interface layout and styling.
    - `clicking.rs`: Logic for the auto-clicking engine.
    - `hotkeys.rs`: Global hotkey management using `global-hotkey`.
    - `settings.rs`: Configuration loading and saving.
  - `Cargo.toml`: Rust package dependencies and configuration.

## Documentation (`docs/`)
- `architecture/`: Architecture details, tech stack, and coding standards.
- `stories/`: User stories and development tasks (currently empty).
- `prd.md`: Product Requirements Document.
- `project-brief.md`: High-level project overview.
- `architecture.md`: High-level system architecture.
