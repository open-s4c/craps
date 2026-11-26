/*
 * Copyright (C) 2025 Huawei Technologies Co., Ltd.
 * SPDX-License-Identifier: 0BSD
 */
#ifndef DICE_MMAN_EXT_H
#define DICE_MMAN_EXT_H

#ifndef _GNU_SOURCE
#define _GNU_SOURCE
#endif
#ifndef _LARGEFILE64_SOURCE
#define _LARGEFILE64_SOURCE
#endif
#include <stddef.h>

#include <sys/mman.h>

#define EVENT_MREMAP   259
#define EVENT_MPROTECT 260

struct mremap_event {
    const void *pc;
    void *old_address;
    size_t old_size;
    size_t new_size;
    int flags;
    void *new_address;
    void * ret;
    void *(*func)(void *, size_t, size_t, int, ...);
};

struct mprotect_event {
    const void *pc;
    void *addr;
    size_t length;
    int prot;
    int ret;
    int (*func)(void *, size_t, int);
};

/* Filesystem replay events */
#define EVENT_MEMFD_CREATE           393

#if defined(__linux__) && defined(__GLIBC__)
#if __GLIBC_PREREQ(2, 27)
struct memfd_create_event {
    const void *pc;
    const char *name;
    unsigned int flags;
    int ret;
    int errno_;
    int (*func)(const char *, unsigned int);
};
#endif
#endif

#endif /* DICE_MMAN_EXT_H */
