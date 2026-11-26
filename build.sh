#!/bin/sh
set -eu

# Keep the new Roll runtime explicit while preserving the original build
# environment for record/replay testing. Use absolute paths for the generator.
ROOT=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
export DICE_EXTENSION_DIRS="$ROOT/dice-extension"
export DICE_ROLL_CONFIG="$ROOT/dice-extension/roll/craps.dice"
export DICE_ROLL_IMPORT_PATHS="dice=$ROOT/dice-extension/roll"
export DICE_MAX_TYPES=322
export DICE_MODULES="dirent;fcntl;ioctl;mman_ext;poll;pthread_detach;pthread_key;pthread_once;socket;stdio;time;uio;unistd;syscall"
if [ "$(uname -s)" = Linux ]; then
    DICE_MODULES="$DICE_MODULES;eventfd;random;epoll"
fi
export DICE_MODULES
export DICE_MEMPOOL_SIZE="${DICE_MEMPOOL_SIZE:-100000000}"
export CRAPS_PRIORITY="${CRAPS_PRIORITY:-9000}"
export CRAPS_INIT_PRIORITY="${CRAPS_INIT_PRIORITY:-8999}"

cargo build --release --features "log-debug dice dice-pthread_create dice-self dice-malloc"
rm -f libcraps_recorder.so
gcc -O3 -shared -o libcraps_recorder.so \
    -Wl,--whole-archive target/release/libcraps_recorder.a -Wl,--no-whole-archive
rm -f libcraps_replayer.so
gcc -O3 -shared -o libcraps_replayer.so \
    -Wl,--whole-archive target/release/libcraps_replayer.a -Wl,--no-whole-archive
