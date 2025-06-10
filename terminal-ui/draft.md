I want to make a representation about terminal user interfaces TUI with rust. Here are the points I want to talk about...
```
* Brief history and definitions about ANSI and TUI. 
  - I wanted to mention the `DEC_VT100_terminal` as first device to use ANSI.
  - I want to mention turbo_vision as well with brief info about it.
* Concepts about TUI, 
  - like how we use ANSI escape characters and more (feel free to add here more points as you find it needed).
  - How we deal with the input as cells.
* Advantages to program your app as TUI:
  - Cross-platform
  - UI is just cells and we don't need to deal with other complications like screen resolutions.. etc
  - immediate rendering approach.
* Advantages for users:
  - Cross-platform
  - Quick and simple apps with fast startup time.
  - Keyboard driven workflow.
```
Give me the answer is raw md format.


-----------------------


# Terminal User Interfaces (TUI) in Rust

## 1. Brief History and Definitions

### ANSI and the DEC VT100 Terminal
- **DEC VT100 Terminal:** 
  Introduced in the late 1970s, the DEC VT100 terminal was one of the first devices to implement ANSI escape sequences. These sequences standardized text formatting, cursor movement, and other control functions, forming the basis for many modern terminal interfaces.

### Turbo Vision
- **Turbo Vision:**  
  Developed by Borland in the early 1990s, Turbo Vision was a framework for creating text-mode user interfaces. It provided features like windowing, menus, and event handling, influencing many later TUI libraries and approaches.

## 2. Concepts Behind TUIs

### ANSI Escape Characters
- **Control and Formatting:**  
  TUIs use ANSI escape sequences to manage text styling (colors, bold, underline) and control cursor positioning, allowing dynamic updates and interactive displays.
  
- **Terminal Manipulation:**  
  These escape codes help redraw parts of the screen without refreshing the entire display, enabling smooth transitions and responsive interfaces.

### Representing the Screen as Cells
- **Grid of Cells:**  
  The terminal display is treated as a grid of cells, where each cell holds a character and its style information. This model simplifies rendering as updates only involve modifying specific cells rather than the whole screen.

- **Input Handling:**  
  Inputs are processed as events (e.g., keystrokes) that correspond to changes in the cell grid, making it straightforward to map user actions to visual updates.

### Additional Concepts
- **Event-Driven Architecture:**  
  Many TUI applications adopt an event-driven approach, where user inputs and system events trigger updates to the UI.
  
- **State Management:**  
  The entire UI is often represented as a state model (the grid), which gets re-rendered when changes occur, ensuring consistency and responsiveness.

- **Rust Ecosystem for TUI:**  
  Rust offers libraries like [`tui-rs`](https://github.com/fdehau/tui-rs) and [`crossterm`](https://github.com/crossterm-rs/crossterm) that provide robust abstractions for handling ANSI sequences, input events, and rendering, making TUI development both efficient and safe.

## 3. Advantages of Programming TUIs

- **Cross-Platform Compatibility:**  
  TUIs run on any terminal emulator across different operating systems, ensuring broad accessibility.

- **Simplified UI Management:**  
  With the UI represented as a grid of cells, developers avoid complexities associated with varying screen resolutions and graphical rendering challenges.

- **Immediate Rendering Approach:**  
  Changes to the state of the interface are instantly reflected on the screen, offering a responsive experience without the overhead of complex redraw mechanisms.

## 4. Advantages for Users

- **Consistent Experience:**  
  Users enjoy the same interface regardless of the platform or device, providing a uniform experience.

- **Fast Startup and Performance:**  
  TUI applications are typically lightweight, leading to quick load times and minimal resource usage.

- **Keyboard-Driven Workflow:**  
  Emphasizing keyboard shortcuts and commands, TUIs allow for efficient, streamlined interaction, which is particularly beneficial for power users and those comfortable with command-line environments.


---------------------


# Understanding ANSI and Its Role in Terminal Interfaces

## What is ANSI?
- **ANSI** stands for the **American National Standards Institute**.
- ANSI is a private non-profit organization that oversees the creation, promulgation, and use of thousands of norms and guidelines that directly impact businesses in nearly every sector, including technology and electronics.

## ANSI Escape Sequences
ANSI escape sequences are standardized codes used to control various aspects of text output in terminal and console environments. Here’s what they do:

### Text Formatting
- **Styling Text:**  
  Modify text appearance by changing attributes such as bold, italic, underline, and even blinking text.
- **Color Control:**  
  Set foreground and background colors for text. For example, `\x1b[31m` changes the text color to red.

### Cursor Movement
- **Positioning the Cursor:**  
  Move the cursor to specific coordinates on the screen. This is useful for updating specific areas of the terminal without redrawing the entire screen.
- **Screen Manipulation:**  
  Clear parts of the screen or the entire display, enhancing the efficiency of updating the UI.

### Additional Functions
- **Screen Redraw:**  
  Enable smooth transitions and partial screen updates by directly addressing cell positions.
- **Creating Effects:**  
  Some sequences can trigger auditory signals (like a bell) or other visual effects that enhance user experience.

## Impact on Terminal User Interfaces (TUIs)
- **Cross-Platform Consistency:**  
  Because ANSI escape sequences are supported by most terminal emulators, applications built using them can run uniformly across different systems.
- **Simplified Rendering Model:**  
  With the terminal treated as a grid of cells, developers can focus on updating only the changed cells rather than redrawing the entire display.
- **Efficient Performance:**  
  Direct control over text and cursor positioning means that changes can be rendered almost immediately, resulting in highly responsive user interfaces.

## Practical Example
To illustrate, consider a simple example for setting text color:
- **Change Text Color to Red:**
  ```rust
  print!("\x1b[31mThis text will appear in red\x1b[0m");
