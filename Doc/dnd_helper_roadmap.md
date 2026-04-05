# D&D Helper — Rust Learning Roadmap

## Project Philosophy

- **Core logic as a library** (`dnd-core`), **CLI as a consumer** (`dnd-cli`), **UI comes last**
- No emulator, no UI framework until Milestone 8
- Everything is testable via `cargo run` in the terminal
- Learn by doing — write the code yourself, use Claude as a copilot (guidance, code reviews, unstucking)

---

## Stack

- **Language**: Rust
- **Architecture**: Cargo workspace with a `lib` crate + a `bin` crate
- **Persistence**: JSON files via `serde` → eventually SQLite
- **UI (later)**: Dioxus (desktop first, then mobile)

---

## Milestone 0 — Project Setup & First Struct

**Rust concepts**: cargo, workspaces, modules, structs, basic types, functions

**Goal**: Set up a cargo workspace with two crates and write your first struct.

**Tasks**:
- Create a cargo workspace with `dnd-core` (library) and `dnd-cli` (binary)
- `dnd-cli` depends on `dnd-core`
- In `dnd-core`: create an `AbilityScores` struct with the six stats (STR, DEX, CON, INT, WIS, CHA)
- Implement a method `modifier(score: u8) -> i8` that returns `(score - 10) / 2`
- In `dnd-cli`: create hardcoded ability scores, print each stat with its modifier

**Done when**: `cargo run` from `dnd-cli` prints something like:
```
STR: 16 (+3)
DEX: 12 (+1)
CON: 14 (+2)
...
```

**Resources**: The Rust Book chapters 3 (variables, types), 5 (structs), 7 (packages, crates, modules)

---

## Milestone 1 — Character Sheet

**Rust concepts**: enums, impl blocks, Display trait, Option, String vs &str

**Goal**: Model a full character sheet and display it.

**Tasks**:
- Create enums: `Race`, `Class`, `Skill` (list the ones you use in your campaigns)
- Create a `Character` struct with:
  - `name: String`, `surname: String`
  - `race: Race`, `class: Class`, `level: u8`
  - `ability_scores: AbilityScores`
  - `max_hp: u32`, `current_hp: u32`
  - `armor_class: u8`
  - `proficiency_bonus: u8`
  - `saving_throw_proficiencies: Vec<Ability>`
  - `skill_proficiencies: Vec<Skill>`
- Implement methods:
  - `apply_damage(&mut self, amount: u32)` — reduce current_hp, floor at 0
  - `heal(&mut self, amount: u32)` — increase current_hp, cap at max_hp
  - `get_proficiency_bonus(&self) -> u8` — derived from level
  - `get_skill_modifier(&self, skill: Skill) -> i8` — ability mod + proficiency if proficient
- Implement the `Display` trait for `Character` to pretty-print the sheet
- CLI: prompt the user to fill in each field, then display the full sheet

**Design note**: Damage is just a number — no type tracking. `apply_damage(19)` not `apply_damage(14, Piercing)`. Keep it simple.

**Done when**: You can create a character interactively and see a nicely formatted character sheet in the terminal.

**Resources**: The Rust Book chapters 6 (enums), 8 (vectors), 10.2 (traits)

---

## Milestone 2 — Status Effects

**Rust concepts**: Vec, HashSet, enums with data, pattern matching, derive macros

**Goal**: Track active status effects on your character.

**Tasks**:
- Create a `StatusEffect` enum (Poisoned, Stunned, Frightened, Blinded, Charmed, Paralyzed, Electrified... whatever your table uses)
- Add `active_statuses: HashSet<StatusEffect>` to `Character` (or `Vec` if you prefer)
- Methods: `add_status(&mut self, status)`, `remove_status(&mut self, status)`, `has_status(&self, status) -> bool`, `clear_statuses(&mut self)`
- Update your `Display` impl to show active statuses on the character sheet
- CLI commands: `status add poisoned`, `status remove poisoned`, `status list`, `status clear`

**Done when**: You can add/remove statuses and see them on your character sheet.

**Resources**: The Rust Book chapters 6.2 (match), 8.2 (HashSet is in std::collections)

---

## Milestone 3 — Save & Load

**Rust concepts**: serde, file I/O (std::fs), Result<T, E>, error handling, the ? operator, derive macros

**Goal**: Persist your character to disk and load it back.

**Tasks**:
- Add `serde`, `serde_json` as dependencies to `dnd-core`
- Add `#[derive(Serialize, Deserialize)]` to ALL your types (Character, AbilityScores, Race, Class, Skill, StatusEffect, etc.)
- Implement `save_character(&self, path: &str) -> Result<(), Error>`
- Implement `load_character(path: &str) -> Result<Character, Error>`
- Handle errors properly: file not found, invalid JSON, permission errors
- CLI: `save` and `load` commands, auto-load on startup if a save file exists

**Challenge**: Stop using `.unwrap()` — handle every `Result` properly with `?` or `match`.

**Done when**: You can create a character, save it, close the program, reopen it, and your character is still there.

**Resources**: The Rust Book chapter 9 (error handling), serde.rs documentation

---

## Milestone 4 — Inventory System

**Rust concepts**: ownership & borrowing (THE big one), iterators, &self vs &mut self, lifetime basics

**Goal**: A working inventory with add/remove/search/weight tracking.

