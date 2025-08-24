# pomo

_A pomodoro project tracker that allows you to focus on your personal project and track your progress. Enjoy your focus time! 🍻_

### Features
  - ✅ Start pomodoro sessions
  - ✅ Create and manage projects with time
  tracking
  - ✅ View project status and time invested
  - ✅ Delete old projects
  - ✅ Colorful CLI interface

  ### Technical Details
  - Built with Rust for performance and reliability
  - SQLite database for persistent storage
  - Cross-platform support (Windows, Linux)
  - Standalone executable - no installation required

  ### Getting Started
  1. Download the appropriate binary for your
  OS
  2. Run from terminal: `./pomo-linux-x86_64` (or `pomo-windows-x86_64.exe`
   on Windows)
  3. Follow the interactive menu to start
  tracking!

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

run this
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
- [ ] add gui
