# Learning Rust Testing — A Slow-Pace Guide for Grand Auto-Battler

> **Scope:** We test only what your two Rust projects already do:
> `GrandAutoBattler_GameEngine` (the library) and `GrandAutoBattler_API_Runner` (the axum server that consumes it).
> We do **not** do TDD (no designing new features through tests) and we do **not** touch the Console app.
>
> **How to use this guide:** Each section = theory → copy-paste code → line-by-line explanation → a **YOUR TURN** drill
> (its solution hidden in a `<details>` spoiler). Do the drill yourself *before* opening the spoiler.
>
> **Your toolchain:** both crates use `edition = "2024"` (requires Rust ≥ 1.85). Where a feature needs a newer compiler
> (e.g. `assert_matches!` in `std` needs Rust ≥ 1.96), I mark it and give the older fallback. Check yours: `rustc --version`.

---

## Section 0 — Theory: What a Rust Test Actually Is

### 0.1 The two ingredients

Every Rust test is just an ordinary function with one attribute attached:

```rust
#[test]
fn my_first_test() {
    // body: arrange, act, assert
}
```

- `#[test]` is an **attribute macro** from the standard library. It tells the compiler:
  "compile this function and register it with the built-in test runner (libtest)."
- A test function **returns nothing** (`()`) and **takes no arguments**.
- If any `assert` inside the body fails, the function **panics**, and the test runner reports that test as FAILED.
  No panics → test PASSED. That's the whole mechanism: *a test passes when it does not panic.*

### 0.2 `#[cfg(test)]` — code that only exists during testing

Everything that is only needed for tests goes into a module guarded by `#[cfg(test)]`:

```rust
#[cfg(test)]
mod tests {
    use super::*; // imports everything the parent module can see
}
```

- `cfg(test)` is a **compile-time condition**. When you run `cargo build` or `cargo run`,
  the whole module is **physically removed from the compiled binary**. It costs you zero size and zero speed.
  Only `cargo test` compiles it in.
- The `use super::*;` line is important: it imports the parent module's items *including private ones*,
  which is exactly what makes **unit tests** able to touch private functions.

### 0.3 Unit tests vs Integration tests (the Rust community's two buckets)

| | **Unit tests** | **Integration tests** |
|---|---|---|
| **Where they live** | Inside `src/`, in the *same file* as the code | In a `tests/` folder at the crate root |
| **Visibility** | Can call **private** `fn`s (via `use super::*`) | Can use **only the public API**, like any external consumer |
| **Purpose** | Edge cases, one function's behavior | "Do the pieces work together the way a user sees them?" |

Your `GrandAutoBattler_GameEngine` is a **library crate** (`src/lib.rs`), which means both kinds are natural.
`GrandAutoBattler_API_Runner` is a **binary** (`src/main.rs`), and its testable logic (types, handlers)
is tested with in-file unit tests.

### 0.4 The assertion toolbox (std only — no extra crates)

| Macro | Use it when |
|---|---|
| `assert!(bool, "message")` | You are checking a condition |
| `assert_eq!(a, b, "message")` | Two values must be equal (prints both on failure) |
| `assert_ne!(a, b)` | Two values must differ |
| `matches!(value, Pattern)` | You care *which variant* of an enum it is, not the payload |
| `assert_matches!(value, Pattern)` | Same, but as a test assertion with a nice failure message — **stable in `std` since Rust 1.96** (`use std::assert_matches::assert_matches;`). On older toolchains, use `assert!(matches!(...))` instead |

The string after the values is a **failure message** — it is printed only when the assertion fails.
Always make it explain *what state surprised you*: `assert_eq!(hp, 100, "expected full HP but got {hp}")`.

### 0.5 Naming: tests are living documentation

A good test name reads like a sentence: **what the unit does + under what state + the expected behavior.**
Convention (from Apollo's Rust best-practices):

```rust
#[cfg(test)]
mod tests {
    mod generate_health_points {              // <- which unit you test
        #[test]
        fn middle_parameter_gives_basic_health() { /* arrange / act / assert */ }
    }
}
```

Run `cargo test generate` and the output `tests::generate_health_points::middle_parameter_gives_basic_health`
tells you exactly what broke. Generic names like `test1` or `test_happy_path` defeat the purpose.

### 0.6 One behavior per test

A test that fails should make it **obvious which behavior broke**. If a test asserts three unrelated things,
a failure forces you to debug all three. Rule: *one test = one behavior.*
(Checking several *fields of one resulting struct* is acceptable — it is still one behavior;
but "parses valid input" AND "rejects garbage" = two tests.)

### 0.7 Running tests — the PowerShell commands you will use

Work **inside the crate folder** (each crate here has its own `Cargo.toml`, there is no workspace):

```powershell
cd d:\REPOS\Rust\FinalRustProject\GrandAutoBattler\GrandAutoBattler_GameEngine
cargo test                                    # all tests in this crate
cargo test generate_health_points             # only tests whose name CONTAINS this string
cargo test -- --nocapture                     # also print any println!/eprintln! output
cargo test -- --list                          # list every test without running
cargo test --lib                              # only in-source (unit) tests
cargo test --tests                            # only tests/ folder (integration) tests
cargo test --doc                              # only doc tests (Section 8)
```

(Anything after the final `--` goes to the test runner, not to cargo.)

In **VS Code** you can also hover over any `#[test]` fn and click the **"Run test"** code-lens —
handy while iterating on a single test.

### 0.8 `unwrap()` / `expect()` are allowed here

In production code we avoid `unwrap()` (it panics). **In tests, `unwrap()`/`expect()` are the norm** —
a test helper failing *should* panic, and the failure message should say why.
Prefer `expect("why this cannot be None")` so the panic text helps you.

---

## Section 1 — Your First Test: a Private, Pure Function

**Target:** [`parse_config_from_str()`](GrandAutoBattler_GameEngine/src/toml_parser.rs:56) in
`toml_parser.rs`.

Why this is the perfect first target:

- It is **private** (`fn`, not `pub fn`) — unit tests can still see it.
- It is **pure**: input `&str` → output `Result<Config>`. No filesystem, no randomness, no global state.
  That means the result is *deterministic* — you can write the expected value down and it will always hold.
- It already has an **error path** to test (garbage TOML → `Err`).

This is the ideal test shape, and most of your engine's "core logic" will look like this.

### 1.1 The test (happy path)

Open `GrandAutoBattler_GameEngine/src/toml_parser.rs` and append to the **very end of the file**:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    mod parse_config_from_str {
        use super::*;

        #[test]
        fn parses_valid_config_toml() {
            // ARRANGE: a string shaped exactly like assets/Settings/GameSettings.toml
            let toml_string = r#"
PARAMETER_MINIMUM = 0
PARAMETER_MAXIMUM = 9
BASIC_HEALTH = 100.0
BASIC_ATTACK = 10.0
ATTACK_PERCENTAGE_PER_POINT = 5.0
DEFENSE_PERCENTAGE_PER_POINT = 5.0
HEALTH_PERCENTAGE_PER_POINT = 5.0
ATTACK_MIDDLE_VALUE_PARAMETER = 4.0
DEFENSE_MIDDLE_VALUE_PARAMETER = 2.0
HEALTH_MIDDLE_VALUE_PARAMETER = 4.0
"#;

            // ACT: call the function under test
            let config = parse_config_from_str(toml_string)
                .expect("this valid TOML must parse");

            // ASSERT: check the fields (they are pub(crate), visible from this module)
            assert_eq!(config.PARAMETER_MAXIMUM, 9, "PARAMETER_MAXIMUM should be 9");
            assert_eq!(config.BASIC_HEALTH, 100.0, "BASIC_HEALTH should be 100.0");
            assert_eq!(config.DEFENSE_MIDDLE_VALUE_PARAMETER, 2.0, "DEFENSE_MIDDLE_VALUE_PARAMETER should be 2.0");
        }
    }
}
```

Then run: `cargo test parse_config_from_str`

### 1.2 Line-by-line explanation

- `#[cfg(test)]` — this whole module is compiled **only** during `cargo test`.
- `mod tests { use super::*; }` — `super` is `toml_parser`. The import pulls in `parse_config_from_str`
  *and* the module's own imports (`GameEngineError`, `Result`, `Config`) — all **including private items**,
  because the test module is a child of `toml_parser`.
