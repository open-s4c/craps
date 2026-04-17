#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <poll.h>

int main() {
    struct pollfd fds[1];
    int timeout = 1000;

    fds[0].fd = STDIN_FILENO;
    fds[0].events = POLLIN;

    int ret = poll(fds, 1, timeout);

    if (ret == -1) {
        perror("poll");
        exit(EXIT_FAILURE);
    } else if (ret == 0) {
        printf("Timeout occurred, no data available.\n");
    } else {
        if (fds[0].revents & POLLIN) {
            printf("Data is available on stdin.\n");
        }
    }

    return 0;
}
