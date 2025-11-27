# SuperClicker

SuperClicker is a lightweight, high-performance auto-clicker application built in Rust. It allows you to automate mouse clicks with precision control, designed to be unobtrusive and efficient.

## Features

*   **High Performance:** Native Windows application with minimal resource usage.
*   **Global Hotkeys:** Control the clicker even when the application is in the background.
    *   **Toggle:** `Ctrl` + `Alt` + `F6` to Start/Stop.
    *   **Dynamic Speed:** Hold `Ctrl` + `Alt` and **Scroll Mouse Wheel** to adjust click interval on the fly.
*   **Precision Control:** Adjust interval from 1ms to 10s.
*   **Button Selection:** Choose between Left, Middle, and Right mouse buttons.
*   **Visual Interface:** Clean, dark-mode compatible UI (using Iced).

## Installation & Usage

### Building from Source

1.  **Prerequisites:** Ensure you have [Rust and Cargo](https://rustup.rs/) installed.
2.  Clone the repository:
    ```bash
    git clone https://github.com/yourusername/SuperClicker.git
    cd SuperClicker
    ```
3.  Build and Run:
    ```bash
    cd super_clicker
    cargo run --release
    ```
    The executable will be located at `super_clicker/target/release/super_clicker.exe`.

## How to Use

1.  **Launch** the application.
2.  **Set Interval:** Type the desired interval in milliseconds (e.g., `100` for 10 clicks/sec) or use the scroll hotkey.
3.  **Select Button:** Choose Left, Right, or Middle click.
4.  **Start Clicking:**
    *   Press `Start` in the UI.
    *   OR press **Ctrl + Alt + F6**.
5.  **Adjust Speed:**
    *   Ensure "Enable Dynamic Interval Adjustment" is checked.
    *   Hold **Ctrl + Alt** and **Scroll Up** to speed up (decrease interval).
    *   Hold **Ctrl + Alt** and **Scroll Down** to slow down (increase interval).

## Technology Stack

*   **Language:** Rust
*   **GUI:** [Iced](https://github.com/iced-rs/iced)
*   **Input Handling:** [rdev](https://github.com/rdev-rs/rdev)

## License

MIT
