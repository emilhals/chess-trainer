# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.10.1](https://github.com/emilhals/chess-trainer/compare/chess-trainer-v0.10.0...chess-trainer-v0.10.1) - 2026-11-27

### Other

- add web/dist and *.log to .gitignore

## [0.10.0](https://github.com/emilhals/chess-trainer/compare/chess-trainer-v0.9.1...chess-trainer-v0.10.0) - 2026-11-27

### Added

- *(handlers)* add event for toggling mode and cycling skin

### Other

- *(ui)* remove instruction_height for board/helper.rs
- *(ui)* rename header.rs to panel.rs
- *(ui)* rename instructions.rs to sidebar.rs

## [0.9.1](https://github.com/emilhals/chess-trainer/compare/chess-trainer-v0.9.0...chess-trainer-v0.9.1) - 2026-11-26

### Fixed

- *(ui)* make board squares actually squares

### Other

- *(ui)* use board grid to position sidebar, rank/file labels
- *(ui)* create grid from centered area and return the grid rect

## [0.9.0](https://github.com/emilhals/chess-trainer/compare/chess-trainer-v0.8.0...chess-trainer-v0.9.0) - 2026-11-24

### Added

- *(skins)* add color for streak number
- *(ui)* add BigNumber widget for current streak
- *(board)* show feedback badge for the last move
- *(trainer)* return the cursor when a wrong move is undone
- *(skins)* add feedback badge colors

### Fixed

- *(ui)* use the streak_area from layout instead of making a new one

## [0.8.0](https://github.com/emilhals/chess-trainer/compare/chess-trainer-v0.7.0...chess-trainer-v0.8.0) - 2026-11-23

### Fixed

- *(native)* add Tab key to TryFrom function
- *(ui)* remove old cell color setting
- *(trainer)* remove stored board geometry from UI

### Other

- *(ui)* move board rendering entry point to ui/board/mod.rs
- *(ui)* move cell render function to cell.rs
- *(ui)* make helper functions free
- *(trainer)* move last-move lookup to Board
- *(ui)* make trainer panels free functions
- *(ui)* move trainer panels into ui/views/trainer
- *(ui)* make trainer_ui the trainer view's mod.rs
- *(gitignore)* add .DS_STORE and .trunk-lock
- *(ui)* remove unused footer module
- *(ui)* move settings to popup/settings
- *(ui)* move trainer_ui.rs into ui/views
- *(ui)* move menu rendering into ui/views
- *(ui)* move main rendering function into ui/mod.rs
- *(ui)* move board rendering and piece art into ui/board

## [0.7.0](https://github.com/emilhals/chess-trainer/compare/chess-trainer-v0.6.0...chess-trainer-v0.7.0) - 2026-11-22

### Other

- *(handlers)* take the app before the key or mouse
- add native feature and workspace
- replace std::time with web_time
- remove file handling from PlayerData and Settings
- *(storage)* load and save application data through the storage abstraction
- *(storage)* add file storage backend
- *(storage)* add storage trait, in-memory backend and JSON helpers

## [0.6.0](https://github.com/emilhals/chess-trainer/compare/v0.5.0...v0.6.0) - 2026-11-19

### Other

- use app-level input types instead of crossterm events
- *(event)* remove unused sender and handler
- move terminal-specific code to native module

## [0.5.0](https://github.com/emilhals/chess-trainer/compare/v0.4.1...v0.5.0) - 2026-11-19

### Added

- add mouse support for moving pieces
- add mouse support to main menu

### Other

- *(trainer)* split click handling and add input guard
- *(ui)* show mode options in fixed positions
- borrow openings and add start_trainer
- update demo.tape

## [0.4.1](https://github.com/emilhals/chess-trainer/compare/v0.4.0...v0.4.1) - 2026-11-01

### Fixed

- *(deps)* add human_panic crate

## [0.4.0](https://github.com/emilhals/chess-trainer/compare/v0.3.0...v0.4.0) - 2026-11-01

