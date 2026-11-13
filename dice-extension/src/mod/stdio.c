/*
 * Copyright (C) 2026 Huawei Technologies Co., Ltd.
 * SPDX-License-Identifier: 0BSD
 */
#include <dice/chains/intercept.h>
#include <dice/events/stdio.h>
#include <dice/interpose.h>
#include <dice/module.h>
#include <dice/pubsub.h>
#include <errno.h>
#include <stdarg.h>

/* Several stdio interfaces can be function-like macros.  Interpose symbols. */
#ifdef getc
#undef getc
#endif
#ifdef getchar
#undef getchar
#endif
#ifdef putc
#undef putc
#endif
#ifdef putchar
#undef putchar
#endif
#ifdef clearerr
#undef clearerr
#endif
#ifdef feof
#undef feof
#endif
#ifdef ferror
#undef ferror
#endif

INTERPOSE(void, clearerr, FILE *stream)
{
    int saved_errno = errno;
    struct clearerr_event ev = {
        .pc      = INTERPOSE_PC,
        .stream  = stream,
        .errno_  = saved_errno,
        .func    = REAL_FUNC(clearerr),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_CLEARERR, &ev, &md);
    errno = ev.errno_;
    ev.func(ev.stream);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_CLEARERR, &ev, &md);
    errno = ev.errno_;
}

INTERPOSE(int, fclose, FILE *stream)
{
    int saved_errno = errno;
    struct fclose_event ev = {
        .pc      = INTERPOSE_PC,
        .stream  = stream,
        .ret     = 0,
        .errno_  = saved_errno,
        .func    = REAL_FUNC(fclose),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_FCLOSE, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.stream);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_FCLOSE, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}

INTERPOSE(int, feof, FILE *stream)
{
    int saved_errno = errno;
    struct feof_event ev = {
        .pc      = INTERPOSE_PC,
        .stream  = stream,
        .ret     = 0,
        .errno_  = saved_errno,
        .func    = REAL_FUNC(feof),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_FEOF, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.stream);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_FEOF, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}

INTERPOSE(int, ferror, FILE *stream)
{
    int saved_errno = errno;
    struct ferror_event ev = {
        .pc      = INTERPOSE_PC,
        .stream  = stream,
        .ret     = 0,
        .errno_  = saved_errno,
        .func    = REAL_FUNC(ferror),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_FERROR, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.stream);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_FERROR, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}

INTERPOSE(int, fflush, FILE *stream)
{
    int saved_errno = errno;
    struct fflush_event ev = {
        .pc      = INTERPOSE_PC,
        .stream  = stream,
        .ret     = 0,
        .errno_  = saved_errno,
        .func    = REAL_FUNC(fflush),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_FFLUSH, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.stream);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_FFLUSH, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}

INTERPOSE(int, fgetc, FILE *stream)
{
    int saved_errno = errno;
    struct fgetc_event ev = {
        .pc      = INTERPOSE_PC,
        .stream  = stream,
        .ret     = 0,
        .errno_  = saved_errno,
        .func    = REAL_FUNC(fgetc),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_FGETC, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.stream);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_FGETC, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}

INTERPOSE(int, fgetpos, FILE *restrict stream, fpos_t *restrict pos)
{
    int saved_errno = errno;
    struct fgetpos_event ev = {
        .pc      = INTERPOSE_PC,
        .stream  = stream,
        .pos     = pos,
        .ret     = 0,
        .errno_  = saved_errno,
        .func    = REAL_FUNC(fgetpos),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_FGETPOS, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.stream, ev.pos);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_FGETPOS, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}

INTERPOSE(char *, fgets, char *restrict s, int size, FILE *restrict stream)
{
    int saved_errno = errno;
    struct fgets_event ev = {
        .pc      = INTERPOSE_PC,
        .s       = s,
        .size    = size,
        .stream  = stream,
        .ret     = NULL,
        .errno_  = saved_errno,
        .func    = REAL_FUNC(fgets),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_FGETS, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.s, ev.size, ev.stream);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_FGETS, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}

INTERPOSE(FILE *, fopen, const char *restrict path, const char *restrict mode)
{
    int saved_errno = errno;
    struct fopen_event ev = {
        .pc      = INTERPOSE_PC,
        .path    = path,
        .mode    = mode,
        .ret     = NULL,
        .errno_  = saved_errno,
        .func    = REAL_FUNC(fopen),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_FOPEN, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.path, ev.mode);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_FOPEN, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}

INTERPOSE(int, fputc, int c, FILE *stream)
{
    int saved_errno = errno;
    struct fputc_event ev = {
        .pc      = INTERPOSE_PC,
        .c       = c,
        .stream  = stream,
        .ret     = 0,
        .errno_  = saved_errno,
        .func    = REAL_FUNC(fputc),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_FPUTC, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.c, ev.stream);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_FPUTC, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}

