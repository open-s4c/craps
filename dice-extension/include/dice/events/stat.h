/*
 * Copyright (C) 2026 Huawei Technologies Co., Ltd.
 * SPDX-License-Identifier: 0BSD
 */
#ifndef DICE_STAT_H
#define DICE_STAT_H

#ifndef _GNU_SOURCE
#define _GNU_SOURCE
#endif
#ifndef _LARGEFILE64_SOURCE
#define _LARGEFILE64_SOURCE
#endif
#include <sys/types.h>
#include <sys/stat.h>

/* Filesystem replay events */
#define EVENT_CHMOD                  322
#define EVENT_FCHMOD                 323
#define EVENT_FCHMODAT               324
#define EVENT_LCHMOD                 325
#define EVENT_FSTAT                  326
#define EVENT_FSTATAT                327
#define EVENT_LSTAT                  328
#define EVENT_STAT                   329
#define EVENT_FSTAT64                330
#define EVENT_FSTATAT64              331
#define EVENT_LSTAT64                332
#define EVENT_STAT64                 333
#define EVENT_STATX                  334
#define EVENT_MKDIR                  335
#define EVENT_MKDIRAT                336
#define EVENT_MKFIFO                 337
#define EVENT_MKFIFOAT               338
#define EVENT_MKNOD                  339
#define EVENT_MKNODAT                340
#define EVENT_UMASK                  341
#define EVENT_XSTAT                  342
#define EVENT_FXSTAT                 343
#define EVENT_LXSTAT                 344
#define EVENT_FXSTATAT               345
#define EVENT_XSTAT64                346
#define EVENT_FXSTAT64               347
#define EVENT_LXSTAT64               348
#define EVENT_FXSTATAT64             349
#define EVENT_XMKNOD                 350
#define EVENT_XMKNODAT               351

struct chmod_event {
    const void *pc;
    const char *path;
    mode_t mode;
    int ret;
    int errno_;
    int (*func)(const char *, mode_t);
};

struct fchmod_event {
    const void *pc;
    int fd;
    mode_t mode;
    int ret;
    int errno_;
    int (*func)(int, mode_t);
};

struct fchmodat_event {
    const void *pc;
    int dirfd;
    const char *path;
    mode_t mode;
    int flags;
    int ret;
    int errno_;
    int (*func)(int, const char *, mode_t, int);
};

struct lchmod_event {
    const void *pc;
    const char *path;
    mode_t mode;
    int ret;
    int errno_;
    int (*func)(const char *, mode_t);
};

struct fstat_event {
    const void *pc;
    int fd;
    struct stat *buf;
    int ret;
    int errno_;
    int (*func)(int, struct stat *);
};

struct fstatat_event {
    const void *pc;
    int dirfd;
    const char *path;
    struct stat *buf;
    int flags;
    int ret;
    int errno_;
    int (*func)(int, const char *, struct stat *, int);
};

struct lstat_event {
    const void *pc;
    const char *path;
    struct stat *buf;
    int ret;
    int errno_;
    int (*func)(const char *, struct stat *);
};

struct stat_event {
    const void *pc;
    const char *path;
    struct stat *buf;
    int ret;
    int errno_;
    int (*func)(const char *, struct stat *);
};

#if defined(__linux__) && defined(__GLIBC__)
struct fstat64_event {
    const void *pc;
    int fd;
    struct stat64 *buf;
    int ret;
    int errno_;
    int (*func)(int, struct stat64 *);
};
#endif

#if defined(__linux__) && defined(__GLIBC__)
struct fstatat64_event {
    const void *pc;
    int dirfd;
    const char *path;
    struct stat64 *buf;
    int flags;
    int ret;
    int errno_;
    int (*func)(int, const char *, struct stat64 *, int);
};
#endif

#if defined(__linux__) && defined(__GLIBC__)
struct lstat64_event {
    const void *pc;
    const char *path;
    struct stat64 *buf;
    int ret;
    int errno_;
    int (*func)(const char *, struct stat64 *);
};
#endif

#if defined(__linux__) && defined(__GLIBC__)
struct stat64_event {
    const void *pc;
    const char *path;
    struct stat64 *buf;
    int ret;
    int errno_;
    int (*func)(const char *, struct stat64 *);
};
#endif

