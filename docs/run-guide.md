# Running SuperClicker

## Prerequisites

- Rust 2024 edition or later (install from [rust-lang.org](https://www.rust-lang.org/))
- Windows OS (for system-wide hotkey and mouse simulation support)
- PowerShell or Command Prompt terminal

## CRITICAL: How to Run (Proper Method)

**ALWAYS use this exact command from any directory:**

```powershell
& cmd /c "cd /d D:\Projects\SuperClicker\super_clicker && cargo build --quiet && cargo run"
```

**Why this method?**
- Uses `cmd /c` to ensure proper directory change (`cd /d` = change drive and directory)
- Builds quietly first, then runs
- Works consistently from any starting directory

**Alternative Method (if you use traditional Command Prompt):**

```cmd
cd /d D:\Projects\SuperClicker\super_clicker
cargo build --quiet
cargo run
```

## Application Usage

### Starting the Clicker

1. Set the desired click interval in the "Click Interval (ms):" field (e.g., 100 for 100ms between clicks)
2. Select the mouse button from the dropdown (Left, Middle, or Right)
3. Click the "Start" button to begin auto-clicking at the specified interval
4. Click the "Stop" button to halt clicking immediately

### Console Output

When clicking is active, the terminal shows:
- `Successfully registered hotkey: Ctrl+Alt+F6` - confirms hotkey registration
- No additional output during clicking (clean operation)
- Messages only appear on startup and shutdown

### Keyboard Hotkey

The default hotkey to toggle clicking on/off is **Ctrl+Alt+F6**. However, system-wide hotkey detection may require administrative privileges on Windows.

## Verifying Click Interval Works Correctly

To test that clicks respect the interval:

1. Open a text editor (Notepad) and place your cursor in a text field
2. Set interval to **500** (0.5 seconds) and click "Start"
3. You should see clicking happening at 0.5 second intervals (2 clicks per second)
4. Check terminal output confirms "Successfully registered hotkey"
5. Click "Stop" to halt

### Expected Behavior

- Clicks occur at the **exact interval specified** (measured from click start to click start)
- With 500ms interval, you should see 2 clicks per second
- The interval is independent of how fast the actual mouse button press/release happens
- Each interval includes the time to press and release the button (~10-15ms)

## Troubleshooting

| Issue | Solution |
|-------|----------|
| `error: could not find Cargo.toml` | Use the exact command shown above with `cmd /c` |
| Compilation takes too long | This is normal on first build. Subsequent builds are faster |
| Clicks not registering | The target application must have focus; try clicking in Notepad to verify |
| Hotkey (Ctrl+Alt+F6) not working | Run terminal as Administrator, or use the UI Start/Stop buttons instead |
| Application crashes | Check console output for error messages |

## Build Modes

- **Debug (default):** `cargo build --quiet && cargo run` - Faster compilation, slower execution
- **Release mode:** `cargo build --release --quiet && cargo run --release` - Slower to compile, faster at runtime

For actual testing, the debug mode is sufficient. Release mode optimizes click timing slightly but isn't necessary for normal use.
