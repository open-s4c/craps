/*
 * Copyright (C) 2025 Huawei Technologies Co., Ltd.
 * SPDX-License-Identifier: 0BSD
 */
#include <dice/chains/intercept.h>
#include <dice/events/mman_ext.h>
#include <dice/interpose.h>
#include <dice/module.h>
#include <stdarg.h>

INTERPOSE(void *, mremap,
          void *old_address,
          size_t old_size,
          size_t new_size,
          int flags,
          ...)
{
    va_list ap;
    va_start(ap, flags);
    void *new_address = va_arg(ap, void *);
    struct mremap_event ev = {
        .pc          = INTERPOSE_PC,
        .old_address = old_address,
        .old_size    = old_size,
        .new_size    = new_size,
        .flags       = flags,
        .new_address = new_address,
        .ret         = NULL,
        .func        = REAL_FUNC(mremap),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_MREMAP, &ev, &md);
    ev.ret = ev.func(
        ev.old_address,
        ev.old_size,
        ev.new_size,
        ev.flags,
        ev.new_address);
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_MREMAP, &ev, &md);

    return ev.ret;
}

INTERPOSE(int, mprotect, void *addr, size_t length, int prot)
{
    struct mprotect_event ev = {
        .pc     = INTERPOSE_PC,
        .addr   = addr,
        .length = length,
        .prot   = prot,
        .ret    = 0,
        .func   = REAL_FUNC(mprotect),
    };

    struct metadata md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_MPROTECT, &ev, &md);
    ev.ret = ev.func(ev.addr, ev.length, ev.prot);
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_MPROTECT, &ev, &md);
    return ev.ret;
}

/* Advertise event type names for debugging messages */
PS_ADVERTISE_TYPE(EVENT_MREMAP)
PS_ADVERTISE_TYPE(EVENT_MPROTECT)
/* Mark module initialization (optional) */
DICE_MODULE_INIT()
