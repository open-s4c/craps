# Copyright (C) 2026 Huawei Technologies Co., Ltd.
# SPDX-License-Identifier: 0BSD
#
# Run the original generated interposition tests against Dice Roll.
# Each module receives its own configured shared library, matching the test
# isolation formerly provided by dice-<module> preload libraries.

include_guard(GLOBAL)
include(CMakeParseArguments)

if(NOT TARGET craps-roll-interpose-tests)
  add_custom_target(craps-roll-interpose-tests)
endif()

function(craps_roll_add_interpose_test MODULE)
  cmake_parse_arguments(
    TEST
    ""
    "HEADER"
    "FUNCTIONS;MAPPINGS;INCLUDE_DIRECTORIES;LINK_LIBRARIES;COMPILE_OPTIONS"
    ${ARGN})

  if(TEST_UNPARSED_ARGUMENTS OR TEST_KEYWORDS_MISSING_VALUES)
    message(FATAL_ERROR
      "craps_roll_add_interpose_test(${MODULE}): invalid arguments")
  endif()
  if(NOT TEST_HEADER OR NOT TEST_FUNCTIONS)
    message(FATAL_ERROR
      "craps_roll_add_interpose_test(${MODULE}): HEADER and FUNCTIONS are required")
  endif()
  if(NOT TARGET "dice-${MODULE}.o")
    message(FATAL_ERROR
      "craps_roll_add_interpose_test(${MODULE}): missing Roll object catalog")
  endif()
  if(NOT TARGET tmplr)
    message(FATAL_ERROR "Craps interpose tests require Dice tmplr")
  endif()

  set(runtime "craps-roll-test-${MODULE}")
  set(target "craps_roll_${MODULE}_test")
  set(test_root "${PROJECT_SOURCE_DIR}/test")
  set(templates "${test_root}/interpose")
  set(generated "${CMAKE_CURRENT_BINARY_DIR}/${target}.c")

  # Never link against legacy libdice or per-module preload libraries.
  # Each test uses a generated Roll runtime containing only its interceptor.
  add_library("${runtime}" SHARED)
  set(roll_imports)
  if(DICE_ROLL_IMPORT_PATHS)
    list(APPEND roll_imports IMPORT_PATHS ${DICE_ROLL_IMPORT_PATHS})
  endif()
  add_dice_core("${runtime}" "${DICE_ROLL_CONFIG}" ${roll_imports})
  add_dice_mods("${runtime}" "${MODULE}")
  get_target_property(header "${runtime}" DICE_ROLL_HEADER)

  string(REPLACE ";" "\\;" escaped_functions "${TEST_FUNCTIONS}")
  set(mappings "${templates}/interposed_.h.in")
  list(APPEND mappings ${TEST_MAPPINGS})
  add_custom_command(
    OUTPUT "${generated}"
    COMMAND $<TARGET_FILE:tmplr>
      -l 2048
      "-DINCL='${TEST_HEADER}'"
      "-DFOO=${escaped_functions}"
      ${mappings}
      "${templates}/interpose_test.c.in"
      > "${target}_.c.tmp"
    COMMAND "${CMAKE_COMMAND}" -E rename "${target}_.c.tmp" "${target}_.c"
    COMMAND $<TARGET_FILE:tmplr>
      -l 2048
      -sP\\$$_
      "-DFOO=${escaped_functions}"
      "${templates}/interposed__.h.in"
      "${target}_.c"
      > "${target}.c.tmp"
    COMMAND "${CMAKE_COMMAND}" -E rename "${target}.c.tmp" "${target}.c"
    WORKING_DIRECTORY "${CMAKE_CURRENT_BINARY_DIR}"
    DEPENDS
      tmplr
      "${templates}/interpose_test.c.in"
      "${templates}/interposed_.h.in"
      "${templates}/interposed__.h.in"
      ${TEST_MAPPINGS})

  add_executable("${target}" "${generated}")
  set_source_files_properties("${generated}" PROPERTIES GENERATED TRUE)
  add_dependencies("${target}" "${runtime}-dice-generate")
  target_link_libraries("${target}" PRIVATE
    "${runtime}-dice.h" "${runtime}" tsano Threads::Threads
    ${TEST_LINK_LIBRARIES})
  target_include_directories("${target}" PRIVATE
    "${test_root}/include" ${TEST_INCLUDE_DIRECTORIES})
  # The extension headers deliberately contain NO EVENT_* constants.
  # Both the interceptor and its test must see the same generated dice.h.
  target_compile_options("${target}" PRIVATE
    "-include" "${header}"
    -fno-builtin-memcpy -fno-builtin-memset -fno-builtin-memmove
    ${TEST_COMPILE_OPTIONS})
  target_compile_definitions("${target}" PRIVATE VATOMIC_BUILTINS)

  if(ENABLE_SANITIZER)
    target_compile_options("${target}" PRIVATE ${LIBSAN_C_FLAGS})
    target_link_options("${target}" PRIVATE ${LIBSAN_LD_FLAGS})
    target_link_libraries("${target}" PRIVATE ${LIBSAN_LINK})
  endif()

  # Use the same Dice preload wrapper as the old interpose tests, now with
  # the generated runtime instead of the legacy dice-<module> library.
  add_test(NAME "craps-roll-interpose-${MODULE}"
    COMMAND ${DICE_SCRIPT}
      -preload "$<TARGET_FILE:${runtime}>"
      "$<TARGET_FILE:${target}>")
  set_tests_properties("craps-roll-interpose-${MODULE}" PROPERTIES TIMEOUT 90)
  add_dependencies(craps-roll-interpose-tests "${target}")
endfunction()
