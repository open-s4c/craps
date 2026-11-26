/*
 * Copyright (C) 2026 Huawei Technologies Co., Ltd.
 * SPDX-License-Identifier: 0BSD
 */
#ifndef DICE_TIME_H
#define DICE_TIME_H

#include <time.h>
#include <sys/time.h>
#ifdef __APPLE__
#include <xlocale.h>
typedef clockid_t timer_t;
#endif


struct asctime_event {
    const void *pc;
    const struct tm *timeptr;
    char *ret;
    char *(*func)(const struct tm *);
};

struct asctime_r_event {
    const void *pc;
    const struct tm *tm;
    char *buf;
    char *ret;
    char *(*func)(const struct tm *, char *);
};

struct clock_event {
    const void *pc;
    clock_t ret;
    clock_t (*func)(void);
};

struct clock_getres_event {
    const void *pc;
    clockid_t clockid;
    struct timespec *res;
    int ret;
    int (*func)(clockid_t, struct timespec *);
};

struct clock_gettime_event {
    const void *pc;
    clockid_t clockid;
    struct timespec *tp;
    int ret;
    int (*func)(clockid_t, struct timespec *);
};

struct clock_settime_event {
    const void *pc;
    clockid_t clockid;
    const struct timespec *tp;
    int ret;
    int (*func)(clockid_t, const struct timespec *);
};

struct ctime_event {
    const void *pc;
    const time_t *timep;
    char *ret;
    char *(*func)(const time_t *);
};

struct ctime_r_event {
    const void *pc;
    const time_t *timep;
    char *buf;
    char *ret;
    char *(*func)(const time_t *, char buf[]);
};

struct difftime_event {
    const void *pc;
    time_t time1;
    time_t time0;
    double ret;
    double (*func)(time_t, time_t);
};

struct getdate_event {
    const void *pc;
    const char *string;
    struct tm *ret;
    struct tm *(*func)(const char *);
};

struct getdate_r_event {
    const void *pc;
    const char *string;
    struct tm *res;
    int ret;
    int (*func)(const char *, struct tm *);
};

struct gmtime_event {
    const void *pc;
    const time_t *timep;
    struct tm *ret;
    struct tm *(*func)(const time_t *);
};

struct gmtime_r_event {
    const void *pc;
    const time_t *timep;
    struct tm *result;
    struct tm *ret;
    struct tm *(*func)(const time_t *, struct tm *);
};

struct localtime_event {
    const void *pc;
    const time_t *timep;
    struct tm *ret;
    struct tm *(*func)(const time_t *);
};

struct localtime_r_event {
    const void *pc;
    const time_t *timep;
    struct tm *result;
    struct tm *ret;
    struct tm *(*func)(const time_t *, struct tm *);
};

struct mktime_event {
    const void *pc;
    struct tm *tm;
    time_t ret;
    time_t (*func)(struct tm *);
};

struct nanosleep_event {
    const void *pc;
    const struct timespec *duration;
    struct timespec *rem;
    int ret;
    int (*func)(const struct timespec *, struct timespec *);
};

struct strftime_event {
    const void *pc;
    char *s;
    size_t max;
    const char *format;
    const struct tm *tm;
    size_t ret;
    size_t (*func)(char *, size_t, const char *, const struct tm *);
};

struct strftime_l_event {
    const void *pc;
    char *s;
    size_t max;
    const char *format;
    const struct tm *tm;
    locale_t locale;
    size_t ret;
    size_t (*func)(char *, size_t, const char *, const struct tm *, locale_t);
};

struct strptime_event {
    const void *pc;
    const char *s;
    const char *format;
    struct tm *tm;
    char *ret;
    char *(*func)(const char *, const char *, struct tm *);
};

struct time_event {
    const void *pc;
    time_t *tloc;
    time_t ret;
    time_t (*func)(time_t *);
};

struct timer_create_event {
    const void *pc;
    clockid_t clockid;
    struct sigevent *sevp;
    timer_t *timerid;
    int ret;
    int (*func)(clockid_t, struct sigevent *, timer_t *);
};

struct timer_delete_event {
    const void *pc;
    timer_t timerid;
    int ret;
    int (*func)(timer_t);
};

struct timer_gettime_event {
    const void *pc;
    timer_t timerid;
    struct itimerspec *curr_value;
    int ret;
    int (*func)(timer_t, struct itimerspec *);
};

struct timer_getoverrun_event {
    const void *pc;
    timer_t timerid;
    int ret;
    int (*func)(timer_t);
};

struct timer_settime_event {
    const void *pc;
    timer_t timerid;
    int flags;
    const struct itimerspec *new_value;
    struct itimerspec *old_value;
    int ret;
    int (*func)(timer_t, int, const struct itimerspec *, struct itimerspec *);
};

struct tzset_event {
    const void *pc;
    void (*func)(void);
};

#ifndef __GLIBC__
typedef int __itimer_which_t;
#endif

struct getitimer_event {
    const void *pc;
    __itimer_which_t which;
    struct itimerval *curr_value;
    int ret;
    int (*func)(__itimer_which_t, struct itimerval *);
};

struct setitimer_event {
    const void *pc;
    __itimer_which_t which;
    const struct itimerval *new_value;
    struct itimerval *old_value;
    int ret;
    int (*func)(__itimer_which_t, const struct itimerval *, struct itimerval *);
};

struct gettimeofday_event {
    const void *pc;
    struct timeval *tv;
    void *tz;
    int ret;
    int (*func)(struct timeval *, void *);
};

struct settimeofday_event {
    const void *pc;
    const struct timeval *tv;
    const struct timezone *tz;
    int ret;
    int (*func)(const struct timeval *, const struct timezone *);
};

struct select_event {
    const void *pc;
    int nfds;
    fd_set *readfds;
    fd_set *writefds;
    fd_set *exceptfds;
    struct timeval *timeout;
    int ret;
    int (*func)(int, fd_set *, fd_set *, fd_set *, struct timeval *);
};

struct utimes_event {
    const void *pc;
    const char *path;
    const struct timeval *times;
    int ret;
    int (*func)(const char *, const struct timeval [2]);
};

#endif /* DICE_TIME_H */