- `mod parse_config_from_str { use super::*; }` — the inner module is named after the **unit under test**.
  Its `use super::*` reaches back to `toml_parser` (two levels up, via `tests`).
  The name costs nothing and makes `cargo test` output self-documenting.
- `r#"..."#` — a **raw string literal** (`#"` variant) so you don't have to escape the newlines and quotes
  inside the TOML. Its content mirrors your real
  [`GameSettings.toml`](GrandAutoBattler_GameEngine/assets/Settings/GameSettings.toml) —
  tests should use inputs that look like real data.
- `expect("this valid TOML must parse")` — `Result::expect` unwraps the `Ok` or panics with your message.
  If *this* panics, the test fails loudly and tells you the input was the problem, not the assertions.
- `assert_eq!(config.PARAMETER_MAXIMUM, 9, ...)` — `Config`'s fields are `pub(crate)`, and since the test
  lives *inside the same crate*, it can read them directly. `assert_eq!` on `u8` works because `u8`
  implements `PartialEq` and `Debug` (both needed for the macro to print on failure).

**Watch the output closely.** You will see:

```
running 1 test
test toml_parser::tests::parse_config_from_str::parses_valid_config_from_str ... ok
```

The path `toml_parser::tests::parse_config_from_str::parses_valid_config_from_str` is the "living documentation"
we discussed: file → unit → behavior.

### 1.3 YOUR TURN

Write the **error-path** test in the same module: give `parse_config_from_str` a garbage string
(e.g. `"this is [not valid toml"`) and assert that it fails **with the `TomlParse` variant** of
[`GameEngineError`](GrandAutoBattler_GameEngine/src/error_types.rs:7), not just "some error".
(Use `matches!` inside `assert!`, and `expect_err` to unwrap the `Err` side.)

<details>
<summary>Solution</summary>

```rust
#[test]
fn returns_toml_parse_error_for_garbage_string() {
    let err = parse_config_from_str("this is [not valid toml")
        .expect_err("garbage input must fail to parse");

    assert!(
        matches!(err, GameEngineError::TomlParse(_)),
        "expected a TomlParse error, got: {err:?}"
    );
}
```

Notes:
- `expect_err` is the mirror of `expect`: it takes the `Err` payload or panics (i.e., fails the test) if it was `Ok`.
- `matches!(err, GameEngineError::TomlParse(_))` checks the **variant** without caring about the inner
  `toml::de::Error` value (that error type is not `PartialEq`, so you cannot `assert_eq!` it).
- The `_{err:?}` in the failure message uses `Debug` to print the whole error if you got it wrong.

If your Rust is ≥ 1.96 you can replace the `assert!` with the dedicated macro:

```rust
use std::assert_matches::assert_matches;

// inside the test:
assert_matches!(err, GameEngineError::TomlParse(_));
```

</details>

---

## Section 2 — Structs, Equality, and `From` Conversions

**Target:** [`UnitType::from(UnitTypesToml)`](GrandAutoBattler_GameEngine/src/game_data_retriever.rs:67) in
`game_data_retriever.rs`.

New ideas in this section:

1. **Testing a conversion** — your `From` impl builds a `BTreeMap` from flat TOML fields and applies an
   `Option` fallback (`females_only.unwrap_or(false)`). Conversions are one of the most bug-prone places
   in a codebase, and they are perfectly testable: pure in, pure out.
2. **Asserting on `Option`** — `assert_eq!(x, None)` and `assert_eq!(x, Some(v))` work out of the box.
3. **Indexing a `BTreeMap` in tests** — `map["key"]` is fine in tests (it panics on missing key,
   which is a useful failure).

### 2.1 The test

Append to the end of `GrandAutoBattler_GameEngine/src/game_data_retriever.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    mod unit_type_from_toml {
        use super::*;
        use crate::toml_parser::UnitTypesToml; // pub(crate), so visible from within the crate

        #[test]
        fn converts_imp_toml_into_unit_type_with_default_gender() {
            // ARRANGE: the "imp" row from assets/Factions/Demons/UnitTypes.toml, as a struct
            let toml_unit = UnitTypesToml {
                unit_type_name: "imp".to_string(),
                unit_type_pretty_name: Some("Imp".to_string()),
                females_only: None,
                health: 8,
                attack: 2,
                range: 8,
                defense: 2,
            };

            // ACT
            let unit_type = UnitType::from(toml_unit);

            // ASSERT: identity fields passed through...
            assert_eq!(unit_type.unit_type_name, "imp");
            assert_eq!(unit_type.unit_type_pretty_name, Some("Imp".to_string()));
            // ...the Option fallback applied...
            assert!(!unit_type.females_only, "None must fall back to false");
            // ...and the parameters landed in the map under the right keys
            assert_eq!(unit_type.parameters_set["health"], 8);
            assert_eq!(unit_type.parameters_set["attack"], 2);
            assert_eq!(unit_type.parameters_set["range"], 8);
            assert_eq!(unit_type.parameters_set["defense"], 2);
        }
    }
}
```

