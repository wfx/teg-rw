## Features

- Flexible board structures loaded from RON files
- Planned support for different rulesets (TEG, Risk like, custom)
- Support for TUI and GUI frontends
- Optional networked multiplayer

## Project structure

```text
base/     ← Core logic (field structure, game state, turn events)
ui/       ← User interfaces (tui and gui)
net/      ← Networking components
wad/     ← Game definitions and related assets (Where's All the Data? :D )
```

## Architecture & Module Layout

We follow a unified schema:
src/
├── error.rs // DataError & related error types
├── validator.rs // Validatable trait + verify_file helper
├── loader.rs // Generic load_ron<T: Deserialize + Validatable>()
├── board.rs // BoardDefinition + impl Validatable
├── rule.rs // RuleDefinition + impl Validatable
├── pieces.rs // PiecesDefinition + impl Validatable
├── game.rs // GameDefinition + impl Validatable
└── lib.rs // pub use … for the public API

**Modules:**
- `error.rs`: all central `DataError` definitions  
- `validator.rs`: the `Validatable` trait and `check_asset` helper  
- `loader.rs`: `load_ron` parses RON + invokes `.validate()`  
- `<name>.rs`: each RON file’s `struct`/`enum` + its `impl Validatable`

## Coding Style

1. **Language:** English in code & docs  
2. **Graphics:** no GTK/Qt; use Web/SDL/…  
3. **Documentation:** use `///` for items, `//!` for module‐level  
4. **Modularity:** no duplicated load/validate logic  
5. **Reuse:** common code only in `validator.rs` & `loader.rs`
