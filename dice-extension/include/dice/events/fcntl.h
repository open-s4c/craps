/*
 * Copyright (C) 2026 Huawei Technologies Co., Ltd.
 * SPDX-License-Identifier: 0BSD
 */
#ifndef DICE_FCNTL_H
#define DICE_FCNTL_H

#ifndef _GNU_SOURCE
#define _GNU_SOURCE
#endif
#ifndef _LARGEFILE64_SOURCE
#define _LARGEFILE64_SOURCE
#endif
#include <fcntl.h>
#include <inttypes.h>

#define EVENT_CREAT           129
#define EVENT_FCNTL           130
#define EVENT_OPEN            131
#define EVENT_OPENAT          132
#define EVENT_OPENAT64        280
#define EVENT_POSIX_FADVISE   133
#define EVENT_POSIX_FALLOCATE 134
#define EVENT_OPEN64          266

struct creat_event {
    const void *pc;
    const char *path;
    mode_t mode;
    int ret;
    int(*func)(const char *, mode_t);
};

struct fcntl_event {
    const void *pc;
    int fd;
    int cmd;
    uint64_t arg;
    int ret;
    int(*func)(int, int, ...);
};

struct open_event {
    const void *pc;
    const char *path;
    int flags;
    mode_t mode;
    int ret;
    int(*func)(const char *, int, ...);
};

struct openat_event {
    const void *pc;
    int dirfd;
    const char *path;
    int flags;
    mode_t mode;
    int ret;
    int(*func)(int, const char *, int, ...);
};

struct openat64_event {
    const void *pc;
    int dirfd;
    const char *path;
    int flags;
    mode_t mode;
    int ret;
    int(*func)(int, const char *, int, ...);
};

struct posix_fadvise_event {
    const void *pc;
    int fd;
    off_t offset;
    off_t size;
    int advice;
    int ret;
    int(*func)(int, off_t, off_t, int);
};

struct posix_fallocate_event {
    const void *pc;
    int fd;
    off_t offset;
    off_t size;
    int ret;
    int(*func)(int, off_t, off_t);
};

struct open64_event {
    const void *pc;
    const char *path;
    int flags;
    mode_t mode;
    int ret;
    int (*func)(const char *, int, ...);
};

/* Filesystem replay events */
#define EVENT_CREAT64                373
#define EVENT_FCNTL64                374
#define EVENT_FALLOCATE              375
#define EVENT_FALLOCATE64            376
#define EVENT_POSIX_FALLOCATE64      377
#define EVENT_POSIX_FADVISE64        378

#if defined(__linux__) && defined(__GLIBC__)
struct creat64_event {
    const void *pc;
    const char *path;
    mode_t mode;
    int ret;
    int errno_;
    int (*func)(const char *, mode_t);
};
#endif

#if defined(__linux__) && defined(__GLIBC__)
#if __GLIBC_PREREQ(2, 28)
struct fcntl64_event {
    const void *pc;
    int fd;
    int cmd;
    uint64_t arg;
    int ret;
    int errno_;
    int (*func)(int, int, ...);
};
#endif
#endif

#if defined(__linux__) && defined(__GLIBC__)
struct fallocate_event {
    const void *pc;
    int fd;
    int mode;
    off_t offset;
    off_t len;
    int ret;
    int errno_;
    int (*func)(int, int, off_t, off_t);
};
#endif

#if defined(__linux__) && defined(__GLIBC__)
struct fallocate64_event {
    const void *pc;
    int fd;
    int mode;
    off64_t offset;
    off64_t len;
    int ret;
    int errno_;
    int (*func)(int, int, off64_t, off64_t);
};
#endif

#if defined(__linux__) && defined(__GLIBC__)
struct posix_fallocate64_event {
    const void *pc;
    int fd;
    off64_t offset;
    off64_t len;
    int ret;
    int errno_;
    int (*func)(int, off64_t, off64_t);
};
#endif

#if defined(__linux__) && defined(__GLIBC__)
struct posix_fadvise64_event {
    const void *pc;
    int fd;
    off64_t offset;
    off64_t len;
    int advice;
    int ret;
    int errno_;
    int (*func)(int, off64_t, off64_t, int);
};
#endif

#endif /* DICE_FCNTL_H */