Run: `cargo test unit_type_from_toml`

### 2.2 Line-by-line explanation

- `use crate::toml_parser::UnitTypesToml;` — the struct is `pub(crate)`: visible **everywhere inside this crate**,
  which includes tests. (An integration test in `tests/` could *not* use it — that's the public-API rule
  you'll meet in Section 6.)
- We build the `UnitTypesToml` **as a literal**, not by parsing a string — for a conversion test,
  you're testing *mapping logic*, not the parser again (that was Section 1). Simpler input, sharper failure.
- `UnitType::from(toml_unit)` — calling the `From` impl directly; `toml_unit.into()` would do the same.
  Calling `From` explicitly in tests makes the intent obvious.
- `assert_eq!(unit_type.unit_type_pretty_name, Some("Imp".to_string()))` — comparing `Option`s directly.
  If you wrote `Some("Imp")` (a `&str`) it would **not compile**: `Option<String> != Option<&str>`.
  This is a type error, which is exactly Rust doing its job.
- `assert!(!unit_type.females_only, ...)` — this is the **fallback behavior** under test:
  `None.unwrap_or(false)`. If someone "fixes" the `From` impl to `unwrap_or(true)`, *only this line* fails
  and tells you the gender default flipped. That's a test earning its keep.
- `unit_type.parameters_set["health"]` — `BTreeMap::Index<&str>`. Panics if the key is missing,
  so a wrong key name in the `From` impl fails the test immediately.

A few fields asserted in one test here is fine: it is still **one behavior** ("the conversion is correct"),
and splitting "health field is mapped" into its own test would be noise. The one-assertion rule targets
*behaviors*, not struct fields.

### 2.3 YOUR TURN

Test the **second fallback** in the same `From` impl's spirit, but on a different unit:
[`Faction::new()`](GrandAutoBattler_GameEngine/src/game_data_retriever.rs:24). Assert that a freshly
created faction has the given name **and** that all five collection fields start empty.
(Put it in its own `mod faction_new` inside the same `tests` module — a new unit = a new module.)

<details>
<summary>Solution</summary>

```rust
mod faction_new {
    use super::*;

    #[test]
    fn new_faction_starts_with_empty_collections() {
        let faction = Faction::new("Demons".to_string());

        assert_eq!(faction.name, "Demons");
        assert!(faction.unit_types.is_empty(), "unit_types should start empty");
        assert!(faction.male_first_names.is_empty(), "male_first_names should start empty");
        assert!(faction.male_last_names.is_empty(), "male_last_names should start empty");
        assert!(faction.female_first_names.is_empty(), "female_first_names should start empty");
        assert!(faction.female_last_names.is_empty(), "female_last_names should start empty");
    }
}
```

Why this test matters for your game: `Faction::new` is called in
[`load_factions()`](GrandAutoBattler_GameEngine/src/toml_parser.rs:93) right before names are loaded
*conditionally*. If the `new()` constructor ever silently starts with pre-filled vectors, battles would
use phantom units — and this test is the alarm.

</details>

---

## Section 3 — Deterministic Math: Hand-Compute the Expected Value

**Target:** [`Combatant::generate_health_points()`](GrandAutoBattler_GameEngine/src/combatant_manager.rs:88) in
`combatant_manager.rs`.

### 3.1 Theory: how to test a formula

A formula like

```text
hp = BASIC_HEALTH + (BASIC_HEALTH * PERCENT * (param - MIDDLE)) / 100
```

is fully deterministic: same inputs → same output. The discipline is:

1. **Freeze the inputs** — build the exact `Config` your [`GameSettings.toml`](GrandAutoBattler_GameEngine/assets/Settings/GameSettings.toml) contains (do *not* call `get_config()` here — that reads from disk relative to the executable, which is a different concern; Section 6 explains why).
2. **Hand-compute the expected value on paper** (I show the arithmetic in comments).
3. Pick inputs that cover the **interesting regions**: the middle value (zero bonus), the max, the min.

### 3.2 The test

Append to `GrandAutoBattler_GameEngine/src/combatant_manager.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_data_retriever::Config;

    /// A Config identical to assets/Settings/GameSettings.toml.
    /// Shared by every test in this file — this is "arrange" boilerplate, and
    /// sharing setup (not assertions) is good practice.
    fn default_config() -> Config {
        Config {
            PARAMETER_MINIMUM: 0,
            PARAMETER_MAXIMUM: 9,
            BASIC_HEALTH: 100.0,
            BASIC_ATTACK: 10.0,
            ATTACK_PERCENTAGE_PER_POINT: 5.0,
            DEFENSE_PERCENTAGE_PER_POINT: 5.0,
            HEALTH_PERCENTAGE_PER_POINT: 5.0,
            ATTACK_MIDDLE_VALUE_PARAMETER: 4.0,
            DEFENSE_MIDDLE_VALUE_PARAMETER: 2.0,
            HEALTH_MIDDLE_VALUE_PARAMETER: 4.0,
        }
    }

    mod generate_health_points {
        use super::*;

        #[test]
        fn middle_parameter_gives_exactly_basic_health() {
            let config = default_config();

            // param == MIDDLE (4) => bonus points 0 => hp == BASIC_HEALTH
            let hp = Combatant::generate_health_points(&config, 4);

            assert_eq!(hp, 100, "at the middle parameter the bonus must be zero");
        }

        #[test]
        fn maximum_parameter_gives_maximum_health() {
            let config = default_config();

            // bonus = 5 * (9 - 4) = 25%; hp = 100 + 100*25/100 = 125
            let hp = Combatant::generate_health_points(&config, 9);

            assert_eq!(hp, 125, "param 9 is +5 points x 5% = +25% over basic 100");
        }

        #[test]
        fn minimum_parameter_gives_reduced_health() {
            let config = default_config();

            // bonus = 5 * (0 - 4) = -20%; hp = 100 - 20 = 80
            let hp = Combatant::generate_health_points(&config, 0);

            assert_eq!(hp, 80, "param 0 is -4 points x 5% = -20% of basic 100");
        }
    }
}
```

Run: `cargo test generate_health_points`

### 3.3 Line-by-line explanation

- `fn default_config() -> Config` — a plain (non-test) helper inside `mod tests`. It is part of the
  `#[cfg(test)]` module, so it **never ships** in the binary. This is the standard "shared arrange"
  pattern: share the *setup*, never share the *assertions*.
- All three tests call the **same function** — they test the same formula at three points of its domain:
  center, top, bottom. Each has exactly **one assertion** and one failure message stating the arithmetic.
- Note the function is `pub(crate)` — again visible to in-crate tests only.
- `generate_health_points` returns `usize` after `trunc()`; the exact integer is what you assert.
  (If it returned `f32` you would need an epsilon comparison — another topic; with `usize` it's exact.)
- **The hand-computation is the test's soul.** If the formula in the code has a bug
  (say `/ 100.0` was deleted), `maximum_parameter_gives_maximum_health` fails with
  `assertion failed: hp == 125 (left: 225, right: 125)` — immediately pointing at the missing division.

### 3.4 YOUR TURN — the reusable pattern moment

Look at [`BattleJournal::calculate_damage()`](GrandAutoBattler_GameEngine/src/battle_journal_manager.rs:37) in
`battle_journal_manager.rs`. It is the **same formula shape** as `generate_health_points`:
bonus points around a middle value, percentage applied, `trunc()` at the end, floored at 0.

Apply everything from Section 3 to it:

- Add a `#[cfg(test)] mod tests` to `battle_journal_manager.rs` with the same `default_config()` helper
  (and note `calculate_damage` takes two `&Combatant`s — you'll need a tiny helper that builds a combatant
  with chosen `attack`/`defense`; `Combatant`'s fields are all `pub`).
- Write at least one test: **neutral attacker (attack = 4, the middle) vs neutral defender (defense = 2, the middle) → damage == 10** (exactly `BASIC_ATTACK`).
- Bonus: a test where a weak attack (2) hits a very defended unit (defense 8) and equals **6**:
  incoming = 10 + 10·5·(2−4)/100 = 9; defense bonus = 5·(8−2) = 30% → 9 − 2.7 = 6.3 → `trunc` → 6.

<details>
<summary>Solution</summary>

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::combatant_manager::Combatant;
    use crate::game_data_retriever::Config;
    use std::collections::BTreeMap;

    fn default_config() -> Config {
        Config {
            PARAMETER_MINIMUM: 0,
            PARAMETER_MAXIMUM: 9,
            BASIC_HEALTH: 100.0,
            BASIC_ATTACK: 10.0,
            ATTACK_PERCENTAGE_PER_POINT: 5.0,
            DEFENSE_PERCENTAGE_PER_POINT: 5.0,
            HEALTH_PERCENTAGE_PER_POINT: 5.0,
            ATTACK_MIDDLE_VALUE_PARAMETER: 4.0,
            DEFENSE_MIDDLE_VALUE_PARAMETER: 2.0,
            HEALTH_MIDDLE_VALUE_PARAMETER: 4.0,
        }
    }

    /// Test-only constructor: everything except `attack`/`defense` is irrelevant to calculate_damage,
    /// so we fill it with harmless constants.
    fn make_combatant(attack: u8, defense: u8) -> Combatant {
        let mut parameters = BTreeMap::new();
        parameters.insert("health".to_string(), 5);
        parameters.insert("attack".to_string(), attack);
        parameters.insert("range".to_string(), 3);
        parameters.insert("defense".to_string(), defense);

        Combatant {
            id: "tester".to_string(),
            faction_name: "Testers".to_string(),
            unit_type_name: "tester".to_string(),
            unit_type_pretty_name: None,
            parameters,
            current_health_points: 100,
            health_points: 100,
            first_name: Some("Test".to_string()),
            last_name: Some("Case".to_string()),
        }
    }

    mod calculate_damage {
        use super::*;

        #[test]
        fn neutral_vs_neutral_deals_exactly_basic_attack() {
            let config = default_config();
            let attacker = make_combatant(4, 0); // attack at the middle => no bonus
            let victim = make_combatant(0, 2);   // defense at the middle => no reduction

            let dmg = BattleJournal::calculate_damage(&config, &attacker, &victim);

            assert_eq!(dmg, 10, "at the middle values damage must equal BASIC_ATTACK");
        }

        #[test]
        fn weak_attack_heavily_defended_is_truncated() {
            let config = default_config();
            let attacker = make_combatant(2, 0); // incoming = 10 - 1 = 9
            let victim = make_combatant(0, 8);   // defense bonus = 5*(8-2) = 30%

            let dmg = BattleJournal::calculate_damage(&config, &attacker, &victim);

            assert_eq!(dmg, 6, "9 - 30% = 6.3, truncated to 6");
        }
    }
}
```

</details>

---

## Section 4 — Testing Errors: `thiserror`, Display, `From`, and Pattern Matching

**Target:** [`GameEngineError`](GrandAutoBattler_GameEngine/src/error_types.rs:7) in `error_types.rs`
plus the error path of `parse_config_from_str` (a revisit, this time deeper).

### 4.1 Theory: what "testing an error" means

Your errors are an enum from **`thiserror`**: each variant has a `#[error("...")]` format string that
implements `Display`. Because `toml::de::Error` and friends are **not `PartialEq`**, you cannot
`assert_eq!` on the error *values*. Instead you test errors in three layers:

1. **Which variant** occurred → `matches!(err, GameEngineError::TomlParse(_))` (or `assert_matches!` on Rust ≥ 1.96).
2. **What it says** → `err.to_string()` (the `Display` text from `#[error(...)]`).
3. **Conversions** → the `#[from]` attributes: `io::Error` and `toml::de::Error` must convert into
   `GameEngineError` via `Into`/`From` so the `?` operator works in your code.

All three layers are plain, dependency-free tests.

### 4.2 The tests

Append to `GrandAutoBattler_GameEngine/src/error_types.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    mod error_display {
        use super::*;

        #[test]
        fn config_not_found_shows_reason() {
            let err = GameEngineError::ConfigNotFound("missing file".to_string());

            assert_eq!(err.to_string(), "Config not found: missing file");
        }
    }

    mod error_from_io {
        use super::*;

        #[test]
        fn io_error_converts_into_game_engine_error() {
            let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file missing");

            // this is exactly what `?` does inside your functions
            let game_err: GameEngineError = io_err.into();

            assert!(
                matches!(game_err, GameEngineError::Io(_)),
                "an io::Error must become GameEngineError::Io"
            );
        }
    }

    mod config_parse_errors {
        use super::*;
        use crate::toml_parser::parse_config_from_str;

        #[test]
        fn garbage_toml_yields_toml_parse_variant() {
            let err = parse_config_from_str("this is [not valid toml")
                .expect_err("garbage must not parse");

            assert!(
                matches!(err, GameEngineError::TomlParse(_)),
                "expected TomlParse, got: {err:?}"
            );
        }

        #[test]
        fn config_with_missing_fields_fails_too() {
            // Valid TOML, but not all Config fields are present -> still Err
            let err = parse_config_from_str("PARAMETER_MAXIMUM = 9")
                .expect_err("partial config must fail");

            assert!(
                matches!(err, GameEngineError::TomlParse(_)),
                "partial config should fail with TomlParse, got: {err:?}"
            );
        }
    }
}
```

Run: `cargo test error_` (hits `error_display` and `error_from_io`), then `cargo test config_parse_errors`.

### 4.3 Line-by-line explanation

- `err.to_string()` — `thiserror` generates `Display` from `#[error("Config not found: {0}")]`.
  Asserting the literal string means: if anyone edits the `#[error(...)]` text in
  [`error_types.rs`](GrandAutoBattler_GameEngine/src/error_types.rs:7), the test fails and forces a decision
  (update the test intentionally, or revert the message). Error strings are **API** — your API server
  forwards `e.to_string()` straight into HTTP responses
  (see [`routes/mod.rs`](GrandAutoBattler_API_Runner/src/routes/mod.rs)).
- `let game_err: GameEngineError = io_err.into();` — the `#[from] std::io::Error` attribute in your enum
  generated `impl From<std::io::Error> for GameEngineError`. This test proves that `?` propagation from
  `fs::read_to_string(...)?` (used in [`toml_parser.rs`](GrandAutoBattler_GameEngine/src/toml_parser.rs))
  lands in the `Io` variant and not somewhere else.
- `config_with_missing_fields_fails_too` — a subtle but valuable case: **well-formed TOML is not enough**;
  every field of `Config` must be present because there are no `#[serde(default)]`s. If someone later
  adds `#[serde(default)]` to a field, this test fails and you must consciously decide whether a
  partial `GameSettings.toml` should be acceptable.

### 4.4 YOUR TURN

Add two more tests in `mod error_display` for the two remaining variants your engine throws at the
game logic:

- `GameEngineError::FactionNotFound("Demons".to_string())` → its display string.
- `GameEngineError::UnitTypeNotFound(3usize)` → its display string (note the `{0}` inserts the index).

Look at the `#[error(...)]` lines in [`error_types.rs`](GrandAutoBattler_GameEngine/src/error_types.rs:7)
to see the exact formats before writing the expected strings.

<details>
<summary>Solution</summary>

```rust
#[test]
fn faction_not_found_shows_faction_name() {
    let err = GameEngineError::FactionNotFound("Demons".to_string());

    assert_eq!(err.to_string(), "Faction not found: Demons");
}

#[test]
fn unit_type_not_found_shows_the_index() {
    let err = GameEngineError::UnitTypeNotFound(3);

    assert_eq!(err.to_string(), "Unit type not found at index: 3");
}
```

</details>

---

## Section 5 — Testing Randomized Code (without removing the randomness)

**Targets:** [`IdRegistry::generate_id()`](GrandAutoBattler_GameEngine/src/game_data_retriever.rs:118) and
[`Combatant::apply_random_variance_to_parameters()`](GrandAutoBattler_GameEngine/src/combatant_manager.rs:67).

### 5.1 Theory: determinism is not required — **invariants** are

Your game is full of `rand` calls. You cannot assert *exact* outputs. So you assert **properties**
(**invariants**) that must hold *no matter what the dice roll*:

| Unit | Invariant |
|---|---|
| `IdRegistry::generate_id` | ids are **unique** across calls; each id is **8 hex characters** |
| `apply_random_variance_to_parameters` | values stay **within [min, max]**; a single-param map **never changes**; a map sitting **at max never changes** |

Techniques used here:

- **Loop + `BTreeSet`** to detect duplicates over many draws.
- **Crafting degenerate inputs** (1 element; all elements at the boundary) where the random choices
  *cannot* change the outcome — turning a random function into a deterministic one *from the test's side*.
  This is often better than mocking the RNG: you test the **real** random code with inputs that pin it down.

### 5.2 The tests

In `game_data_retriever.rs`, inside your existing `mod tests`, add:

```rust
    mod id_registry_generate_id {
        use super::*;

        #[test]
        fn ids_are_unique_over_many_draws() {
            let mut registry = IdRegistry::new();

            let ids: Vec<String> = (0..1000).map(|_| registry.generate_id()).collect();

            let unique_count: std::collections::BTreeSet<_> = ids.iter().collect();
            assert_eq!(
                unique_count.len(),
                ids.len(),
                "two generated ids collided in 1000 draws"
            );
        }

        #[test]
        fn id_is_eight_characters_long() {
            let mut registry = IdRegistry::new();

            let id = registry.generate_id();

            assert_eq!(id.len(), 8, "ids are formatted as {:08x}", );
        }

        #[test]
        fn id_contains_only_hex_digits() {
            let mut registry = IdRegistry::new();

            let id = registry.generate_id();

            assert!(
                id.chars().all(|c| c.is_ascii_hexdigit()),
                "id must be hex, got: {id}"
            );
        }
    }
```

And in `combatant_manager.rs`, inside your existing `mod tests`, add:

```rust
    mod apply_random_variance {
        use super::*;
        use std::collections::BTreeMap;

        #[test]
        fn single_parameter_map_is_unchanged() {
            let mut params: BTreeMap<String, u8> = BTreeMap::new();
            params.insert("health".to_string(), 7);

            // With 1 parameter: 0 bonus indices, and the decrement only triggers below the minimum
            Combatant::apply_random_variance_to_parameters(&mut params, &9, &0);

            assert_eq!(params["health"], 7, "a single parameter cannot be varied");
        }

        #[test]
        fn map_at_maximum_is_unchanged() {
            let mut params: BTreeMap<String, u8> = BTreeMap::new();
            params.insert("health".to_string(), 9);
            params.insert("attack".to_string(), 9);
            params.insert("defense".to_string(), 9);

            // Every increment is blocked by `if *val < *maximum`, decrement blocked by min=0
            Combatant::apply_random_variance_to_parameters(&mut params, &9, &0);

            assert_eq!(
                params.values().sum::<u8>(),
                27,
                "at max, no bonus points can be applied"
            );
        }
    }
```

Run: `cargo test id_registry` and `cargo test apply_random_variance` — run them **10 times**;
random code must fail 10 out of 10 to be believed broken, and pass 10 out of 10 to be believed fixed.

### 5.3 Line-by-line explanation

- `(0..1000).map(|_| registry.generate_id())` — 1000 draws. The collision probability of an 8-hex
  (32-bit) id over 1000 draws is already non-trivial, so the test is *sensitive* to a broken
  uniqueness loop (e.g., if someone removes the `insert`-check and reuses ids).
- `let unique_count: BTreeSet<_> = ids.iter().collect();` — a set keeps one copy per value;
  if `unique_count.len() < ids.len()`, a duplicate existed.
- `id_is_eight_characters_long` + `id_contains_only_hex_digits` — **two** tests instead of one
  `assert!(id.len() == 8 && id is hex)`: separate *behaviors* (format length vs. alphabet),
  per the one-behavior rule from Section 0.6.
- `single_parameter_map_is_unchanged` — read
  [`apply_random_variance_to_parameters`](GrandAutoBattler_GameEngine/src/combatant_manager.rs:67):
  `number_of_applicable_parameters = len - 1 = 0` → the bonus loop never runs; the one "unaffected"
  index is chosen, but the decrement requires `*val < *minimum` → `7 < 0` is false. **The randomness
  is real, but every random path is a no-op for this input.** That's the trick.
- `map_at_maximum_is_unchanged` — same idea at the upper boundary: the `if *val < *maximum` guard
  blocks every increment, min=0 blocks the decrement. The sum of 9+9+9 must stay 27.

### 5.4 YOUR TURN

Add one more invariant test to `mod apply_random_variance`:

A map with **two parameters both at 5**, min = 5, max = 9. Reason about the code:
`number_of_applicable = 1` → exactly **one** entry gets `+1` (5→6, allowed); the unaffected entry (5)
is not below min (5), so it never decreases. Therefore the sum must be **exactly 11** on every run.
Assert that, and add a second assertion that every value still lies within `[5, 9]`.

<details>
<summary>Solution</summary>

```rust
#[test]
fn two_equal_parameters_gain_exactly_one_bonus_point() {
    let mut params: BTreeMap<String, u8> = BTreeMap::new();
    params.insert("health".to_string(), 5);
    params.insert("attack".to_string(), 5);

    Combatant::apply_random_variance_to_parameters(&mut params, &9, &5);

    assert_eq!(params.values().sum::<u8>(), 11, "one bonus point is distributed");
    assert!(
        params.values().all(|v| (5..=9).contains(v)),
        "all values must stay within [min, max]: {params:?}"
    );
}
```

Note the `(5..=9).contains(v)` range-check — a single line that expresses the whole boundary invariant.

</details>

---

## Section 6 — Integration Tests: Thinking Like a Consumer

**Target:** a new `tests/` folder in `GrandAutoBattler_GameEngine`.

### 6.1 Theory: the public-API-only rule

Everything in Section 1–5 lived **inside** `src/`, so tests could call private items. An **integration
test** is a *separate tiny crate* that merely **depends on your library**. Cargo compiles
`GrandAutoBattler_GameEngine` once, and your `tests/*.rs` files link against it — with *exactly* the
same visibility an external user (your **API server**, the Angular app's backend) would get.

Consequences:

- `mod toml_parser;` and `mod game_data_retriever;` in
  [`lib.rs`](GrandAutoBattler_GameEngine/src/lib.rs) are **private** — integration tests **cannot**
  name them, cannot call `parse_config_from_str`, cannot name `Config`, `Faction`, or `IdRegistry`.
  (Try it — the compiler will politely refuse.)
- They **can** use: `battle_journal_manager`, `combatant_manager`, `error_types`,
  `game_instance_manager`, `game_result_manager` — everything `pub` in `lib.rs`.
- Each file `tests/*.rs` becomes **one integration-test crate** (so two files cannot share helpers
  without a `tests/common/` module — not needed here).

Integration tests are the right home for:

- **Serde round-trips of the public wire types** — the exact thing your API server does with
  `serde_json`.
- **Constructor defaults** of public structs.
- Anything that only makes sense when *several modules cooperate*.

### 6.2 The tests

Create the file `GrandAutoBattler_GameEngine/tests/public_api.rs`:

```rust
use GrandAutoBattler_GameEngine::battle_journal_manager::{BattleJournal, Command};
use GrandAutoBattler_GameEngine::game_result_manager::GameResult;
use std::collections::BTreeMap;

#[test]
fn fresh_journal_starts_with_no_events() {
    let journal = BattleJournal::new();

    assert!(journal.events.is_empty(), "a fresh journal must have no events");
}

#[test]
fn game_result_accepts_empty_sides() {
    let sides: BTreeMap<usize, Vec<GrandAutoBattler_GameEngine::combatant_manager::Combatant>> =
        BTreeMap::new();
    let journal = BattleJournal::new();

    let result = GameResult::new(sides, journal).expect("empty sides should be accepted");

    assert!(result.generated_combatants.is_empty());
}

#[test]
fn attack_command_survives_json_round_trip() {
    // Your API serializes these types to JSON for the browser.
    // If this breaks, the whole front end breaks — so we pin it here.
    let cmd = Command::Attack {
        src: "a1".to_string(),
        dst: "b2".to_string(),
        dmg: 42,
    };

    let json = serde_json::to_string(&cmd).expect("serializing Command must work");
    let back: Command = serde_json::from_str(&json).expect("deserializing Command must work");

    assert!(
        matches!(back, Command::Attack { dmg: 42, .. }),
        "round trip must preserve damage; json was: {json}"
    );
}
```

Run: `cd GrandAutoBattler_GameEngine; cargo test --test public_api`

### 6.3 Line-by-line explanation

- `use GrandAutoBattler_GameEngine::...` — note the **full crate name**. Inside `src/` you'd write
  `crate::...`; in `tests/` you're *outside* the crate, so you address it by package name,
  exactly like [`routes/mod.rs`](GrandAutoBattler_API_Runner/src/routes/mod.rs) does in the API server.
- `serde_json` is already in `Cargo.toml`'s `[dependencies]` — and **integration tests may use the
  crate's normal dependencies too**, so no `Cargo.toml` changes are needed.
- `Command::Attack { dmg: 42, .. }` — struct-variant pattern with `..` ("ignore the rest"),
  so we only assert the field that matters for this test.
- Why a round-trip test at all? `Command`, `Effect`, `GameEvent`, `Combatant`, `BattleJournal`,
  `GameResult` are all `Serialize + Deserialize` **because your API ships them to Angular as JSON**
  (see [`types.rs`](GrandAutoBattler_API_Runner/src/types.rs)). If a field ever gets a typo'd
  `#[serde(rename)]` or a type change that breaks `Option` deserialization, this test fails in the
  engine crate — before any HTTP request ever exists.

### 6.4 The honest caveat: `env::current_exe()` is a testability seam

`GameInstance::new()` → `load_config()`/`load_factions()` resolve paths from
`std::env::current_exe()` (see [`toml_parser.rs`](GrandAutoBattler_GameEngine/src/toml_parser.rs:61)) —
the **executable's** directory. When *your* runner executes, `build.rs` copies `assets/` next to the
binary, so it works. But a test executable in `target/debug/deps/` has **no `assets/` next to it**,
so `GameInstance::new()` in an integration test would fail with `ConfigNotFound` — not because the
code is wrong, but because of *where the binary lives*.

Today: integration tests deliberately avoid anything that reads from disk (as above).
**If you ever want to** test `GameInstance::new()` directly, the standard fix is to give the engine
its own `build.rs` that copies `assets/` into `OUT_DIR` (or next to the test binary) during `cargo test`,
or to make the loader accept an explicit root path. That's a production refactor — out of scope for
this guide, but now you know exactly *why* and *what* it would mean. (This is the classic reason
"testable from disk" functions get a `path: impl AsRef<Path>` parameter in larger Rust projects.)

### 6.5 YOUR TURN

Write one more integration test in `tests/public_api.rs`: round-trip a **`BattleJournal`** that
contains **two `GameEvent`s** (two `Command::Attack` entries) through `serde_json`, then assert
`back.events.len() == 2`. (You may need to look at
[`GameEvent`](GrandAutoBattler_GameEngine/src/battle_journal_manager.rs:240) and
[`Effect`](GrandAutoBattler_GameEngine/src/battle_journal_manager.rs:14) to construct them —
`Effect::Damage` has 5 fields.)

<details>
<summary>Solution</summary>

```rust
use GrandAutoBattler_GameEngine::battle_journal_manager::{BattleJournal, Command, Effect, GameEvent};

#[test]
fn journal_with_two_events_survives_json_round_trip() {
    let mk_event = |turn: u32, dmg: u32| GameEvent {
        round: 1,
        turn,
        cmd: Command::Attack {
            src: "a".to_string(),
            dst: "b".to_string(),
            dmg,
        },
        effects: vec![Effect::Damage {
            src: "a".to_string(),
            dst: "b".to_string(),
            dmg,
            dst_hp_before: 100.0,
            dst_hp_after: 100.0 - dmg as f32,
        }],
    };

    let journal = BattleJournal {
        winner_side: 1,
        events: vec![mk_event(1, 12), mk_event(2, 5)],
    };

    let json = serde_json::to_string(&journal).expect("journal must serialize");
    let back: BattleJournal = serde_json::from_str(&json).expect("journal must deserialize");

    assert_eq!(back.events.len(), 2, "both events must survive the round trip");
}
```

</details>

---

## Section 7 — Testing the API Runner: Serde Types and Axum Handlers

**Target:** `GrandAutoBattler_API_Runner` — [`types.rs`](GrandAutoBattler_API_Runner/src/types.rs) and
[`routes/mod.rs`](GrandAutoBattler_API_Runner/src/routes/mod.rs).

### 7.1 Theory: testing a binary crate and an HTTP handler

`GrandAutoBattler_API_Runner` has **no `src/lib.rs`** — it's a binary. The good news: `#[cfg(test)]`
modules **inside `src/` files of a binary** are tested perfectly normally by `cargo test` (Cargo compiles
the binary in test mode). The rule "integration tests need a lib" applies only to the `tests/` folder;
in-source unit tests work in binaries.

Two kinds of targets here:

1. **`types.rs`** — pure data types crossing the wire. The most valuable test: **does the JSON your
   Angular app sends actually deserialize into the struct?** Your request type contains
   `Vec<Option<usize>>` — a `null` in JSON must become `None`. That's a one-line bug away.
2. **Axum handlers** — each handler is just an `async fn` returning
   `(StatusCode, Json<T>)`. Axum's router is an ordinary `tower::Service`, so with **one extra
   dev-dependency** (`tower` for the `ServiceExt::oneshot` method) you can call a route **without
   starting a server, without a port, without any network** — you hand it a synthetic `Request`
   and get a synthetic `Response` back.

Dev-dependency to add to `GrandAutoBattler_API_Runner/Cargo.toml` (new section at the bottom):

```toml
[dev-dependencies]
tower = { version = "0.5", features = ["util"] }
```

(`dev-dependencies` = available only while compiling tests — your production binary stays lean,
and a typo here breaks only `cargo test`, never `cargo build`.)

### 7.2 Test A — the request DTO (types.rs)

Append to `GrandAutoBattler_API_Runner/src/types.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    mod get_game_result_request {
        use super::*;

        #[test]
        fn json_with_null_slots_deserializes_into_options() {
            // This is literally what your Angular front end can POST:
            // null = "no unit in this slot", 0/1/2/3 = unit type index
            let json = r#"{"our_faction_index":0,"opponent_faction_index":1,"our_unittype_indices":[0,null,2,null]}"#;

            let request: GetGameResultByIdAndSelectionRequest =
                serde_json::from_str(json).expect("front-end-shaped JSON must deserialize");

            assert_eq!(request.our_faction_index, 0);
            assert_eq!(request.our_unittype_indices[1], None, "null slot must be None");
            assert_eq!(request.our_unittype_indices[2], Some(2), "numbered slot must be Some(index)");
        }
    }
}
```

Run: `cd ..\GrandAutoBattler_API_Runner; cargo test json_with_null_slots`

### 7.3 Test B — handler without a server (routes/mod.rs)

Append to `GrandAutoBattler_API_Runner/src/routes/mod.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::{to_bytes, Body},
        http::{Request, StatusCode},
        routing::get,
    };
    use tower::ServiceExt; // gives Router a .oneshot() method

    /// Build the app the same way create_routes() does, but we own the state here.
    fn app() -> Router {
        let state = ApiState {
            game_manager: Arc::new(Mutex::new(GameInstancesManager::new())),
        };
        Router::new()
            .route("/health", get(health_check))
            .route("/games/:game_id/factions", get(available_factions_retrieve))
            .with_state(state)
    }

    mod health_check {
        use super::*;

        #[tokio::test]
        async fn returns_ok_with_status_field() {
            let request = Request::builder()
                .uri("/health")
                .body(Body::empty())
                .expect("building a request cannot fail");

            let response = app().oneshot(request).await.expect("handler must not error");

            assert_eq!(response.status(), StatusCode::OK);

            let bytes = to_bytes(response.into_body(), usize::MAX)
                .await
                .expect("reading the response body must work");
            let body: serde_json::Value = serde_json::from_slice(&bytes).expect("body must be JSON");

            assert_eq!(body["status"], "ok", "health body must say ok");
        }
    }

    mod available_factions {
        use super::*;

        #[tokio::test]
        async fn unknown_game_id_returns_404() {
            // GameInstancesManager::new() is empty -> any id is unknown -> GameInstanceNotFound -> 404
            let request = Request::builder()
                .uri("/games/does-not-exist/factions")
                .body(Body::empty())
                .expect("building a request cannot fail");

            let response = app().oneshot(request).await.expect("handler must not error");

            assert_eq!(response.status(), StatusCode::NOT_FOUND);
        }
    }
}
```

Run: `cargo test health_check` and `cargo test available_factions`

### 7.4 Line-by-line explanation

- `Request::builder().uri("/health").body(Body::empty())` — a fully **synthetic HTTP request**.
  `Body::empty()` = zero-byte body (fine for a `GET`).
- `app().oneshot(request)` — `oneshot` consumes the router and runs **one** request through it,
  inside a real tokio runtime (provided by `#[tokio::test]`). No port is bound, no thread is blocked.
  You can run every handler test in parallel — they share no sockets.
- `to_bytes(response.into_body(), usize::MAX)` — drains the (small) response body into a `Vec<u8>`
  so you can inspect the JSON. The `usize::MAX` bound just satisfies the API ("max bytes").
- `body["status"]` — indexing a `serde_json::Value` returns a `Value`; comparing `Value == &str`
  works via `PartialEq<str>`... precisely, `assert_eq!(body["status"], "ok")` compares against a
  `&str` because `serde_json::Value` implements `PartialEq<str>`. If the field is missing you get
  `Null != "ok"` — a clean failure.
- `unknown_game_id_returns_404` — the handler's **error branch** is exercised for real:
  `get_available_factions` → `GameInstanceNotFound` → `StatusCode::NOT_FOUND`. Note we never had
  to fake an error: an empty manager *guarantees* one. That's the cheapest, most honest test setup.
- Why not test `new_game_id_retrieve` (the 200 branch)? It calls
  `create_new_game_instance()` → `load_config()` → reads `assets/` next to the *test* binary,
  which doesn't exist (same seam as Section 6.4). It *would* work as a test of the **500 branch**,
  but that couples your test suite to "assets are absent" — a fragile assumption. Leave it; the two
  tests above pin the *structure* of your handlers, and the engine logic itself is already tested
  in Sections 1–5 where it's deterministic.

### 7.5 YOUR TURN

Add a test in `mod health_check` that hits the **`"/"` route** (your router registers `health_check`
on both `"/"` and `"/health"` — see [`create_routes`](GrandAutoBattler_API_Runner/src/routes/mod.rs:62)):
build a request for `"/"`, and assert the response status is `OK`. (You must register `"/"` in your
`app()` helper too — copy that line from `create_routes`.)

<details>
<summary>Solution</summary>

```rust
#[tokio::test]
async fn root_path_also_returns_ok() {
    let request = Request::builder()
        .uri("/")
        .body(Body::empty())
        .expect("building a request cannot fail");

    let response = app().oneshot(request).await.expect("handler must not error");

    assert_eq!(response.status(), StatusCode::OK);
}
```

plus, in the `app()` helper:

```rust
Router::new()
    .route("/", get(health_check))
    .route("/health", get(health_check))
    ...
```

</details>

---

## Section 8 — Wrap-Up: Doc Tests, Special Attributes, and the Full Workflow

### 8.1 Doc tests — examples in `///` comments that `cargo test` runs

A `///` doc comment can embed an example that the **rustdoc test runner** compiles and executes
(`cargo test --doc`). Two birds: living documentation *and* a regression check on the public API.

Add this to [`BattleJournal::new`](GrandAutoBattler_GameEngine/src/battle_journal_manager.rs:33):

```rust
/// Creates an empty battle journal with no events.
///
/// ```
/// use GrandAutoBattler_GameEngine::battle_journal_manager::BattleJournal;
///
/// let journal = BattleJournal::new();
/// assert!(journal.events.is_empty());
/// ```
pub fn new() -> Self {
```

Notes:

- Doc-test code uses the **full crate path** (`use GrandAutoBattler_GameEngine::...`) — like an
  integration test, it's an external consumer.
- Lines starting with `#` are hidden in the rendered docs but still executed — use them for
  setup noise.
- Doc tests run with `cargo test --doc` (and with plain `cargo test` too, by default).
- When in doubt, keep doc examples tiny — they're documentation first, tests second.

### 8.2 `#[should_panic]` and `#[ignore]`

```rust
#[test]
#[should_panic(expected = "known failure reason")]
fn this_function_panics_on_bogus_input() {
    // ...
}
```

- Use `#[should_panic]` **only** when panicking is the *designed* behavior of a function
  (e.g., `Vec::remove` with a bad index). Your engine instead returns `Result` — good:
  keep using `assert_matches!` on the `Err`, not `#[should_panic]`.
- `#[ignore = "needs a running assets folder (see guide Section 6.4)"]` marks a test to be skipped
  by `cargo test` (it runs with `cargo test -- --include-ignored`). Use it for tests that are
  correct but need an environment — better than deleting them.

### 8.3 The rules recap (the checklist you now have the muscle memory for)

1. Test **near** the code: unit tests in the same file; `tests/` only for public-API/consumer views.
2. **One behavior** per test; share *setup helpers*, never assertions.
3. Name tests as sentences, grouped in a module named after the unit under test.
4. Assert with the **narrowest tool**: `assert_eq!` > `assert!` > `matches!` > (Rust ≥ 1.96) `assert_matches!`.
   Always write a failure message that states the expected state.
5. Randomized code → assert **invariants**, or craft inputs that pin the randomness down.
6. Error code → test the **variant**, the **Display text**, and the **`From` conversions**.
7. `unwrap()`/`expect()` freely in tests — with messages.

### 8.4 The full PowerShell workflow (run from each crate folder)

```powershell
cd d:\REPOS\Rust\FinalRustProject\GrandAutoBattler\GrandAutoBattler_GameEngine
cargo test                                # everything (unit + integration + doc)
cargo test --lib                          # just the unit tests in src/
cargo test --test public_api              # just the integration file
cargo test --doc                          # just doc tests
cargo test health                         # filter by name substring
cargo test -- --nocapture                 # see any println! output from tests
cargo clippy --all-targets -- -D warnings # lints your TESTS too (--all-targets includes cfg(test) code)
```

Then the same `cargo test` / `cargo clippy --all-targets -- -D warnings` in
`GrandAutoBattler_API_Runner`.

**Why `--all-targets` for clippy matters:** without it, clippy checks `src/` in normal mode and
**never sees your test code** — a typo or a bad assertion helper in a test module would go unlinted.

---

*End of the guide. All drills' solutions are inside the `<details>` blocks — write yours first, then compare.*
