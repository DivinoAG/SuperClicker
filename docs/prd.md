
# SuperClicker Product Requirements Document (PRD)

## 1. Goals and Background Context

### Goals
- To provide users with a simple and efficient tool for automating mouse clicks.
- To create a lightweight native Windows application that runs in the background with minimal performance impact.
- To offer intuitive controls, including hotkeys for starting/stopping and dynamically adjusting the click interval.

### Background Context
Many users, particularly in gaming and data entry, perform highly repetitive clicking tasks. This manual process can be tedious and inefficient. SuperClicker aims to solve this by providing a reliable and easy-to-use auto-clicker. Unlike some existing solutions, it will provide a straightforward way to dynamically control the click speed and select the desired mouse button, running unobtrusively in the background.

### Change Log
| Date       | Version | Description                               | Author |
| :--------- | :------ | :---------------------------------------- | :----- |
| 2025-11-27 | 1.1     | Hotkey toggle implemented; docs aligned   | James  |
| 2025-10-29 | 1.0     | Initial Draft                             | John   |

## 2. Requirements

### Functional
- **FR1:** The application shall be able to simulate mouse clicks (left, middle, or right button) at the current cursor position.
- **FR2:** The application shall allow the user to set a specific interval between clicks, either via a direct input field in the UI or through a dynamic hotkey.
- **FR3:** The application shall provide a keyboard shortcut to start and stop the auto-clicking functionality.
- **FR4:** The application shall provide a keyboard shortcut that, when held, allows the user to adjust the click interval using the mouse scroll wheel.
- **FR5:** The adjustment of the interval via the scroll wheel shall have variable speed, with faster scrolling leading to larger increments.
- **FR6:** The application shall provide an option in the UI to disable the dynamic interval adjustment hotkey.
- **FR7:** The application shall allow the user to customize the hotkeys for toggling the auto-clicker and for dynamic interval adjustment.
- **FR8:** The hotkey customization UI shall include toggles for Ctrl, Alt, and Shift, and an input for a single key.
- **FR9:** The user shall be able to clear the key input to use a combination of only Ctrl, Alt, and Shift.
- **FR10:** The application shall have a user interface window.
- **FR11:** The UI window shall display the current status of the auto-clicker (e.g., "Running," "Stopped").
- **FR12:** The UI window shall display the current click interval and allow for direct text input.
- **FR13:** The UI window shall contain a control (e.g., a dropdown menu) to select the mouse button to be clicked (Left, Middle, Right).

### Non-Functional
- **NFR1:** The application must be a native Windows application.
- **NFR2:** The application should consume minimal system resources (CPU and memory) to avoid impacting the performance of other running applications.
- **NFR3:** The system-wide hotkeys must be responsive and not conflict with common application shortcuts.

## 3. User Interface Design Goals

- **Overall UX Vision:** A simple, no-frills utility. The user should be able to understand and use the application within seconds. The focus is on function over form.
- **Key Interaction Paradigms:** The primary interaction is through system-wide hotkeys. The UI window is for initial configuration and status checking.
- **Core Screens and Views:** A single main window is sufficient. It will contain:
    - Status display (text).
    - Current interval display (text box for direct input).
    - Mouse button selection (dropdown).
    - A checkbox to enable/disable the dynamic interval adjustment hotkey.
    - A section for "Toggle Hotkey" with Ctrl, Alt, Shift checkboxes and a key input field.
    - A section for "Interval Adjustment Hotkey" with Ctrl, Alt, Shift checkboxes and a key input field.
- **Accessibility:** WCAG AA compliance should be a target to ensure it is usable by more people.
- **Target Device and Platforms:** Desktop Only (Windows).

## 4. Technical Assumptions

- **Repository Structure:** Monorepo.
- **Service Architecture:** Monolith (single executable application).
    - **Technology Stack Assumption:** Rust is the chosen technology stack, providing a performant and memory-safe environment for a native Windows application.
- **Testing Requirements:** Unit and Integration testing. End-to-end manual testing will be required to ensure compatibility with different applications.

## 5. Epic List

- **Epic 1: Foundational Setup & Core Click Logic:** **COMPLETED (Hotkeys Primary Control)** - Basic clicking mechanism and main window implemented. Hotkeys are now the primary start/stop control, replacing UI buttons.
- **Epic 2: Advanced Control and Hotkeys:** **IN PROGRESS** - Basic hotkey toggle implemented. Dynamic adjustment and customization are pending.
- **Epic 3: Hotkey Customization:** **PENDING** - User-configurable hotkeys not yet implemented.
- **Epic 4: UI/UX Refactor:** **PENDING** - Improve the application's visual design, layout, and user experience to be more polished, professional, and compact, respecting system themes.
- **Epic 5: Technical Debt & Refinement:** **PENDING** - Address identified technical debt and optimize performance.

## 6. Epic Details

### Epic 1: Foundational Setup & Core Click Logic

