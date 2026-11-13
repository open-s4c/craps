/*
 * Copyright (C) 2026 Huawei Technologies Co., Ltd.
 * SPDX-License-Identifier: 0BSD
 */
#ifndef DICE_STDIO_H
#define DICE_STDIO_H
#include <stdarg.h>
#include <stdio.h>

#define EVENT_CLEARERR     285
#define EVENT_FCLOSE       286
#define EVENT_FEOF         287
#define EVENT_FERROR       288
#define EVENT_FFLUSH       289
#define EVENT_FGETC        290
#define EVENT_FGETPOS      291
#define EVENT_FGETS        292
#define EVENT_FOPEN        293
#define EVENT_FPUTC        294
#define EVENT_FPUTS        295
#define EVENT_FREAD        296
#define EVENT_FREOPEN      297
#define EVENT_FSEEK        298
#define EVENT_FSETPOS      299
#define EVENT_FTELL        300
#define EVENT_FWRITE       301
#define EVENT_GETC         302
#define EVENT_GETCHAR      303
#define EVENT_PERROR       304
#define EVENT_PUTC         305
#define EVENT_PUTCHAR      306
#define EVENT_PUTS         307
#define EVENT_REMOVE       308
#define EVENT_RENAME       309
#define EVENT_REWIND       310
#define EVENT_SETBUF       311
#define EVENT_SETVBUF      312
#define EVENT_TMPFILE      313
#define EVENT_TMPNAM       314
#define EVENT_UNGETC       315
#define EVENT_VFPRINTF     316
#define EVENT_VPRINTF      317
#define EVENT_VSNPRINTF    318
#define EVENT_VSPRINTF     319

struct clearerr_event {
    const void *pc;
    FILE *stream;
    int errno_;
    void (*func)(FILE *);
};

struct fclose_event {
    const void *pc;
    FILE *stream;
    int ret;
    int errno_;
    int (*func)(FILE *);
};

struct feof_event {
    const void *pc;
    FILE *stream;
    int ret;
    int errno_;
    int (*func)(FILE *);
};

struct ferror_event {
    const void *pc;
    FILE *stream;
    int ret;
    int errno_;
    int (*func)(FILE *);
};

struct fflush_event {
    const void *pc;
    FILE *stream;
    int ret;
    int errno_;
    int (*func)(FILE *);
};

struct fgetc_event {
    const void *pc;
    FILE *stream;
    int ret;
    int errno_;
    int (*func)(FILE *);
};

struct fgetpos_event {
    const void *pc;
    FILE *stream;
    fpos_t *pos;
    int ret;
    int errno_;
    int (*func)(FILE *, fpos_t *);
};

struct fgets_event {
    const void *pc;
    char *s;
    int size;
    FILE *stream;
    char *ret;
    int errno_;
    char *(*func)(char *, int, FILE *);
};

struct fopen_event {
    const void *pc;
    const char *path;
    const char *mode;
    FILE *ret;
    int errno_;
    FILE *(*func)(const char *, const char *);
};

struct fputc_event {
    const void *pc;
    int c;
    FILE *stream;
    int ret;
    int errno_;
    int (*func)(int, FILE *);
};

struct fputs_event {
    const void *pc;
    const char *s;
    FILE *stream;
    int ret;
    int errno_;
    int (*func)(const char *, FILE *);
};

struct fread_event {
    const void *pc;
    void *ptr;
    size_t size;
    size_t n;
    FILE *stream;
    size_t ret;
    int errno_;
    size_t (*func)(void *, size_t, size_t, FILE *);
};

struct freopen_event {
    const void *pc;
    const char *path;
    const char *mode;
    FILE *stream;
    FILE *ret;
    int errno_;
    FILE *(*func)(const char *, const char *, FILE *);
};

struct fseek_event {
    const void *pc;
    FILE *stream;
    long offset;
    int whence;
    int ret;
    int errno_;
    int (*func)(FILE *, long, int);
};

