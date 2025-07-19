# Rule Structure Documentation (TEG-style FSM)

This document explains the structure and semantics of the rule definition file used in the TEG project.
It defines valid game phases, transitions, actions, and goals in a data-driven format.

## 📌 `default_phase`
Defines the initial phase when the game begins.
```ron
default_phase: "setup_start"
```

## 🌀 `phases`
A mapping from phase names to allowed actions within that phase.
Each action includes:
- `result`: A mapping from symbolic return values to the next phase
- Optional `constraints`: Static parameters passed to the rule evaluator

### Example
```ron
"encounter": {
    encounter: {
        result: {
            "won": "change_ownership",
            "lost": "encounter",
            "continue": "encounter",
        },
        constraints: {
            min_origin_figures: 2,
            max_dice: 3,
            adjacency_required: true
        }
    },
    end_encounter: { result: { "ok": "redistribute" } }
}
```

## 🧩 Actions
Each action must be supported by the engine (see engine reference). Result values are symbolic, returned by the engine or rule logic.

Common result tokens:
- `"ok"`, `"next"`, `"done"`
- `"won"`, `"lost"`, `"continue"`
- `"eligible"`, `"ineligible"`
- `"next_player"`

## 🔧 `constraints`
Constraints are interpreted by the rule evaluator to limit or validate actions.
They are defined per action, and may include:
- `min_origin_figures`
- `max_move`
- `only_adjacent`
- `conquest_required`
- etc.

## 🎯 `goals`
List of possible win conditions.
Each goal includes a `type` and type-specific parameters. 

### Supported Types
#### `control_sets`
Player must fully control all fields in the given `sets`.
```ron
(type: "control_sets", sets: ["Asia"], min_total_figures: 9)
```

#### `control_total`
Player must control at least N fields.
```ron
(type: "control_total", field_count: 30)
```

#### `remove_player`
Another player must be eliminated. Includes fallback.
```ron
(type: "remove_player", target_player: 1, fallback_goal: (...))
```

## 🔄 Phase Flow Control
The result of each action determines the next phase, allowing:
- loops within a phase (e.g. multiple `encounter` actions)
- jumps to follow-up phases (e.g. `change_ownership`)
- phase completion (e.g. `end_phase` → `start_phase`)

## ✅ Design Goals
- Fully declarative
- Neutral vocabulary (no military terms)
- Extensible structure
- Validatable at load time