*Goal: To create a basic, functional application that can be controlled via hotkeys, can perform clicks at a set interval, and allows the user to select the mouse button.*

**Story 1.1: Project Scaffolding and Main Window (COMPLETED)**
> As a developer,
> I want to set up the initial Rust project structure and create a basic main window,
> so that we have a foundation to build the application upon.

- **Acceptance Criteria:**
    1. A new Rust `iced` project is created.
    2. A main application window is created and can be launched.
    3. The window has a title of "SuperClicker".
    4. The window contains placeholder text for status.
    5. The window contains a text input field for the click interval, pre-filled with a default value (e.g., "100").
    6. The window contains a dropdown menu for mouse button selection with "Left", "Middle", and "Right" as options.
    7. The window contains a checkbox, labeled "Enable Dynamic Interval Adjustment", which is checked by default.

**Story 1.2: Basic Clicking Logic (COMPLETED)**
> As a user,
> I want the application to repeatedly click the mouse at a specified interval when I use the toggle hotkey,
> so that I can automate repetitive clicks.

- **Acceptance Criteria:**
    1. The application begins clicking the selected mouse button at the current cursor location when the toggle hotkey is pressed.
    2. The click interval is based on the value in the interval input field.
    3. When the toggle hotkey is pressed again, the clicking ceases.
    4. The status text in the window updates to "Running" or "Stopped" accordingly.

### Epic 2: Advanced Control and Hotkeys

*Goal: To enable full background control of the application using system-wide hotkeys for a seamless user experience.*

**Story 2.1: Toggle Hotkey (PARTIALLY IMPLEMENTED)**
> As a user,
> I want to press a global hotkey to start and stop the auto-clicker,
> so that I don't have to switch to the application window.

- **Acceptance Criteria:**
    1. The application uses a default global hotkey (Ctrl+Alt+F6) to toggle the auto-clicker on and off.
    2. The hotkey works even when the SuperClicker application is not the active window.
    3. The "Start" and "Stop" buttons in the UI are no longer present, as the hotkey is now the primary control.

**Story 2.2: Dynamic Interval Adjustment Hotkey**
> As a user,
> I want to hold a hotkey and use my mouse wheel to change the click interval,
> so that I can adjust the click speed on the fly.

- **Acceptance Criteria:**
    1. The application uses the user-configured hotkey for interval adjustment.
    2. When this hotkey is held down and enabled, scrolling the mouse wheel up decreases the click interval (speeds up clicking).
    3. When this hotkey is held down and enabled, scrolling the mouse wheel down increases the click interval (slows down clicking).
    4. The interval adjustment is proportional to the scroll speed (faster scroll, larger change).
    5. A checkbox in the UI allows the user to enable or disable this hotkey.
    6. The interval display in the main window updates in real-time as it is adjusted.

### Epic 3: Hotkey Customization

*Goal: To allow users to define their own hotkeys for the main application functions.*

**Story 3.1: UI for Hotkey Customization**
> As a user,
> I want a UI to configure my preferred hotkeys,
> so that I can avoid conflicts with other applications and use combinations that are comfortable for me.

- **Acceptance Criteria:**
    1. The main window has a clearly labeled section for "Toggle Hotkey Customization".
    2. This section contains checkboxes for Ctrl, Alt, and Shift.
    3. This section contains a text input field that captures a single key press.
    4. The main window has a similar section for "Interval Adjustment Hotkey Customization".
    5. The input field can be cleared to allow for combinations of only modifier keys (Ctrl, Alt, Shift).

**Story 3.2: Logic for Custom Hotkeys**
> As a developer,
> I want to save the user's custom hotkey configuration and register them with the operating system,
> so that the user's preferences are applied and persist across sessions.

- **Acceptance Criteria:**
    1. The application saves the customized hotkey settings.
    2. The application unregisters the old hotkeys and registers the new custom hotkeys.
    3. The custom hotkeys are loaded and applied when the application starts.
    4. Default hotkeys (Ctrl+Alt+F6 and Ctrl+Alt+Shift) are used if no custom configuration is saved.

### Epic 4: UI/UX Refactor

*Goal: To transform the application's user interface into a polished, professional, and compact utility that adheres to modern UI/UX best practices and system aesthetics.*

**Story 4.1: Initial Window Refactor & Layout**
> As a user,
> I want the application window to be compact and non-resizable,
> so that it takes up minimal screen space and has a professional appearance.

- **Acceptance Criteria:**
    1. The main application window has a fixed size, determined by its content.
    2. The window is not resizable by the user.
    3. The window displays the application title "SuperClicker" in its title bar.
    4. Elements within the window are laid out with consistent padding and margins, as per the Frontend Specification.

**Story 4.2: Adaptive Theming & Enhanced Status Display**
> As a user,
> I want the application to respect my system's light/dark mode settings and show a clear status,
> so that the app integrates seamlessly with my OS and I can easily tell if it's active.