struct fsetpos_event {
    const void *pc;
    FILE *stream;
    const fpos_t *pos;
    int ret;
    int errno_;
    int (*func)(FILE *, const fpos_t *);
};

struct ftell_event {
    const void *pc;
    FILE *stream;
    long ret;
    int errno_;
    long (*func)(FILE *);
};

struct fwrite_event {
    const void *pc;
    const void *ptr;
    size_t size;
    size_t n;
    FILE *stream;
    size_t ret;
    int errno_;
    size_t (*func)(const void *, size_t, size_t, FILE *);
};

struct getc_event {
    const void *pc;
    FILE *stream;
    int ret;
    int errno_;
    int (*func)(FILE *);
};

struct getchar_event {
    const void *pc;
    int ret;
    int errno_;
    int (*func)(void);
};

struct perror_event {
    const void *pc;
    const char *s;
    int errno_;
    void (*func)(const char *);
};

struct putc_event {
    const void *pc;
    int c;
    FILE *stream;
    int ret;
    int errno_;
    int (*func)(int, FILE *);
};

struct putchar_event {
    const void *pc;
    int c;
    int ret;
    int errno_;
    int (*func)(int);
};

struct puts_event {
    const void *pc;
    const char *s;
    int ret;
    int errno_;
    int (*func)(const char *);
};

struct remove_event {
    const void *pc;
    const char *path;
    int ret;
    int errno_;
    int (*func)(const char *);
};

struct rename_event {
    const void *pc;
    const char *oldpath;
    const char *newpath;
    int ret;
    int errno_;
    int (*func)(const char *, const char *);
};

struct rewind_event {
    const void *pc;
    FILE *stream;
    int errno_;
    void (*func)(FILE *);
};

struct setbuf_event {
    const void *pc;
    FILE *stream;
    char *buf;
    int errno_;
    void (*func)(FILE *, char *);
};

struct setvbuf_event {
    const void *pc;
    FILE *stream;
    char *buf;
    int mode;
    size_t size;
    int ret;
    int errno_;
    int (*func)(FILE *, char *, int, size_t);
};

struct tmpfile_event {
    const void *pc;
    FILE *ret;
    int errno_;
    FILE *(*func)(void);
};

struct tmpnam_event {
    const void *pc;
    char *s;
    char *ret;
    int errno_;
    char *(*func)(char *);
};

struct ungetc_event {
    const void *pc;
    int c;
    FILE *stream;
    int ret;
    int errno_;
    int (*func)(int, FILE *);
};

struct vfprintf_event {
    const void *pc;
    FILE *stream;
    const char *format;
    va_list ap;
    int ret;
    int errno_;
    int (*func)(FILE *, const char *, va_list);
};

struct vfscanf_event {
    const void *pc;
    FILE *stream;
    const char *format;
    va_list ap;
    int ret;
    int errno_;
    int (*func)(FILE *, const char *, va_list);
};

struct vprintf_event {
    const void *pc;
    const char *format;
    va_list ap;
    int ret;
    int errno_;
    int (*func)(const char *, va_list);
};

struct vscanf_event {
    const void *pc;
    const char *format;
    va_list ap;
    int ret;
    int errno_;
    int (*func)(const char *, va_list);
};

struct vsnprintf_event {
    const void *pc;
    char *str;
    size_t size;
    const char *format;
    va_list ap;
    int ret;
    int errno_;
    int (*func)(char *, size_t, const char *, va_list);
};

struct vsprintf_event {
    const void *pc;
    char *str;
    const char *format;
    va_list ap;
    int ret;
    int errno_;
    int (*func)(char *, const char *, va_list);
};

struct vsscanf_event {
    const void *pc;
    const char *str;
    const char *format;
    va_list ap;
    int ret;
    int errno_;
    int (*func)(const char *, const char *, va_list);
};

#endif /* DICE_STDIO_H */
