# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
