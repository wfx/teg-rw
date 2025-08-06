## Features

- Flexible board structures loaded from RON files
- Support for custom rule sets (TEG, Risk-like, abstract, etc.)
- TUI and GUI frontends supported
- Networked multiplayer planned

## GameDefinition Model

Each game is defined in a `.game.ron` file and references all required components explicitly.

Example:

```ron
GameDefinition(
    id: "teg",
    name: "TEG Classic",
    rule: "",
    board: "",
    pieces: "",
    cards: "classic",
)
```

This loads:

- `teg.rule.ron`
- `teg.board.ron`
- `teg.pieces.ron`
- `classic.cards.ron`

No other files are loaded unless explicitly listed.

### Resolution Rules

Each component field maps to a file with a fixed suffix:

| Field    | Suffix         |
|----------|----------------|
| `rule`   | `.rule.ron`    |
| `board`  | `.board.ron`   |
| `pieces` | `.pieces.ron`  |
| `cards`  | `.cards.ron`   |
| `dices`  | `.dices.ron`   |

- If the field is `""`, the `id` is used as prefix.
- If the field is `"custom"`, `"custom.<suffix>.ron"` is loaded.
- If the field is missing, no file is loaded.

## Responsibility

The game designer must ensure all referenced files exist and are compatible. There is no fallback or implicit behavior.

## Project Structure

```
base/     ← Core logic (field structure, game state, turn events)
ui/       ← User interfaces (tui and gui)
net/      ← Networking components
wad/      ← Game definitions and related assets (Where's All the Data?)
```

## Architecture & Module Layout

We follow a unified schema:

```
src/
├── error.rs         // DataError & related error types
├── validator.rs     // Validatable trait + check_file helper
├── loader.rs        // Generic load_ron<T: Deserialize + Validatable>()
├── board.rs         // BoardDefinition + impl Validatable
├── rule.rs          // RuleDefinition + impl Validatable
├── pieces.rs        // PiecesDefinition + impl Validatable
├── cards.rs         // CardsDefinition + impl Validatable
├── dice.rs          // DiceDefinition + impl Validatable
├── game.rs          // GameDefinition + impl Validatable
└── lib.rs           // pub use … for the public API
```

## Coding Style

1. **Language:** English in code & documentation  
2. **Graphics:** use any UI backend (GTK, QT, Web, SDL, etc.)  
3. **Documentation:** `///` for items, `//!` for module-level docs  
4. **Modularity:** no duplicate load/validate logic  
5. **Reuse:** shared logic only in `validator.rs`, `loader.rs`, and other relevant modules.
