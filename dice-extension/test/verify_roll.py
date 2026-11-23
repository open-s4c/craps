#!/usr/bin/env python3
"""Verify that Craps' Dice Roll event declarations match C publications."""
from __future__ import annotations

import pathlib
import re
import sys

EXTENSION = pathlib.Path(__file__).resolve().parents[1]
SOURCE = EXTENSION / "src" / "mod"
CONFIG = EXTENSION / "roll"

PUBLISH = re.compile(
    r"\bPS_PUBLISH\s*\(\s*"
    r"(INTERCEPT_(?:EVENT|BEFORE|AFTER))\s*,\s*"
    r"(EVENT_[A-Z0-9_]+)\b",
    re.MULTILINE,
)
ROUTES = re.compile(
    r"\(\s*(CHAIN_INTERCEPT_(?:EVENT|BEFORE|AFTER))"
    r"\s+([^()]+)\)",
    re.MULTILINE,
)
IDS = re.compile(r"^\s*#\s*define\s+EVENT_[A-Z0-9_]+\b", re.MULTILINE)
EVENT = re.compile(r"\bEVENT_[A-Z0-9_]+\b")
COMMENTS = re.compile(r"/\*.*?\*/|//[^\n]*", re.DOTALL)

errors: list[str] = []
events: set[str] = set()
module_count = 0

for path in sorted((EXTENSION / "include" / "dice" / "events").glob("*.h")):
    if IDS.search(path.read_text()):
        errors.append(f"{path.name}: numeric/alias EVENT_ macros must come from Roll")

for source in sorted(SOURCE.glob("*.c")):
    name = source.stem
    manifest = CONFIG / f"{name}.dice"
    if not manifest.exists():
        errors.append(f"{name}: missing Roll module description")
        continue
    module_count += 1
    c_source = COMMENTS.sub("", source.read_text())
    declared = manifest.read_text()
    actual = {
        (f"CHAIN_{chain}", event)
        for chain, event in PUBLISH.findall(c_source)
    }
    expected = {
        (chain, event)
        for chain, symbols in ROUTES.findall(declared)
        for event in EVENT.findall(symbols)
    }
    for chain, event in sorted(actual - expected):
        errors.append(f"{name}: {chain}/{event} published by C, not declared")
    for chain, event in sorted(expected - actual):
        errors.append(f"{name}: {chain}/{event} declared but not published")
    events.update(event for _, event in expected)

top = (CONFIG / "craps.dice").read_text()
if "ids.dice" in top or re.search(r"\(\s*EVENT_[A-Z0-9_]+\s+\d+\s*\)", top):
    errors.append("craps.dice: pinned numeric event IDs are forbidden")
for source in sorted(SOURCE.glob("*.c")):
    if f"(import (dice {source.stem}))" not in top:
        errors.append(f"{source.stem}: not imported by craps.dice")

if len(events) < 200:
    errors.append(f"Expected >=200 independently described events, got {len(events)}")

if errors:
    for error in errors:
        print(error, file=sys.stderr)
    sys.exit(1)
print(f"Verified {module_count} Roll modules, {len(events)} event names, "
      "all C publications, and zero event macros in extension headers")
