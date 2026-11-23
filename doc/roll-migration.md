# Craps migration to Dice Roll

This experimental branch embeds the generated Dice runtime from the
`dice-rs/mutable-events-roll` submodule. It does not modify `main`.

The full extension event-number list is pinned in
`dice-extension/roll/ids.dice`: **218 unique values, IDs 103 through 321**.
The module `.dice` manifests describe the interceptors' existing publication
routes. `dice-extension/roll/craps.dice` imports both built-in modules and all
external module descriptions. Dynamic Rust callbacks remain enabled.

Run `git submodule update --init --recursive`, then `./build.sh`. This script
sets `DICE_ROLL_CONFIG`, `DICE_ROLL_IMPORT_PATHS`, `DICE_EXTENSION_DIRS`,
`DICE_MODULES`, and the legacy maximum event count. The build invokes
Dice Roll through `dice-rs`'s Cargo build script.

Do not merge until the following are checked on a Linux host with Cargo:
- Dice Roll `--check` and `--query` for the manifest.
- Compilation of every external interceptor, including `syscall.S`.
- Confirm event IDs and chain IDs match the existing Rust bindings.
- Run the C interposition tests and the Rust recorder/replayer tests.
- Run a Servo record/replay pair and compare call ordering and event payloads.
- Check that Rust subscription constructors execute after Dice initialization
  and that callbacks are present for every used chain/type.

Roll rebuilds must not renumber legacy trace events. This is an opt-in
migration and is **not yet build-validated**.
