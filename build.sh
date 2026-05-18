#!/bin/sh
DICE_MEMPOOL_SIZE=100000000 CRAPS_PRIORITY=9000 cargo build --release --features "log-debug dice dice-pthread_create dice-self dice-poll dice-malloc dice-random"
rm -f libcraps_recorder.so && gcc -O3 -shared -o libcraps_recorder.so -Wl,--whole-archive target/release/libcraps_recorder.a -Wl,--no-whole-archive
rm -f libcraps_replayer.so && gcc -O3 -shared -o libcraps_replayer.so -Wl,--whole-archive target/release/libcraps_replayer.a -Wl,--no-whole-archive
