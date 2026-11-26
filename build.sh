#!/bin/sh
set -eu

# Dice Roll is the only runtime build: C and Rust both consume the generated
# configuration. No fixed extension IDs or legacy dispatch tables.
ROOT=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
export DICE_EXTENSION_DIRS="$ROOT/dice-extension"
export DICE_ROLL_CONFIG="$ROOT/dice-extension/roll/craps.dice"
export DICE_ROLL_IMPORT_PATHS="dice=$ROOT/dice-extension/roll"
export DICE_MODULES="dirent;fcntl;ioctl;mman_ext;poll;pthread_detach;pthread_key;pthread_once;socket;stat;stdio;time;uio;unistd;syscall"
if [ "$(uname -s)" = Linux ]; then
    DICE_MODULES="$DICE_MODULES;eventfd;random;epoll"
fi
export DICE_MODULES
export DICE_MEMPOOL_SIZE="${DICE_MEMPOOL_SIZE:-100000000}"
export CRAPS_PRIORITY="${CRAPS_PRIORITY:-9000}"
export CRAPS_INIT_PRIORITY="${CRAPS_INIT_PRIORITY:-8999}"

cargo build --release --features "log-debug dice dice-pthread_create dice-self dice-malloc"
# The workspace currently defines one Craps static library (libcraps.a).
# Runtime mode selects record or replay; keep the old .so names as aliases.
for output in libcraps_recorder.so libcraps_replayer.so; do
    rm -f "$output"
    gcc -O3 -shared -o "$output" \
        -Wl,--whole-archive target/release/libcraps.a -Wl,--no-whole-archive \
        -ldl -lpthread -lm
done
