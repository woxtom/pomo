# pomo

_A pomodoro project tracker that allows you to focus on your personal project and track your progress. Enjoy your focus time! 🍻_

### Features
  - ✅ Start pomodoro sessions with customizable duration
  - ✅ Loop focus and rest cycles with pause controls
  - ✅ Create and manage projects with time tracking
  - ✅ View project status with visual progress bars
  - ✅ Delete old projects
  - ✅ **NEW** Journaling system - markdown notes for each project
  - ✅ Colorful CLI interface with platform-appropriate sounds

### Technical Details
  - Built with Rust for performance and reliability
  - SQLite database for persistent storage
  - Cross-platform support (Windows, Linux)
  - Standalone executable - no installation required
  - **NEW** Smart editor detection (notepad on Windows, $EDITOR/nano on Linux)

### Getting Started
  1. Download the appropriate binary for your OS
  2. Run from terminal: `./pomo-linux-x86_64` (or `pomo-windows-x86_64.exe` on Windows)
  3. Follow the interactive menu to start tracking!

### Focus Session Controls
- `'p'` toggles pause/resume during focus or rest
- `'q'` ends the current session immediately
- Configure focus and rest lengths before starting the loop

### Journaling Feature

Now you can keep detailed notes for each project! After focus sessions, the journal system allows you to:
- 📝 Write markdown notes in your preferred editor
- 🗂️ Organize journals by project with automatic file management
- 🔍 Easily edit journals outside the application
- 💾 Journals stored in platform-appropriate data directories

### Shortcuts in CLI

You may change the executable's name to pomo or whatever you like for simplicity!

#### Windows
1. Press Win + R, type sysdm.cpl, press Enter
2. Click Advanced tab → Environment Variables
3. Under User variables (for current user) or System variables (for all users), find Path
4. Click Edit → New
5. Add the full path to the directory containing your .exe
6. Click OK to save
7. Restart PowerShell

#### Linux

```bash
sudo cp ./pomo /usr/local/bin
```

Now you can invoke pomo by only typing `pomo` in your terminal! Enjoy your focus time!

### TODOs

- [x] add notification sounds
- [x] show status
- [x] make cli colorful
- [x] add shortcuts in cli
- [x] bundle it up as an executable
- [x] add journal feature
- [ ] add gui
