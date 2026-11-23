/*
 * Copyright (C) 2026 Huawei Technologies Co., Ltd.
 * SPDX-License-Identifier: 0BSD
 */
#ifndef DICE_EPOLL_H
#define DICE_EPOLL_H

#include <sys/epoll.h>


struct epoll_create_event {
    const void *pc;
    int size;
    int ret;
    int(*func)(int);
};

struct epoll_create1_event {
    const void *pc;
    int flags;
    int ret;
    int(*func)(int);
};

struct epoll_ctl_event {
    const void *pc;
    int epfd;
    int op;
    int fd;
    struct epoll_event *event;  // can be NULL
    int ret;
    int(*func)(int, int, int, struct epoll_event*);
};

struct epoll_wait_event {
    const void *pc;
    int epfd;
    struct epoll_event *events;
    int maxevents;
    int timeout;
    int ret;
    int(*func)(int, struct epoll_event*, int, int);
};

struct epoll_pwait_event {
    const void *pc;
    int epfd;
    struct epoll_event *events;
    int maxevents;
    int timeout;
    const sigset_t *sigmask;
    int ret;
    int(*func)(int, struct epoll_event*, int, int, const sigset_t*);
};

struct epoll_pwait2_event {
    const void *pc;
    int epfd;
    struct epoll_event *events;
    int maxevents;
    const struct timespec *timeout;
    const sigset_t *sigmask;
    int ret;
    int(*func)(int, struct epoll_event*, int, const struct timespec*, const sigset_t*);
};

#endif /* DICE_EPOLL_H */
