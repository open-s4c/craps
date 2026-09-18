set(EVENTS_INCLUDE_DIR
    "${CMAKE_CURRENT_LIST_DIR}/include")

set(EVENTS_MODULE_DIR
    "${CMAKE_CURRENT_LIST_DIR}/src/mod")


# more-events changed DICE_MAX_TYPES from 128 to 280.
#
# We deliberately do not force that from here because dice.h has already
# been configured by the time this directory is entered. The caller needs
# to configure Dice with -DDICE_MAX_TYPES=280.
if(DICE_MAX_TYPES LESS 280)
  message(
    FATAL_ERROR
    "The modules require DICE_MAX_TYPES >= 280. "
    "Configure Dice with -DDICE_MAX_TYPES=280.")
endif()


function(add_event_module MODULE)
  dice_add_interpose_module(
    ${MODULE}
    SOURCES
      "${EVENTS_MODULE_DIR}/${MODULE}.c"
    INCLUDE_DIRECTORIES
      "${EVENTS_INCLUDE_DIR}")
endfunction()


add_event_module(dirent)
add_event_module(fcntl)
add_event_module(poll)
add_event_module(pthread_once)
add_event_module(socket)
add_event_module(time)
add_event_module(uio)
add_event_module(unistd)


# more-events excluded these two on Apple.
if(NOT APPLE)
  add_event_module(eventfd)
  add_event_module(random)
endif()


# more-events excluded epoll on Apple/BSD.
if(NOT APPLE AND NOT BSD)
  add_event_module(epoll)
endif()


# syscall is special: syscall.c is only the before/after support code.
# syscall.S contains the actual interposed syscall() symbol.
#
# syscall.S itself explicitly supports Linux x86_64/aarch64 only.
if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
  dice_add_interpose_module(
    syscall
    SOURCES
      "${EVENTS_MODULE_DIR}/syscall.c"
      "${EVENTS_MODULE_DIR}/syscall.S"
    INCLUDE_DIRECTORIES
      "${EVENTS_INCLUDE_DIR}"
    COMPILE_OPTIONS
      -fPIC)
endif()


# Install the externally owned event headers into the same include hierarchy
# as Dice's built-in event headers.
install(
  DIRECTORY
    "${EVENTS_INCLUDE_DIR}/dice"
  DESTINATION
    include)

# ------------------------------------------------------------------------------
# Tests
# ------------------------------------------------------------------------------