#if defined(__linux__) && defined(__GLIBC__)
#if __GLIBC_PREREQ(2, 28)
struct statx_event {
    const void *pc;
    int dirfd;
    const char *pathname;
    int flags;
    unsigned int mask;
    struct statx *statxbuf;
    int ret;
    int errno_;
    int (*func)(int, const char *, int, unsigned int, struct statx *);
};
#endif
#endif

struct mkdir_event {
    const void *pc;
    const char *path;
    mode_t mode;
    int ret;
    int errno_;
    int (*func)(const char *, mode_t);
};

struct mkdirat_event {
    const void *pc;
    int dirfd;
    const char *path;
    mode_t mode;
    int ret;
    int errno_;
    int (*func)(int, const char *, mode_t);
};

struct mkfifo_event {
    const void *pc;
    const char *path;
    mode_t mode;
    int ret;
    int errno_;
    int (*func)(const char *, mode_t);
};

struct mkfifoat_event {
    const void *pc;
    int dirfd;
    const char *path;
    mode_t mode;
    int ret;
    int errno_;
    int (*func)(int, const char *, mode_t);
};

struct mknod_event {
    const void *pc;
    const char *path;
    mode_t mode;
    dev_t dev;
    int ret;
    int errno_;
    int (*func)(const char *, mode_t, dev_t);
};

struct mknodat_event {
    const void *pc;
    int dirfd;
    const char *path;
    mode_t mode;
    dev_t dev;
    int ret;
    int errno_;
    int (*func)(int, const char *, mode_t, dev_t);
};

struct umask_event {
    const void *pc;
    mode_t mask;
    mode_t ret;
    int errno_;
    mode_t (*func)(mode_t);
};

#if defined(__linux__) && defined(__GLIBC__)
struct xstat_event {
    const void *pc;
    int ver;
    const char *path;
    struct stat *buf;
    int ret;
    int errno_;
    int (*func)(int, const char *, struct stat *);
};
#endif

#if defined(__linux__) && defined(__GLIBC__)
struct fxstat_event {
    const void *pc;
    int ver;
    int fd;
    struct stat *buf;
    int ret;
    int errno_;
    int (*func)(int, int, struct stat *);
};
#endif

#if defined(__linux__) && defined(__GLIBC__)
struct lxstat_event {
    const void *pc;
    int ver;
    const char *path;
    struct stat *buf;
    int ret;
    int errno_;
    int (*func)(int, const char *, struct stat *);
};
#endif

#if defined(__linux__) && defined(__GLIBC__)
struct fxstatat_event {
    const void *pc;
    int ver;
    int dirfd;
    const char *path;
    struct stat *buf;
    int flags;
    int ret;
    int errno_;
    int (*func)(int, int, const char *, struct stat *, int);
};
#endif

#if defined(__linux__) && defined(__GLIBC__)
struct xstat64_event {
    const void *pc;
    int ver;
    const char *path;
    struct stat64 *buf;
    int ret;
    int errno_;
    int (*func)(int, const char *, struct stat64 *);
};
#endif

#if defined(__linux__) && defined(__GLIBC__)
struct fxstat64_event {
    const void *pc;
    int ver;
    int fd;
    struct stat64 *buf;
    int ret;
    int errno_;
    int (*func)(int, int, struct stat64 *);
};
#endif

#if defined(__linux__) && defined(__GLIBC__)
struct lxstat64_event {
    const void *pc;
    int ver;
    const char *path;
    struct stat64 *buf;
    int ret;
    int errno_;
    int (*func)(int, const char *, struct stat64 *);
};
#endif

#if defined(__linux__) && defined(__GLIBC__)
struct fxstatat64_event {
    const void *pc;
    int ver;
    int dirfd;
    const char *path;
    struct stat64 *buf;
    int flags;
    int ret;
    int errno_;
    int (*func)(int, int, const char *, struct stat64 *, int);
};
#endif

#if defined(__linux__) && defined(__GLIBC__)
struct xmknod_event {
    const void *pc;
    int ver;
    const char *path;
    mode_t mode;
    dev_t *dev;
    int ret;
    int errno_;
    int (*func)(int, const char *, mode_t, dev_t *);
};
#endif

#if defined(__linux__) && defined(__GLIBC__)
struct xmknodat_event {
    const void *pc;
    int ver;
    int dirfd;
    const char *path;
    mode_t mode;
    dev_t *dev;
    int ret;
    int errno_;
    int (*func)(int, int, const char *, mode_t, dev_t *);
};
#endif

#endif /* DICE_STAT_H */
