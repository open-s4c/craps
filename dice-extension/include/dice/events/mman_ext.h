/*
 * Copyright (C) 2025 Huawei Technologies Co., Ltd.
 * SPDX-License-Identifier: 0BSD
 */
#ifndef DICE_MMAN_EXT_H
#define DICE_MMAN_EXT_H

#include <stddef.h>

#include <sys/mman.h>

#ifndef DICE_ROLLED
#define EVENT_MREMAP   259
#endif
#ifndef DICE_ROLLED
#define EVENT_MPROTECT 260
#endif

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

#endif /* DICE_MMAN_EXT_H */
