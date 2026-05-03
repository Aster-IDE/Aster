# Aster Architecture

## Design Philosophy
Aster wants to follow a simple design, nothing cluttered, but not too boring either, so we designed it to be easy on the eyes while also being simple.

---

## Core Architecture

### Simple UI Layout
- **Top Bar**: Simple File/Edit/View menu
- **Main Area**: Single text editor
- **Status Bar**: Basic info (line count, file type, modified status)

### Feature Scope (What's IN)
- Text editing with basic undo/redo
- File operations: New, Open, Save, Save As
- Support for: TXT, Markdown, RTF
- Basic find/replace
- Cherry blossom theme
- Simple status bar

### Feature Scope (What's OUT)
- No tabs/multi-file editing
- No file explorer
- No syntax highlighting
- No LSP/intellisense
- No git integration
- No extensions
- No complex settings
- No command palette
- No activity bar

### File Format Support
- **TXT**: Plain text with UTF-8 encoding
- **Markdown**: Text with .md extension, no special rendering (future enhancement)
- **RTF**: Basic RTF parsing (future enhancement)

### Theme System
Minimal theming - just cherry blossom colors:
- Background: Dark purple/pink gradient
- Text: Light pink/white
- Accent: Cherry blossom pink

## Development Approach
1. Start with basic text editing
2. Add file operations
3. Implement basic find/replace
4. Add RTF and Markdown support (rendering / visualization for what it would look like when formatted)
5. Polish UI and theme

This ensures Aster stays true to being a "Simpler Text Editor" while maintaining the beautiful aesthetic of the AsterIDE project.