/*
 * Copyright (C) 2026 Huawei Technologies Co., Ltd.
 * SPDX-License-Identifier: 0BSD
 */
#ifndef DICE_SOCKET_H
#define DICE_SOCKET_H

#include <sys/socket.h>

#ifndef DICE_ROLLED
#define EVENT_ACCEPT      104
#endif
#ifndef DICE_ROLLED
#define EVENT_ACCEPT4     105
#endif
#ifndef DICE_ROLLED
#define EVENT_BIND        106
#endif
#ifndef DICE_ROLLED
#define EVENT_CONNECT     107
#endif
#ifndef DICE_ROLLED
#define EVENT_GETPEERNAME 108
#endif
#ifndef DICE_ROLLED
#define EVENT_GETSOCKNAME 109
#endif
#ifndef DICE_ROLLED
#define EVENT_GETSOCKOPT  110
#endif
#ifndef DICE_ROLLED
#define EVENT_SETSOCKOPT  111
#endif
#ifndef DICE_ROLLED
#define EVENT_LISTEN      112
#endif
#ifndef DICE_ROLLED
#define EVENT_RECV        113
#endif
#ifndef DICE_ROLLED
#define EVENT_RECVFROM    114
#endif
#ifndef DICE_ROLLED
#define EVENT_RECVMSG     115
#endif
#ifndef DICE_ROLLED
#define EVENT_SEND        116
#endif
#ifndef DICE_ROLLED
#define EVENT_SENDTO      117
#endif
#ifndef DICE_ROLLED
#define EVENT_SENDMSG     118
#endif
#ifndef DICE_ROLLED
#define EVENT_SHUTDOWN    119
#endif
#ifndef DICE_ROLLED
#define EVENT_SOCKET      120
#endif
#ifndef DICE_ROLLED
#define EVENT_SOCKETPAIR  121
#endif

struct accept_event {
    const void *pc;
    int sockfd;
    struct sockaddr* addr;
    socklen_t* addrlen;
    int ret;
    int errno_;
    int(*func)(int, struct sockaddr*, socklen_t*);
};

struct accept4_event {
    const void *pc;
    int sockfd;
    struct sockaddr* addr;
    socklen_t* addrlen;
    int flags;
    int ret;
    int errno_;
    int(*func)(int, struct sockaddr*, socklen_t*, int);
};

struct bind_event {
    const void *pc;
    int sockfd;
    const struct sockaddr *addr;
    socklen_t addrlen;
    int ret;
    int errno_;
    int(*func)(int, const struct sockaddr*, socklen_t);
};

struct connect_event {
    const void *pc;
    int sockfd;
    struct sockaddr *addr;
    socklen_t addrlen;
    int ret;
    int errno_;
    int(*func)(int, const struct sockaddr*, socklen_t);
};

struct getpeername_event {
    const void *pc;
    int sockfd;
    struct sockaddr *addr;
    socklen_t *addrlen;
    int ret;
    int errno_;
    int(*func)(int, struct sockaddr*, socklen_t*);
};

struct getsockname_event {
    const void *pc;
    int sockfd;
    struct sockaddr *addr;
    socklen_t *addrlen;
    int ret;
    int errno_;
    int(*func)(int, struct sockaddr*, socklen_t*);
};

struct getsockopt_event {
    const void *pc;
    int sockfd;
    int level;
    int optname;
    void *optval;
    socklen_t *optlen;
    int ret;
    int errno_;
    int (*func)(int, int, int, void *, socklen_t *);
};

struct setsockopt_event {
    const void *pc;
    int sockfd;
    int level;
    int optname;
    const void *optval;
    socklen_t optlen;
    int ret;
    int errno_;
    int(*func)(int, int, int, const void*, socklen_t);
};

struct listen_event {
    const void *pc;
    int sockfd;
    int backlog;
    int ret;
    int errno_;
    int(*func)(int, int);
};

struct recv_event {
    const void *pc;
    int sockfd;
    void *buf;
    size_t len;
    int flags;
    ssize_t ret;
    int errno_;
    ssize_t(*func)(int, void*, size_t, int);
};

struct recvfrom_event {
    const void *pc;
    int sockfd;
    void *buf;
    size_t len;
    int flags;
    struct sockaddr *src_addr;
    socklen_t *addrlen;
    ssize_t ret;
    int errno_;
    ssize_t(*func)(int, void*, size_t, int, struct sockaddr*, socklen_t*);
};

struct recvmsg_event {
    const void *pc;
    int sockfd;
    struct msghdr *msg;
    int flags;
    ssize_t ret;
    int errno_;
    ssize_t(*func)(int, struct msghdr*, int);
};

struct send_event {
    const void *pc;
    int sockfd;
    const void *buf;
    size_t len;
    int flags;
    ssize_t ret;
    int errno_;
    ssize_t(*func)(int, const void*, size_t, int);
};

struct sendto_event {
    const void *pc;
    int sockfd;
    const void *buf;
    size_t len;
    int flags;
    const struct sockaddr *dest_addr;
    socklen_t addrlen;
    ssize_t ret;
    int errno_;
    ssize_t(*func)(int, const void*, size_t, int, const struct sockaddr*, socklen_t);
};

struct sendmsg_event {
    const void *pc;
    int sockfd;
    const struct msghdr *msg;
    int flags;
    ssize_t ret;
    int errno_;
    ssize_t(*func)(int, const struct msghdr*, int);
};

struct shutdown_event {
    const void *pc;
    int sockfd;
    int how;
    int ret;
    int errno_;
    int(*func)(int, int);
};

struct socket_event {
    const void *pc;
    int domain;
    int type;
    int protocol;
    int ret;
    int errno_;
    int(*func)(int, int, int);
};

struct socketpair_event {
    const void *pc;
    int domain;
    int type;
    int protocol;
    int *sv;
    int ret;
    int errno_;
    int(*func)(int, int, int, int*);
};

#endif /* DICE_SOCKET_H */
