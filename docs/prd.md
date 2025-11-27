
# SuperClicker Product Requirements Document (PRD)

## 1. Goals and Background Context

### Goals
- To provide users with a simple and efficient tool for automating mouse clicks.
- To create a lightweight native Windows application that runs in the background with minimal performance impact.
- To offer intuitive controls, including hotkeys for starting/stopping and dynamically adjusting the click interval.

### Background Context
Many users, particularly in gaming and data entry, perform highly repetitive clicking tasks. This manual process can be tedious and inefficient. SuperClicker aims to solve this by providing a reliable and easy-to-use auto-clicker. Unlike some existing solutions, it will provide a straightforward way to dynamically control the click speed and select the desired mouse button, running unobtrusively in the background.

### Change Log
| Date       | Version | Description      | Author |
| :--------- | :------ | :--------------- | :----- |
| 2025-10-29 | 1.0     | Initial Draft    | John   |

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

- **Epic 1: Foundational Setup & Core Click Logic:** Establish the project structure and implement the fundamental clicking mechanism and the main application window.
- **Epic 2: Advanced Control and Hotkeys:** Integrate system-wide hotkeys for controlling the application and dynamically adjusting the click interval.
- **Epic 3: Hotkey Customization:** Implement the UI and logic for user-configurable hotkeys.

## 6. Epic Details

### Epic 1: Foundational Setup & Core Click Logic

*Goal: To create a basic, functional application that can be manually started and stopped from the UI, can perform clicks at a set interval, and allows the user to select the mouse button.*

**Story 1.1: Project Scaffolding and Main Window**
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

**Story 1.2: Basic Clicking Logic**
> As a user,
> I want the application to repeatedly click the mouse at a specified interval when I press a "Start" button,
> so that I can automate repetitive clicks.

- **Acceptance Criteria:**
    1. The main window has "Start" and "Stop" buttons.
    2. When "Start" is clicked, the application begins clicking the selected mouse button at the current cursor location.
    3. The click interval is based on the value in the interval input field.
    4. When "Stop" is clicked, the clicking ceases.
    5. The status text in the window updates to "Running" or "Stopped" accordingly.

### Epic 2: Advanced Control and Hotkeys

*Goal: To enable full background control of the application using system-wide hotkeys for a seamless user experience.*

**Story 2.1: Toggle Hotkey**
> As a user,
> I want to press a global hotkey to start and stop the auto-clicker,
> so that I don't have to switch to the application window.

- **Acceptance Criteria:**
    1. The application uses the user-configured hotkey to toggle the auto-clicker on and off.
    2. The hotkey works even when the SuperClicker application is not the active window.
    3. The "Start" and "Stop" buttons in the UI are removed or disabled, as the hotkey is now the primary control.

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

## 7. Next Steps

- **UX Expert Prompt:** Please review the UI/UX goals. The vision is a minimal, functional UI. We need to ensure the layout is intuitive for the single-window design.
- **Architect Prompt:** Please review the technical assumptions and the proposed epics/stories to create a suitable architecture document. The key challenges will be implementing reliable global hotkeys and ensuring minimal performance overhead.
