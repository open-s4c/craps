/*
 * Copyright (C) 2026 Huawei Technologies Co., Ltd.
 * SPDX-License-Identifier: 0BSD
 */
#ifndef DICE_DIRENT_H
#define DICE_DIRENT_H

#ifndef _GNU_SOURCE
#define _GNU_SOURCE
#endif
#ifndef _LARGEFILE64_SOURCE
#define _LARGEFILE64_SOURCE
#endif
#include <sys/types.h>
#include <dirent.h>

#define EVENT_CLOSEDIR  267
#define EVENT_FDOPENDIR 268
#define EVENT_OPENDIR   269
#define EVENT_READDIR   270
#define EVENT_READDIR_R 271
#define EVENT_REWINDDIR 272
#define EVENT_SEEKDIR   273
#define EVENT_TELLDIR   274
#define EVENT_SCANDIR   281
#define EVENT_SCANDIRAT 282
#define EVENT_SCANDIR64   283
#define EVENT_SCANDIRAT64 284

struct closedir_event {
    const void *pc;
    DIR *dirp;
    int ret;
    int(*func)(DIR *);
};

struct fdopendir_event {
    const void *pc;
    int fd;
    DIR *ret;
    DIR *(*func)(int);
};

struct opendir_event {
    const void *pc;
    const char *name;
    DIR *ret;
    DIR *(*func)(const char *);
};

struct readdir_event {
    const void *pc;
    DIR *dirp;
    struct dirent *ret;
    struct dirent *(*func)(DIR *);
};

struct readdir_r_event {
    const void *pc;
    DIR *dirp;
    struct dirent *entry;
    struct dirent **result;
    int ret;
    int(*func)(DIR *, struct dirent *, struct dirent **);
};

struct rewinddir_event {
    const void *pc;
    DIR *dirp;
    void(*func)(DIR *);
};

struct seekdir_event {
    const void *pc;
    DIR *dirp;
    long loc;
    void(*func)(DIR *, long);
};

struct telldir_event {
    const void *pc;
    DIR *dirp;
    long ret;
    long(*func)(DIR *);
};

struct scandir_event {
    const void *pc;
    const char *dirp;
    struct dirent ***namelist;
    int (*filter)(const struct dirent *);
    int (*compar)(
        const struct dirent **,
        const struct dirent **
    );
    int ret;
    int errno_;
    int (*func)(
        const char *,
        struct dirent ***,
        int (*)(const struct dirent *),
        int (*)(
            const struct dirent **,
            const struct dirent **
        )
    );
};

struct scandirat_event {
    const void *pc;
    int dirfd;
    const char *dirp;
    struct dirent ***namelist;
    int (*filter)(const struct dirent *);
    int (*compar)(
        const struct dirent **,
        const struct dirent **
    );
    int ret;
    int errno_;
    int (*func)(
        int,
        const char *,
        struct dirent ***,
        int (*)(const struct dirent *),
        int (*)(
            const struct dirent **,
            const struct dirent **
        )
    );
};

struct scandir64_event {
    const void *pc;
    const char *dirp;
    struct dirent64 ***namelist;
    int (*filter)(const struct dirent64 *);
    int (*compar)(
        const struct dirent64 **,
        const struct dirent64 **
    );
    int ret;
    int errno_;
    int (*func)(
        const char *,
        struct dirent64 ***,
        int (*)(const struct dirent64 *),
        int (*)(
            const struct dirent64 **,
            const struct dirent64 **
        )
    );
};

struct scandirat64_event {
    const void *pc;
    int dirfd;
    const char *dirp;
    struct dirent64 ***namelist;
    int (*filter)(const struct dirent64 *);
    int (*compar)(
        const struct dirent64 **,
        const struct dirent64 **
    );
    int ret;
    int errno_;
    int (*func)(
        int,
        const char *,
        struct dirent64 ***,
        int (*)(const struct dirent64 *),
        int (*)(
            const struct dirent64 **,
            const struct dirent64 **
        )
    );
};

/* Filesystem replay events */
#define EVENT_READDIR64              379
#define EVENT_READDIR64_R            380
#define EVENT_GETDENTS64             381

#if defined(__linux__) && defined(__GLIBC__)
struct readdir64_event {
    const void *pc;
    DIR *dirp;
    struct dirent64 *ret;
    int errno_;
    struct dirent64 *(*func)(DIR *);
};
#endif

#if defined(__linux__) && defined(__GLIBC__)
struct readdir64_r_event {
    const void *pc;
    DIR *dirp;
    struct dirent64 *entry;
    struct dirent64 **result;
    int ret;
    int errno_;
    int (*func)(DIR *, struct dirent64 *, struct dirent64 **);
};
#endif

#if defined(__linux__) && defined(__GLIBC__)
#if __GLIBC_PREREQ(2, 30)
struct getdents64_event {
    const void *pc;
    int fd;
    void *buf;
    size_t nbytes;
    ssize_t ret;
    int errno_;
    ssize_t (*func)(int, void *, size_t);
};
#endif
#endif

#endif /* DICE_DIRENT_H */
