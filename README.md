![icon](https://github.com/wfx/teg/blob/master/docs/assets/teg_icono.png)Tenes Emapandas Graciela (Rust Rewrite)
=========================================

[![License: AGPL v3](https://img.shields.io/badge/License-AGPL%20v3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)

**teg-rs** is a complete rewrite of the classic [TEG game](https://github.com/wfx/teg) in Rust.

The goal is to create a clean, modern, and modular system for turn-based strategy games with customizable rulesets and board definitions.

## Features

- Flexible board structures loaded from RON files
- Planned support for different rulesets (TEG, Risk, custom)
- Support for TUI and GUI frontends
- Optional networked multiplayer

## Project structure

```text
base/     ← Core logic (field structure, game state, turn events)
ui/       ← User interfaces (tui and gui)
net/      ← Networking components
data/     ← Boards and related assets
rules/    ← Ruleset definitions and implementations
```

## License ##

This project is licensed under the GNU Affero General Public License v3.0 (AGPL-3.0).
See the LICENSE file for details.