**Tasks**:
- Create an `Item` struct: `name: String`, `description: String`, `weight: f32`, `quantity: u32`
- Create an `Inventory` struct wrapping a `Vec<Item>`
- Methods:
  - `add_item(&mut self, item: Item)` — if item with same name exists, increase quantity
  - `remove_item(&mut self, name: &str, quantity: u32) -> Result<(), Error>` — fail if not enough
  - `list_items(&self)` — display all items
  - `find_item(&self, name: &str) -> Option<&Item>`
  - `total_weight(&self) -> f32`
- Add `inventory: Inventory` to `Character`
- CLI commands: `inv add 3 "Healing Potion"`, `inv remove 1 "Rope"`, `inv list`, `inv weight`
- Parse the CLI input string to extract quantity and item name
- Don't forget to update your serde derives so inventory gets saved/loaded too

**Warning**: This is where the borrow checker fights you. When you try to find an item and then modify it, Rust will complain. Work through it — this is the most important learning milestone.

**Done when**: You can manage a full inventory, and it persists across saves.

**Resources**: The Rust Book chapter 4 (ownership — re-read this one), chapter 13 (iterators)

---

## Milestone 5 — Better CLI & UX

**Rust concepts**: external crates, traits for abstraction, string parsing, code organization

**Goal**: Make the CLI actually pleasant to use.

**Tasks**:
- Add a crate like `inquire` or `dialoguer` for interactive prompts/menus
- Alternatively, use `clap` for a command-based interface
- Create a main loop with a clear menu:
  - `sheet` — display character sheet
  - `damage <amount>` — apply damage
  - `heal <amount>` — heal
  - `status add/remove/list/clear`
  - `inv add/remove/list/search/weight`
  - `save` / `load`
  - `quit`
- Add input validation: don't crash on bad input, show helpful error messages
- Add colored output if you want (crate: `colored`)

**Done when**: Someone unfamiliar with your code could use the CLI without confusion.

**Resources**: docs.rs pages for whichever crate you pick

---

## Milestone 6 — Crafting System

**Rust concepts**: HashMap, complex data relationships, trait implementations, data-driven design

**Goal**: A crafting system connected to your inventory.

**Tasks**:
- Create a `Recipe` struct:
  - `name: String`
  - `ingredients: HashMap<String, u32>` (item name → quantity needed)
  - `result: Item` (what gets crafted)
- Store recipes in a `recipes.json` file (data-driven, easy to expand)
- Load recipes on startup with serde
- Implement `craft(&mut self, recipe_name: &str, recipes: &[Recipe]) -> Result<Item, CraftError>`
  - Check if all ingredients are in inventory
  - If yes: remove ingredients, add result item, return Ok
  - If no: return an error saying what's missing
- CLI: `craft list` (show available recipes), `craft <recipe_name>` (attempt to craft)
- Bonus: show which recipes you *can* craft with current inventory

**Done when**: You can load recipes from a file, check ingredients, craft items, and see your inventory update.

**Resources**: The Rust Book chapter 8.3 (HashMap), chapter 12 (a project tying things together)

---

## Milestone 7 — Polish & Architecture Review

**Rust concepts**: testing, modules, documentation, refactoring, idiomatic Rust

**Goal**: Turn your code from "it works" into "it's good Rust."

**Tasks**:
- Split `lib.rs` into modules: `character.rs`, `inventory.rs`, `crafting.rs`, `types.rs`
- Write unit tests:
  - Damage reduces HP correctly, doesn't go below 0
  - Healing doesn't exceed max HP
  - Adding duplicate items increases quantity
  - Removing more items than available returns an error
  - Crafting consumes ingredients and produces result
  - Crafting fails gracefully when ingredients are missing
- Add `///` doc comments to all public types and methods
- Run `cargo doc --open` and review your documentation
- Run `cargo clippy` and fix all warnings
- Run `cargo fmt` to format everything

**Done when**: `cargo test` passes, `cargo clippy` is clean, `cargo doc` looks good, and you're proud of the code.

**Resources**: The Rust Book chapter 11 (testing), chapter 14 (cargo, crates.io, documentation)

---

## Milestone 8 — UI (When You're Ready)

**Rust concepts**: component architecture, state management, cross-compilation, async basics

**Goal**: Build a visual frontend that uses your existing library.

**Tasks**:
- Add Dioxus as a dependency in a new `dnd-ui` crate in your workspace
- Start with desktop target (easier to debug)
- Build screens: character sheet view, inventory management, crafting panel
- All logic comes from `dnd-core` — the UI just calls your existing methods
- Once desktop works: configure for Android/iOS

**Done when**: You have a working desktop app that does everything your CLI does, but with a GUI.

**Resources**: Dioxus documentation (dioxuslabs.com), their Getting Started guide

---

## Future Ideas (No Rush)

- Multiple characters per campaign
- Party inventory / shared loot
- Initiative tracker for encounters
- Spell slot tracking + rest mechanics (short rest / long rest)
- Homebrew item and recipe editor
- SRD spell database with search and filters
- Export character sheet to PDF
- SQLite instead of JSON for persistence
- Sync between devices

---

## When You're Stuck

1. Read the compiler error carefully — Rust's errors are unusually helpful
2. Re-read the relevant Rust Book chapter
3. Check if someone had the same issue on Stack Overflow or the Rust Users Forum
4. Ask Claude — share your code AND the compiler error, and work through it together
5. r/rust and the Rust Discord are very beginner-friendly

---

*Remember: the goal is to learn Rust, not to ship fast. Take your time on each milestone. The compiler is your teacher — listen to it.*