INTERPOSE(int, fputs, const char *restrict s, FILE *restrict stream)
{
    int saved_errno = errno;
    struct fputs_event ev = {
        .pc      = INTERPOSE_PC,
        .s       = s,
        .stream  = stream,
        .ret     = 0,
        .errno_  = saved_errno,
        .func    = REAL_FUNC(fputs),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_FPUTS, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.s, ev.stream);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_FPUTS, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}

INTERPOSE(size_t, fread, void *restrict ptr, size_t size, size_t n, FILE *restrict stream)
{
    int saved_errno = errno;
    struct fread_event ev = {
        .pc      = INTERPOSE_PC,
        .ptr     = ptr,
        .size    = size,
        .n       = n,
        .stream  = stream,
        .ret     = 0,
        .errno_  = saved_errno,
        .func    = REAL_FUNC(fread),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_FREAD, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.ptr, ev.size, ev.n, ev.stream);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_FREAD, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}

INTERPOSE(FILE *, freopen, const char *restrict path, const char *restrict mode, FILE *restrict stream)
{
    int saved_errno = errno;
    struct freopen_event ev = {
        .pc      = INTERPOSE_PC,
        .path    = path,
        .mode    = mode,
        .stream  = stream,
        .ret     = NULL,
        .errno_  = saved_errno,
        .func    = REAL_FUNC(freopen),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_FREOPEN, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.path, ev.mode, ev.stream);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_FREOPEN, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}

INTERPOSE(int, fseek, FILE *stream, long offset, int whence)
{
    int saved_errno = errno;
    struct fseek_event ev = {
        .pc      = INTERPOSE_PC,
        .stream  = stream,
        .offset  = offset,
        .whence  = whence,
        .ret     = 0,
        .errno_  = saved_errno,
        .func    = REAL_FUNC(fseek),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_FSEEK, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.stream, ev.offset, ev.whence);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_FSEEK, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}

INTERPOSE(int, fsetpos, FILE *stream, const fpos_t *pos)
{
    int saved_errno = errno;
    struct fsetpos_event ev = {
        .pc      = INTERPOSE_PC,
        .stream  = stream,
        .pos     = pos,
        .ret     = 0,
        .errno_  = saved_errno,
        .func    = REAL_FUNC(fsetpos),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_FSETPOS, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.stream, ev.pos);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_FSETPOS, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}

INTERPOSE(long, ftell, FILE *stream)
{
    int saved_errno = errno;
    struct ftell_event ev = {
        .pc      = INTERPOSE_PC,
        .stream  = stream,
        .ret     = 0,
        .errno_  = saved_errno,
        .func    = REAL_FUNC(ftell),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_FTELL, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.stream);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_FTELL, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}

INTERPOSE(size_t, fwrite, const void *restrict ptr, size_t size, size_t n, FILE *restrict stream)
{
    int saved_errno = errno;
    struct fwrite_event ev = {
        .pc      = INTERPOSE_PC,
        .ptr     = ptr,
        .size    = size,
        .n       = n,
        .stream  = stream,
        .ret     = 0,
        .errno_  = saved_errno,
        .func    = REAL_FUNC(fwrite),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_FWRITE, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.ptr, ev.size, ev.n, ev.stream);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_FWRITE, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}

INTERPOSE(int, getc, FILE *stream)
{
    int saved_errno = errno;
    struct getc_event ev = {
        .pc      = INTERPOSE_PC,
        .stream  = stream,
        .ret     = 0,
        .errno_  = saved_errno,
        .func    = REAL_FUNC(getc),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_GETC, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.stream);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_GETC, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}

INTERPOSE(int, getchar, void)
{
    int saved_errno = errno;
    struct getchar_event ev = {
        .pc      = INTERPOSE_PC,
        .ret     = 0,
        .errno_  = saved_errno,
        .func    = REAL_FUNC(getchar),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_GETCHAR, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func();
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_GETCHAR, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}

INTERPOSE(void, perror, const char *s)
{
    int saved_errno = errno;
    struct perror_event ev = {
        .pc      = INTERPOSE_PC,
        .s       = s,
        .errno_  = saved_errno,
        .func    = REAL_FUNC(perror),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_PERROR, &ev, &md);
    errno = ev.errno_;
    ev.func(ev.s);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_PERROR, &ev, &md);
    errno = ev.errno_;
}

