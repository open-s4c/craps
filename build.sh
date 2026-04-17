#!/bin/sh
DICE_MEMPOOL_SIZE=100000000 cargo build --release --features "dice dice-pthread_create dice-self dice-poll dice-malloc dice-random"
rm -f librecorder.so && gcc -O3 -shared -o librecorder.so -Wl,--whole-archive target/release/librecorder.a -Wl,--no-whole-archive
rm -f libreplayer.so && gcc -O3 -shared -o libreplayer.so -Wl,--whole-archive target/release/libreplayer.a -Wl,--no-whole-archive
