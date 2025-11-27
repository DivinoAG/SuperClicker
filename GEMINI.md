# SuperClicker Project Context for Gemini Agent

This document provides a comprehensive overview of the `SuperClicker` project, designed to serve as essential context for future interactions with the Gemini Agent. It outlines the project's purpose, technical stack, development practices, and key operational guidelines.

---

## 1. Project Overview

**Project Name:** SuperClicker
**Purpose:** SuperClicker is a lightweight, high-performance auto-clicker application built in Rust. It automates mouse clicks with precision control, designed to be unobtrusive, efficient, and user-friendly on Windows platforms.

**Core Functionality:**
*   Automated mouse clicking (Left, Middle, Right buttons).
*   Global Hotkeys for Start/Stop (`Ctrl` + `Alt` + `F6`).
*   Dynamic adjustment of click interval via hotkey (`Ctrl` + `Alt` + Scroll Wheel).
*   Configurable click interval (1ms to 10s).
*   Graphical User Interface (GUI) for configuration and status display.

**Key Design Principles (from `docs/front-end-spec.md`):**
*   **Compactness:** Minimal screen real estate usage.
*   **Clarity:** Instantly readable status.
*   **Efficiency:** Settings adjustable with minimal clicks.
*   **System Integration:** Blends with Windows aesthetics.
*   **Adaptive Theming:** Automatically respects system Light/Dark mode.

---

## 2. Technical Stack

**Language:** Rust (Edition 2024)
**GUI Framework:** [Iced (v0.12)](https://github.com/iced-rs/iced) with `smol` async runtime.
**Input Handling:** [rdev (v0.5.3)](https://github.com/rdev-rs/rdev) for both mouse simulation and global hotkey detection.
**Serialization:** `serde`, `serde_json`.
**System Utilities:** `dirs` (for path handling), `dark-light` (for system theme detection).

**Core Modules (`super_clicker/src/`):**
*   `main.rs`: Application entry point, window setup.
*   `app.rs`: Main application logic, state management (Iced).
*   `ui.rs`: User interface layout and custom widget styling.
*   `clicking.rs`: Logic for the auto-clicking engine (uses `rdev`).
*   `hotkeys.rs`: Global hotkey management (uses `rdev` events, integrates with Iced subscriptions).
*   `settings.rs`: Configuration loading and saving.

---

## 3. Building and Running

**Prerequisites:**
*   Rust and Cargo (ensure installed via `rustup`).

**Commands:**
To build and run the application from the project root:
```bash
cd super_clicker
cargo run --release
```
The compiled executable will be located in `super_clicker/target/release/super_clicker.exe`.

**Console Window Note:** Currently, the application opens with a console window. This is intended to be suppressed in a later polish phase by adding `#![windows_subsystem = "windows"]` to `main.rs`.

---

## 4. Development & Operational Guidelines

This project utilizes a structured development workflow managed by the BMAD system.

**Key Documentation:**
*   `docs/prd.md`: Product Requirements Document (Goals, Requirements, Epics, Stories).
*   `docs/front-end-spec.md`: Detailed UI/UX Specification (Layout, Colors, Components, Accessibility).
*   `docs/architecture/coding-standards.md`: Coding guidelines.
*   `docs/architecture/tech-stack.md`: Current technology stack details.
*   `docs/stories/`: Individual user stories and development tasks.

**BMAD Agent Configuration:**
The `.bmad-core/` directory contains configuration for various agents (e.g., `pm`, `dev`). Relevant configurations are in `.bmad-core/core-config.yaml`.

**Operational Guidelines for Gemini Agent:**
*   **Document Before Implement (DBI):** Every change discussed must always be documented (in stories/specs) FIRST, and THEN it should be implemented. This includes detailing new requests or modifications to existing specifications.
*   **Conflict Clarification:** If a new user request conflicts with a previous specification, always clarify with the user if the new request should overwrite the old one. Update the spec/stories accordingly BEFORE any new implementation. The documentation must ALWAYS reflect the most up-to-date state.
*   **UI/UX Refinement:** Always refer to `docs/front-end-spec.md` and `docs/reference_ui.jpg` for visual design guidance when implementing UI elements. Ensure WCAG AA compliance for contrast.
*   **Build & Test:** Always verify changes by building (`cargo build --release`) and running the application. Propose and implement UI tests (e.g., integration tests for component rendering) to prevent regressions.

---

## 5. Current Development Context (as of last interaction)

The agent is currently working on **Story 4.3: Control Grouping & Dynamic Disabling** (part of Epic 4: UI/UX Refactor).
**Known Issues / Pending Fixes:**
*   **Critical Regression:** Button labels are currently not displaying in the UI. This needs immediate resolution.
*   **Regression:** The conditional rendering logic for `Interval` and `Mouse Button` inputs (to display as read-only text when the clicker is running) has regressed and needs to be restored.
*   **Styling:** Input fields, dropdown, and checkbox styling need further refinement to ensure backgrounds match the main app background, and disabled states are visually distinct.

This `GEMINI.md` will be updated as the project evolves.