INTERPOSE(int, putc, int c, FILE *stream)
{
    int saved_errno = errno;
    struct putc_event ev = {
        .pc      = INTERPOSE_PC,
        .c       = c,
        .stream  = stream,
        .ret     = 0,
        .errno_  = saved_errno,
        .func    = REAL_FUNC(putc),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_PUTC, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.c, ev.stream);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_PUTC, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}

INTERPOSE(int, putchar, int c)
{
    int saved_errno = errno;
    struct putchar_event ev = {
        .pc      = INTERPOSE_PC,
        .c       = c,
        .ret     = 0,
        .errno_  = saved_errno,
        .func    = REAL_FUNC(putchar),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_PUTCHAR, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.c);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_PUTCHAR, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}

INTERPOSE(int, puts, const char *s)
{
    int saved_errno = errno;
    struct puts_event ev = {
        .pc      = INTERPOSE_PC,
        .s       = s,
        .ret     = 0,
        .errno_  = saved_errno,
        .func    = REAL_FUNC(puts),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_PUTS, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.s);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_PUTS, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}

INTERPOSE(int, remove, const char *path)
{
    int saved_errno = errno;
    struct remove_event ev = {
        .pc      = INTERPOSE_PC,
        .path    = path,
        .ret     = 0,
        .errno_  = saved_errno,
        .func    = REAL_FUNC(remove),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_REMOVE, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.path);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_REMOVE, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}

INTERPOSE(int, rename, const char *oldpath, const char *newpath)
{
    int saved_errno = errno;
    struct rename_event ev = {
        .pc      = INTERPOSE_PC,
        .oldpath = oldpath,
        .newpath = newpath,
        .ret     = 0,
        .errno_  = saved_errno,
        .func    = REAL_FUNC(rename),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_RENAME, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.oldpath, ev.newpath);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_RENAME, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}

INTERPOSE(void, rewind, FILE *stream)
{
    int saved_errno = errno;
    struct rewind_event ev = {
        .pc      = INTERPOSE_PC,
        .stream  = stream,
        .errno_  = saved_errno,
        .func    = REAL_FUNC(rewind),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_REWIND, &ev, &md);
    errno = ev.errno_;
    ev.func(ev.stream);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_REWIND, &ev, &md);
    errno = ev.errno_;
}

INTERPOSE(void, setbuf, FILE *restrict stream, char *restrict buf)
{
    int saved_errno = errno;
    struct setbuf_event ev = {
        .pc      = INTERPOSE_PC,
        .stream  = stream,
        .buf     = buf,
        .errno_  = saved_errno,
        .func    = REAL_FUNC(setbuf),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_SETBUF, &ev, &md);
    errno = ev.errno_;
    ev.func(ev.stream, ev.buf);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_SETBUF, &ev, &md);
    errno = ev.errno_;
}

INTERPOSE(int, setvbuf, FILE *restrict stream, char *restrict buf, int mode, size_t size)
{
    int saved_errno = errno;
    struct setvbuf_event ev = {
        .pc      = INTERPOSE_PC,
        .stream  = stream,
        .buf     = buf,
        .mode    = mode,
        .size    = size,
        .ret     = 0,
        .errno_  = saved_errno,
        .func    = REAL_FUNC(setvbuf),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_SETVBUF, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.stream, ev.buf, ev.mode, ev.size);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_SETVBUF, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}

INTERPOSE(FILE *, tmpfile, void)
{
    int saved_errno = errno;
    struct tmpfile_event ev = {
        .pc      = INTERPOSE_PC,
        .ret     = NULL,
        .errno_  = saved_errno,
        .func    = REAL_FUNC(tmpfile),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_TMPFILE, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func();
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_TMPFILE, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}

INTERPOSE(char *, tmpnam, char s[L_tmpnam])
{
    int saved_errno = errno;
    struct tmpnam_event ev = {
        .pc      = INTERPOSE_PC,
        .s       = s,
        .ret     = NULL,
        .errno_  = saved_errno,
        .func    = REAL_FUNC(tmpnam),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_TMPNAM, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.s);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_TMPNAM, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}

INTERPOSE(int, ungetc, int c, FILE *stream)
{
    int saved_errno = errno;
    struct ungetc_event ev = {
        .pc      = INTERPOSE_PC,
        .c       = c,
        .stream  = stream,
        .ret     = 0,
        .errno_  = saved_errno,
        .func    = REAL_FUNC(ungetc),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_UNGETC, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.c, ev.stream);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_UNGETC, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}

INTERPOSE(int, vfprintf, FILE *restrict stream, const char *restrict format, va_list ap)
{
    int saved_errno = errno;
    struct vfprintf_event ev = {
        .pc      = INTERPOSE_PC,
        .stream  = stream,
        .format  = format,
        .ret     = 0,
        .errno_  = saved_errno,
        .func    = REAL_FUNC(vfprintf),
    };

    va_copy(ev.ap, ap);
    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_VFPRINTF, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.stream, ev.format, ev.ap);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_VFPRINTF, &ev, &md);
    va_end(ev.ap);
    errno = ev.errno_;
    return ev.ret;
}

