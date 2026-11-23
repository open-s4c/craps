
/*
 * Copyright (C) 2026 Huawei Technologies Co., Ltd.
 * SPDX-License-Identifier: 0BSD
 */
#ifndef DICE_IOCTL_H
#define DICE_IOCTL_H

#include <stdint.h>
#include <sys/ioctl.h>

#ifndef DICE_ROLLED
#define EVENT_IOCTL 321
#endif

/*
 * ioctl() is variadic.  argp stores the raw third-argument value, which can
 * represent a pointer or an integer depending on op.  arg_kind records how
 * to forward it; it is not a description of the pointed-to memory.
 */
enum dice_ioctl_arg_kind {
    DICE_IOCTL_ARG_NONE,
    DICE_IOCTL_ARG_INT,
    DICE_IOCTL_ARG_PTR,
};

struct ioctl_event {
    const void *pc;
    int fd;
    unsigned long op;
    uintptr_t argp;
    enum dice_ioctl_arg_kind arg_kind;
    int ret;
    int errno_;
    int (*func)(int, unsigned long, ...);
};

#endif /* DICE_IOCTL_H */