if(DICE_TESTS)
  include(CheckSymbolExists)

  # --------------------------------------------------------------------------
  # Platform-specific interfaces used by the extracted interpose tests
  # --------------------------------------------------------------------------

  check_symbol_exists(epoll_pwait2 "sys/epoll.h" HAVE_EPOLL_PWAIT2)
  check_symbol_exists(getdate "time.h" HAVE_GETDATE)
  check_symbol_exists(getdate_r "time.h" HAVE_GETDATE_R)
  check_symbol_exists(settimeofday "sys/time.h" HAVE_SETTIMEOFDAY)
  check_symbol_exists(gethostid "unistd.h" HAVE_GETHOSTID)
  check_symbol_exists(sethostid "unistd.h" HAVE_SETHOSTID)
  check_symbol_exists(getpass "unistd.h" HAVE_GETPASS)
  check_symbol_exists(getwd "unistd.h" HAVE_GETWD)
  check_symbol_exists(
    get_current_dir_name
    "unistd.h"
    HAVE_GET_CURRENT_DIR_NAME)
  check_symbol_exists(cuserid "stdio.h" HAVE_CUSERID)
  check_symbol_exists(ualarm "unistd.h" HAVE_UALARM)

  set(
    MORE_EVENTS_TEST_SIGNATURES
    "${CMAKE_CURRENT_LIST_DIR}/test/interpose/interposed_.h.in")

  set(
    MORE_EVENTS_TEST_INCLUDE_DIR
    "${CMAKE_CURRENT_LIST_DIR}/include")

  # Small wrapper around Dice's generic test generator.
  #
  # The old more-events CMake linked libcrypt to every interpose test on
  # non-Apple systems and suppressed deprecation warnings for every generated
  # test. Preserve that behavior here.
  function(more_events_add_interpose_test MODULE HEADER)
    set(EXTRA_ARGS
        MAPPINGS
          "${MORE_EVENTS_TEST_SIGNATURES}"
        INCLUDE_DIRECTORIES
          "${MORE_EVENTS_TEST_INCLUDE_DIR}"
        COMPILE_OPTIONS
          -Wno-deprecated-declarations)

    if(NOT APPLE)
      list(
        APPEND
        EXTRA_ARGS
        LINK_LIBRARIES
          crypt)
    endif()

    dice_add_interpose_test(
      ${MODULE}
      HEADER "${HEADER}"
      FUNCTIONS ${ARGN}
      ${EXTRA_ARGS})
  endfunction()


  # --------------------------------------------------------------------------
  # random
  # --------------------------------------------------------------------------

  if(NOT APPLE)
    more_events_add_interpose_test(
      random
      "<dice/events/random.h>"
      getrandom)
  endif()


  # --------------------------------------------------------------------------
  # poll
  # --------------------------------------------------------------------------

  if(NOT (
      CMAKE_C_COMPILER_ID STREQUAL "GNU"
      AND CMAKE_BUILD_TYPE STREQUAL "Release"))
    more_events_add_interpose_test(
      poll
      "<dice/events/poll.h>"
      poll)
  endif()


  # --------------------------------------------------------------------------
  # epoll
  # --------------------------------------------------------------------------

  if(NOT APPLE AND NOT BSD)
    set(
      EPOLL_FUNCTIONS
      epoll_create
      epoll_create1
      epoll_ctl
      epoll_wait
      epoll_pwait)

    if(HAVE_EPOLL_PWAIT2 AND NOT MUSL)
      list(APPEND EPOLL_FUNCTIONS epoll_pwait2)
    endif()

    more_events_add_interpose_test(
      epoll
      "<dice/events/epoll.h>"
      ${EPOLL_FUNCTIONS})
  endif()


  # --------------------------------------------------------------------------
  # eventfd
  # --------------------------------------------------------------------------

  if(NOT APPLE)
    more_events_add_interpose_test(
      eventfd
      "<dice/events/eventfd.h>"
      eventfd)
  endif()


  # --------------------------------------------------------------------------
  # fcntl.h
  # --------------------------------------------------------------------------

  if(NOT APPLE)
    set(
      FCNTL_FUNCTIONS
      creat
      # fcntl
      # openat
      posix_fadvise
      posix_fallocate)

    if(NOT (
        CMAKE_C_COMPILER_ID STREQUAL "GNU"
        AND CMAKE_BUILD_TYPE STREQUAL "Release"))
      list(APPEND FCNTL_FUNCTIONS open)
    endif()

    more_events_add_interpose_test(
      fcntl
      "<dice/events/fcntl.h>"
      ${FCNTL_FUNCTIONS})
  endif()


  # --------------------------------------------------------------------------
  # socket
  # --------------------------------------------------------------------------

  set(
    SOCKET_FUNCTIONS
    accept
    bind
    connect
    getpeername
    getsockname
    getsockopt
    setsockopt
    listen
    recv
    recvfrom
    recvmsg
    send
    sendto
    sendmsg
    shutdown
    socket
    socketpair)

  if(NOT APPLE)
    list(APPEND SOCKET_FUNCTIONS accept4)
  endif()

  more_events_add_interpose_test(
    socket
    "<dice/events/socket.h>"
    ${SOCKET_FUNCTIONS})


  # --------------------------------------------------------------------------
  # time.h
  # --------------------------------------------------------------------------

  if(NOT (
      CMAKE_C_COMPILER_ID STREQUAL "Clang"
      AND CMAKE_BUILD_TYPE STREQUAL "Release")
     AND NOT BSD)

    set(
      TIME_FUNCTIONS
      asctime
      asctime_r
      clock
      time
      clock_getres
      clock_gettime
      clock_settime
      ctime
      ctime_r
      difftime
      gmtime
      gmtime_r
      localtime
      localtime_r
      mktime
      nanosleep
      strftime
      strptime
      getitimer
      setitimer
      gettimeofday
      select
      tzset
      utimes)

    if(HAVE_GETDATE AND NOT BSD)
      list(APPEND TIME_FUNCTIONS getdate)
    endif()

    if(HAVE_SETTIMEOFDAY AND NOT BSD)
      list(APPEND TIME_FUNCTIONS settimeofday)
    endif()

    if(NOT APPLE)
      list(
        APPEND
        TIME_FUNCTIONS
        timer_create
        timer_delete
        timer_gettime
        timer_getoverrun
        timer_settime
        strftime_l)
    endif()

    if(HAVE_GETDATE_R)
      list(APPEND TIME_FUNCTIONS getdate_r)
    endif()

    more_events_add_interpose_test(
      time
      "<dice/events/time.h>"
      ${TIME_FUNCTIONS})
  endif()


  # --------------------------------------------------------------------------
  # uio.h
  # --------------------------------------------------------------------------

  if(NOT APPLE AND NOT BSD)
    more_events_add_interpose_test(
      uio
      "<dice/events/uio.h>"
      readv
      writev)
  endif()


  # --------------------------------------------------------------------------
  # unistd.h
  # --------------------------------------------------------------------------

  if(NOT CMAKE_BUILD_TYPE STREQUAL "Release")
    set(
      UNISTD_FUNCTIONS
      getdtablesize
      getegid
      geteuid
      write
      getgid
      getgroups
      getlogin
      getpagesize
      getpgrp
      getpid
      getppid
      getuid
      fork
      pause
      # pipe
      setsid
      sync
      access
      alarm
      chdir
      chown
      chroot
      close
      confstr
      ctermid
      dup
      dup2
      faccessat
      fchdir
      fchown
      fchownat
      fpathconf
      fsync
      ftruncate
      getcwd
      getlogin_r
      getopt
      getpgid
      getsid
      isatty
      lchown
      link
      linkat
      lockf
      lseek
      nice
      pathconf
      pread
      pwrite
      read
      readlink
      readlinkat
      rmdir
      setgid
      setpgid
      setregid
      setreuid
      setuid
      sleep
      swab
      symlink
      symlinkat
      sysconf
      tcgetpgrp
      tcsetpgrp
      truncate
      ttyname
      ttyname_r
      unlink
      unlinkat
      usleep)

    if(NOT APPLE)
      list(
        APPEND
        UNISTD_FUNCTIONS
        brk
        crypt
        fdatasync
        # pipe2
        sbrk
        vfork)
    endif()

    if(NOT APPLE AND NOT BSD)
      list(
        APPEND
        UNISTD_FUNCTIONS
        setpgrp
        syncfs
        dup3)
    endif()

    if(HAVE_GETHOSTID)
      list(APPEND UNISTD_FUNCTIONS gethostid)
    endif()

    if(HAVE_SETHOSTID AND NOT APPLE AND NOT BSD)
      list(APPEND UNISTD_FUNCTIONS sethostid)
    endif()

    if(HAVE_GETPASS)
      list(APPEND UNISTD_FUNCTIONS getpass)
    endif()

    if(HAVE_GETWD)
      list(APPEND UNISTD_FUNCTIONS getwd)
    endif()

    if(HAVE_GET_CURRENT_DIR_NAME)
      list(APPEND UNISTD_FUNCTIONS get_current_dir_name)
    endif()

    if(HAVE_CUSERID)
      list(APPEND UNISTD_FUNCTIONS cuserid)
    endif()

    if(HAVE_UALARM)
      list(APPEND UNISTD_FUNCTIONS ualarm)
    endif()

    more_events_add_interpose_test(
      unistd
      "<dice/events/unistd.h>"
      ${UNISTD_FUNCTIONS})
  endif()
endif()