INTERPOSE(int, vprintf, const char *restrict format, va_list ap)
{
    int saved_errno = errno;
    struct vprintf_event ev = {
        .pc      = INTERPOSE_PC,
        .format  = format,
        .ret     = 0,
        .errno_  = saved_errno,
        .func    = REAL_FUNC(vprintf),
    };

    va_copy(ev.ap, ap);
    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_VPRINTF, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.format, ev.ap);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_VPRINTF, &ev, &md);
    va_end(ev.ap);
    errno = ev.errno_;
    return ev.ret;
}

INTERPOSE(int, vsnprintf, char *restrict str, size_t size, const char *restrict format, va_list ap)
{
    int saved_errno = errno;
    struct vsnprintf_event ev = {
        .pc      = INTERPOSE_PC,
        .str     = str,
        .size    = size,
        .format  = format,
        .ret     = 0,
        .errno_  = saved_errno,
        .func    = REAL_FUNC(vsnprintf),
    };

    va_copy(ev.ap, ap);
    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_VSNPRINTF, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.str, ev.size, ev.format, ev.ap);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_VSNPRINTF, &ev, &md);
    va_end(ev.ap);
    errno = ev.errno_;
    return ev.ret;
}

INTERPOSE(int, vsprintf, char *restrict str, const char *restrict format, va_list ap)
{
    int saved_errno = errno;
    struct vsprintf_event ev = {
        .pc      = INTERPOSE_PC,
        .str     = str,
        .format  = format,
        .ret     = 0,
        .errno_  = saved_errno,
        .func    = REAL_FUNC(vsprintf),
    };

    va_copy(ev.ap, ap);
    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_VSPRINTF, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.str, ev.format, ev.ap);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_VSPRINTF, &ev, &md);
    va_end(ev.ap);
    errno = ev.errno_;
    return ev.ret;
}

/* Advertise event types */
PS_ADVERTISE_TYPE(EVENT_CLEARERR)
PS_ADVERTISE_TYPE(EVENT_FCLOSE)
PS_ADVERTISE_TYPE(EVENT_FEOF)
PS_ADVERTISE_TYPE(EVENT_FERROR)
PS_ADVERTISE_TYPE(EVENT_FFLUSH)
PS_ADVERTISE_TYPE(EVENT_FGETC)
PS_ADVERTISE_TYPE(EVENT_FGETPOS)
PS_ADVERTISE_TYPE(EVENT_FGETS)
PS_ADVERTISE_TYPE(EVENT_FOPEN)
PS_ADVERTISE_TYPE(EVENT_FPUTC)
PS_ADVERTISE_TYPE(EVENT_FPUTS)
PS_ADVERTISE_TYPE(EVENT_FREAD)
PS_ADVERTISE_TYPE(EVENT_FREOPEN)
PS_ADVERTISE_TYPE(EVENT_FSEEK)
PS_ADVERTISE_TYPE(EVENT_FSETPOS)
PS_ADVERTISE_TYPE(EVENT_FTELL)
PS_ADVERTISE_TYPE(EVENT_FWRITE)
PS_ADVERTISE_TYPE(EVENT_GETC)
PS_ADVERTISE_TYPE(EVENT_GETCHAR)
PS_ADVERTISE_TYPE(EVENT_PERROR)
PS_ADVERTISE_TYPE(EVENT_PUTC)
PS_ADVERTISE_TYPE(EVENT_PUTCHAR)
PS_ADVERTISE_TYPE(EVENT_PUTS)
PS_ADVERTISE_TYPE(EVENT_REMOVE)
PS_ADVERTISE_TYPE(EVENT_RENAME)
PS_ADVERTISE_TYPE(EVENT_REWIND)
PS_ADVERTISE_TYPE(EVENT_SETBUF)
PS_ADVERTISE_TYPE(EVENT_SETVBUF)
PS_ADVERTISE_TYPE(EVENT_TMPFILE)
PS_ADVERTISE_TYPE(EVENT_TMPNAM)
PS_ADVERTISE_TYPE(EVENT_UNGETC)
PS_ADVERTISE_TYPE(EVENT_VFPRINTF)
PS_ADVERTISE_TYPE(EVENT_VPRINTF)
PS_ADVERTISE_TYPE(EVENT_VSNPRINTF)
PS_ADVERTISE_TYPE(EVENT_VSPRINTF)

/* Module init (optional) */
DICE_MODULE_INIT()