### Fixed

- *(trainer)* correct line selection and progression

### Other

- *(deps)* remove unused dependency

## [0.3.0](https://github.com/emilhals/chess-trainer/compare/v0.2.0...v0.3.0) - 2026-10-28

### Added

- *(ui)* add mode selector to main menu
- *(menu)* add mode selector keyboard navigation

### Other

- *(menu)* improve opening selection layout
- *(ui)* add spacing between board and instructions
- *(ui)* improve Kbd widget sizing and padding
- *(state)* move popup logic into PopupState
- *(state)* extract menu state into separate struct
- *(deps)* add unicode-width
- add release-plz.yml

## [0.2.0](https://github.com/emilhals/chess-trainer/compare/v0.1.0...v0.2.0) - 2026-10-25

### Added

- add settings menu
- add board skins

### Fixed

- resolve formatting issue
- resolve clippy warnings
- correct misspelling
- move saving player data to after restoring the terminal

### Other

- Merge pull request #2 from emilhals/feat/board-skins
- update Rust toolchain and test workflow
- set up Rust workflow
- remove usage instruction
- add badges, features, usage instructions and supported openings
- add vhs tape and demo preview to README
- add Homebrew installation to README.md
- update README.md
- configure release-plz

## [0.1.0](https://github.com/emilhals/chess-trainer/releases/tag/v0.1.0) - 2026-10-20

### Added

- *(ui)* improve layout and mode-specific instructions
- *(trainer/openings)* add Instruction struct to hold move and play text
- *(trainer/ui)* add board position coordinates
- *(app)* add player color handling and more reset functionalities
- *(trainer/ui)* add helper function for finding height of instruction text
- add logic for finding a random next line
- add menu title and unicode icons for chess pieces
- *(player)* add player data for tracking best streak and completed lines
- *(openings)* add more informative instructions for learn mode
- *(app)* move "next line"-logic up to the App
- *(ui)* show selected mode, highest streak and completed lines for openings
- *(ui)* add top panel used to display current opening, line and mode
- *(ui)* add divider render function
- add replay current line functionality
- *(trainer)* add handling for when the player is Black
- *(app)* select opening from menu
- *(openings)* add Kings Indian Defense lines
- *(openings)* add more Italian Game lines
- undo move on board if a move is wrong and show response
- *(utils)* add util functions
- *(ui)* add helpers, widgets and UI handling for the trainer and main view
- *(trainer)* add logic and handling for trainer logic, chess movement, validation, and UI
- add code for event handling, main function
- *(data)* add chess openings
- *(pieces)* add code for chess pieces
- *(handlers)* add handlers for trainer and home
- *(state)* add state for the trainer and application UI

### Fixed

- propagate errors from function call
- *(trainer)* advance to next line after completing a line in Drill mode
- *(openings)* fix wrong move in Rosentreter Gambit
- *(pieces)* Use Bishop art instead of Queen
- *(handlers)* only allow mode change in home view
- *(ui)* adjust lengths in layouts
- *(widgets)* remove underline in the padding Span
- remove setting opening state in main.rs
- *(openings)* add/correct instructions for moves

### Other

- *(player-data)* store progress in user data directory
- add directories package
- add data/player.json to .gitignore
- *(openings)* embed opening data into binary
- add package metadata and optimization for releases
- add MIT license
- *(ui)* update chessboard color palette
- *(trainer)* log failed move execution
- *(deps)* enable ratatui rendered line info
- *(openings)* add validation test for checking valid moves for lines
- remove unused imports
- extract library crate from binary
- *(trainer)* split UI module into sub modules
- remove module
- remove unused variables and imports
- *(ui)* remove the popup displayed when a line is complete
- *(state)* remove previous view functionality
- *(ui)* put trainer home in main_menu.rs instead of main_ui.rs
- add files
- *(ui)* split Content-property into Key & Content-property.
- add Cargo files, .gitignore and README.md