- **Acceptance Criteria:**
    1. The application's UI theme (colors, backgrounds) automatically adapts to the system's light or dark mode setting.
    2. The "Status" display (Running/Stopped) is visually prominent with distinct colors (e.g., green for Running, neutral for Stopped).
    3. All UI elements maintain sufficient color contrast in both light and dark modes (WCAG AA).

**Story 4.3: Control Grouping & Dynamic Disabling**
> As a user,
> I want related controls to be visually grouped and non-essential settings to be disabled while the clicker is running,
> so that the UI is organized and I'm prevented from making accidental changes during operation.

- **Acceptance Criteria:**
    1. Primary controls (Interval Input, Mouse Button Selector) are visually grouped together.
    2. Hotkey configuration elements are visually grouped separately.
    3. When the auto-clicker is "RUNNING", all interactive UI elements *except* the Stop button and the current interval display (which is updated by hotkey) are disabled and appear greyed out.
    4. When the auto-clicker is "STOPPED", all interactive UI elements are enabled.

**Story 4.4: Manual Start/Stop Buttons**
> As a user,
> I want clear Start and Stop buttons in the UI,
> so that I have a manual way to control the auto-clicker in addition to the hotkey.

- **Acceptance Criteria:**
    1. The main window contains a clearly labeled "Start" button.
    2. The main window contains a clearly labeled "Stop" button.
    3. The "Start" button is enabled only when the auto-clicker is "STOPPED".
    4. The "Stop" button is enabled only when the auto-clicker is "RUNNING".
    5. Clicking "Start" initiates the auto-clicking functionality.
    6. Clicking "Stop" ceases the auto-clicking functionality.

**Story 4.5: UI Component Styling**
> As a developer,
> I want to apply consistent and compact styling to all UI components,
> so that the application has a polished and professional appearance.

- **Acceptance Criteria:**
    1. All `iced` widgets (Text Input, Dropdown, Checkbox, Button) use custom styling that aligns with the Frontend Specification's Component Library.
    2. Components have compact heights and appropriate internal padding.
    3. Visual feedback (e.g., hover, focus states) is clear and consistent.
    4. The system default UI font is used throughout the application.

**Story 4.6: Accessibility Enhancements**
> As a user,
> I want to navigate and interact with the UI using only my keyboard,
> so that the application is accessible to users who cannot use a mouse.

- **Acceptance Criteria:**
    1. All interactive UI elements are navigable via the Tab key in a logical order.
    2. Interactive elements (buttons, text inputs, checkboxes) can be activated using Enter or Space.
    3. Clear focus indicators are visible for the currently selected UI element.
    4. Labels are programmatically associated with their respective controls for screen reader compatibility.

**Story 4.7: Micro-interactions and Feedback**
> As a user,
> I want subtle animations and visual cues for key interactions and state changes,
> so that the application feels responsive and provides clear feedback.

- **Acceptance Criteria:**
    1. A subtle animation (e.g., quick fade, color transition) is applied when the application's status changes from "RUNNING" to "STOPPED" and vice-versa.
    2. Interactive elements transition smoothly when becoming disabled or enabled.
    3. A brief visual cue is provided when a key press is successfully captured in a hotkey input field.

### Epic 5: Technical Debt & Refinement

*Goal: To address identified technical debt and optimize application performance.*

**Story 5.1: Tech Debt - Consolidate Input Libraries**
> As a developer,
> I want to consolidate the input handling libraries (`rdev` and `global-hotkey`) into a single, cohesive solution,
> so that the project reduces its dependency count, simplifies maintenance, and eliminates potential conflicts or redundant functionality.

- **Acceptance Criteria:**
    1. The project uses only one primary library for both mouse simulation and global hotkey detection.
    2. The chosen library fully supports both functionalities required by the application.
    3. All existing mouse clicking and hotkey toggling functionality remains intact.
    4. The codebase is updated to use only the consolidated input library.
    5. The `Cargo.toml` file reflects the removal of the unnecessary input library.

**Story 5.2: Refinement - Hotkey Polling Optimization**
> As a developer,
> I want to optimize the hotkey detection mechanism,
> so that the application consumes fewer CPU resources and is more energy-efficient.

- **Acceptance Criteria:**
    1. The hotkey detection no longer relies on a fixed-interval polling mechanism within `app.rs`.
    2. The application utilizes an event-driven mechanism for hotkey detection, if supported by the underlying `global-hotkey` crate and `iced` framework.
    3. The responsiveness of the hotkey toggle remains consistent or improves.
    4. CPU usage related to hotkey detection is measurably reduced.

## 7. Next Steps

- **PRD Review:** Review the updated PRD with project stakeholders to ensure alignment on current status and future priorities.
- **Story Creation:** Create new user stories and technical stories based on the updated Epic list and the identified technical debt (removing `rdev`, optimizing hotkey polling).
- **Prioritization:** Prioritize the newly created stories for future development sprints.
