#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/random.h>

#define NUM_BYTES 16

int main() {
    unsigned char buffer[NUM_BYTES];

    ssize_t bytes_read = getrandom(buffer, NUM_BYTES, 0);

    if (bytes_read == -1) {
        perror("getrandom failed");
        return 1;
    }

    printf("Random bytes: ");
    for (size_t i = 0; i < NUM_BYTES; i++) {
        printf("%02x ", buffer[i]);
    }
    printf("\n");

    return 0;
}