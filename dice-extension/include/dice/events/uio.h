/*
 * Copyright (C) 2026 Huawei Technologies Co., Ltd.
 * SPDX-License-Identifier: 0BSD
 */
#ifndef DICE_UIO_H
#define DICE_UIO_H

#ifndef _GNU_SOURCE
#define _GNU_SOURCE
#endif
#ifndef _LARGEFILE64_SOURCE
#define _LARGEFILE64_SOURCE
#endif
#include <sys/uio.h>

#define EVENT_READV   135
#define EVENT_WRITEV  136

struct readv_event {
    const void *pc;
    int fd;
    const struct iovec *iov;
    int iovcnt;
    ssize_t ret;
    int errno_;
    ssize_t(*func)(int, const struct iovec *, int);
};

struct writev_event {
    const void *pc;
    int fd;
    const struct iovec *iov;
    int iovcnt;
    ssize_t ret;
    int errno_;
    ssize_t(*func)(int, const struct iovec *, int);
};

/* Filesystem replay events */
#define EVENT_PREADV                 365
#define EVENT_PREADV64               366
#define EVENT_PREADV2                367
#define EVENT_PREADV64V2             368
#define EVENT_PWRITEV                369
#define EVENT_PWRITEV64              370
#define EVENT_PWRITEV2               371
#define EVENT_PWRITEV64V2            372

#if defined(__linux__) && defined(__GLIBC__)
struct preadv_event {
    const void *pc;
    int fd;
    const struct iovec *iov;
    int iovcnt;
    off_t offset;
    ssize_t ret;
    int errno_;
    ssize_t (*func)(int, const struct iovec *, int, off_t);
};
#endif

#if defined(__linux__) && defined(__GLIBC__)
struct preadv64_event {
    const void *pc;
    int fd;
    const struct iovec *iov;
    int iovcnt;
    off64_t offset;
    ssize_t ret;
    int errno_;
    ssize_t (*func)(int, const struct iovec *, int, off64_t);
};
#endif

#if defined(__linux__) && defined(__GLIBC__)
#if __GLIBC_PREREQ(2, 26)
struct preadv2_event {
    const void *pc;
    int fd;
    const struct iovec *iov;
    int iovcnt;
    off_t offset;
    int flags;
    ssize_t ret;
    int errno_;
    ssize_t (*func)(int, const struct iovec *, int, off_t, int);
};
#endif
#endif

#if defined(__linux__) && defined(__GLIBC__)
#if __GLIBC_PREREQ(2, 26)
struct preadv64v2_event {
    const void *pc;
    int fd;
    const struct iovec *iov;
    int iovcnt;
    off64_t offset;
    int flags;
    ssize_t ret;
    int errno_;
    ssize_t (*func)(int, const struct iovec *, int, off64_t, int);
};
#endif
#endif

#if defined(__linux__) && defined(__GLIBC__)
struct pwritev_event {
    const void *pc;
    int fd;
    const struct iovec *iov;
    int iovcnt;
    off_t offset;
    ssize_t ret;
    int errno_;
    ssize_t (*func)(int, const struct iovec *, int, off_t);
};
#endif

#if defined(__linux__) && defined(__GLIBC__)
struct pwritev64_event {
    const void *pc;
    int fd;
    const struct iovec *iov;
    int iovcnt;
    off64_t offset;
    ssize_t ret;
    int errno_;
    ssize_t (*func)(int, const struct iovec *, int, off64_t);
};
#endif

#if defined(__linux__) && defined(__GLIBC__)
#if __GLIBC_PREREQ(2, 26)
struct pwritev2_event {
    const void *pc;
    int fd;
    const struct iovec *iov;
    int iovcnt;
    off_t offset;
    int flags;
    ssize_t ret;
    int errno_;
    ssize_t (*func)(int, const struct iovec *, int, off_t, int);
};
#endif
#endif

#if defined(__linux__) && defined(__GLIBC__)
#if __GLIBC_PREREQ(2, 26)
struct pwritev64v2_event {
    const void *pc;
    int fd;
    const struct iovec *iov;
    int iovcnt;
    off64_t offset;
    int flags;
    ssize_t ret;
    int errno_;
    ssize_t (*func)(int, const struct iovec *, int, off64_t, int);
};
#endif
#endif

#endif /* DICE_UIO_H */
