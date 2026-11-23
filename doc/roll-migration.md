# Dice Roll is the only Craps runtime

Craps' C extension defines **event structures**, not numeric event IDs.
`dice-extension/roll/craps.dice` imports the built-in Dice modules and all
Craps module descriptions; the individual `.dice` files declare publications
and initialization subscriptions. Dice Roll assigns the event IDs, module slots
and dispatch routes automatically. The generated C header is also the source
of truth for Rust bindgen.

`dice-extension/include/dice/events/*.h` contain **no numeric EVENT_ defines**.
The extension's CMake builds only `dice-*.o` module catalogs that
`add_dice_mods()` consumes as part of the configured `dice-roll-runtime`.
Legacy Dice build paths and `DICE_MAX_TYPES` are unsupported in Craps.

To build:

```sh
git submodule update --init --recursive
./build.sh
```

The script sets `DICE_ROLL_CONFIG`, `DICE_ROLL_IMPORT_PATHS`,
`DICE_EXTENSION_DIRS` and `DICE_MODULES` and builds the configured C/Rust
runtime. To inspect the generated event IDs and dispatch routes, run
`dice-roll --query` or `dice-roll --graph` on the manifest.

**Trace compatibility:** this changes the numeric event IDs. Record and replay
must use binaries built from the **same** Roll configuration, and pre-migration
traces are not assumed to be compatible. Never compare or hard-code event IDs;
use generated event constants and `DiceEvent::ID`.

**Before merging:** run the Roll `--check` validator, build the complete
extension (including `syscall.S`), execute the interposer tests, run the Rust
tests and validate a Servo record/replay cycle. The migration remains a draft
until these checks pass.
