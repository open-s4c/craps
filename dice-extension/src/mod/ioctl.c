
/*
 * Copyright (C) 2026 Huawei Technologies Co., Ltd.
 * SPDX-License-Identifier: 0BSD
 */
#include <dice/chains/intercept.h>
#include <dice/events/ioctl.h>
#include <dice/interpose.h>
#include <dice/module.h>
#include <dice/pubsub.h>

#include <errno.h>
#include <stdarg.h>
#include <stdint.h>

/*
 * ioctl() has no universally reliable way to determine the type or presence
 * of its optional third argument from op.  Handle known no-argument and
 * integer-argument Linux terminal requests explicitly; remaining requests
 * are treated as pointer arguments.  Extend this list for other devices.
 */
static enum dice_ioctl_arg_kind
ioctl_arg_kind(unsigned long op)
{
    switch (op) {
#ifdef TIOCEXCL
    case TIOCEXCL:
#endif
#ifdef TIOCNXCL
    case TIOCNXCL:
#endif
#ifdef TIOCNOTTY
    case TIOCNOTTY:
#endif
#ifdef TIOCCONS
    case TIOCCONS:
#endif
#ifdef TIOCSBRK
    case TIOCSBRK:
#endif
#ifdef TIOCCBRK
    case TIOCCBRK:
#endif
#ifdef TIOCVHANGUP
    case TIOCVHANGUP:
#endif
#ifdef TIOCSERCONFIG
    case TIOCSERCONFIG:
#endif
#ifdef FIOCLEX
    case FIOCLEX:
#endif
#ifdef FIONCLEX
    case FIONCLEX:
#endif
        return DICE_IOCTL_ARG_NONE;

#ifdef TCSBRK
    case TCSBRK:
#endif
#ifdef TCSBRKP
    case TCSBRKP:
#endif
#ifdef TCXONC
    case TCXONC:
#endif
#ifdef TCFLSH
    case TCFLSH:
#endif
#ifdef TIOCSCTTY
    case TIOCSCTTY:
#endif
#ifdef TIOCMIWAIT
    case TIOCMIWAIT:
#endif
#ifdef TIOCGPTPEER
    case TIOCGPTPEER:
#endif
        return DICE_IOCTL_ARG_INT;

    default:
        return DICE_IOCTL_ARG_PTR;
    }
}

INTERPOSE(int, ioctl, int fd, unsigned long op, ...)
{
    enum dice_ioctl_arg_kind kind = ioctl_arg_kind(op);
    uintptr_t argp = 0;

    if (kind != DICE_IOCTL_ARG_NONE) {
        va_list ap;
        va_start(ap, op);
        if (kind == DICE_IOCTL_ARG_INT) {
            argp = (uintptr_t)(unsigned int)va_arg(ap, int);
        } else {
            /* Linux ABI: object-pointer arguments occupy one pointer slot. */
            argp = (uintptr_t)va_arg(ap, void *);
        }
        va_end(ap);
    }

    int saved_errno = errno;
    struct ioctl_event ev = {
        .pc       = INTERPOSE_PC,
        .fd       = fd,
        .op       = op,
        .argp     = argp,
        .arg_kind = kind,
        .ret      = 0,
        .errno_   = saved_errno,
        .func     = REAL_FUNC(ioctl),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_IOCTL, &ev, &md);

    errno = ev.errno_;
    switch (ev.arg_kind) {
    case DICE_IOCTL_ARG_NONE:
        ev.ret = ev.func(ev.fd, ev.op);
        break;
    case DICE_IOCTL_ARG_INT:
        ev.ret = ev.func(ev.fd, ev.op, (int)ev.argp);
        break;
    case DICE_IOCTL_ARG_PTR:
        ev.ret = ev.func(ev.fd, ev.op, (void *)ev.argp);
        break;
    }
    ev.errno_ = errno;

    PS_PUBLISH(INTERCEPT_AFTER, EVENT_IOCTL, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}

PS_ADVERTISE_TYPE(EVENT_IOCTL)

DICE_MODULE_INIT()
