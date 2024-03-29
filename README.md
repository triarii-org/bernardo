# Bernardo

[![ci_status](https://github.com/triarii-org/bernardo/actions/workflows/ci.yml/badge.svg)](https://github.com/triarii-org/bernardo/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/triarii-org/bernardo/graph/badge.svg)](https://codecov.io/gh/triarii-org/bernardo)

### Introduction

This is a repository currently hosting two projects: Gladius and Bernardo.
Bernardo is a TUI widget library.
Gladius is a code editor.
Currently they are developed in single repo, because separating them at this time would hinder the effort.

Furthermore *both projects are in alpha state*, they *will* crash, don't use them for anything important yet.

Licenses: GPLv3. I might re-release Bernardo as LGPL at later date, but that has not been decided yet. If you decide
to contribute, be sure you are OK with that.

Here is a website describing reasoning behind the projects: https://njskalski.gitlab.io/triarii

Here's what editor can currently do:
- read, write
- syntax highlighting (done via tree-sitter, themes from syntect crate)
- undo/redo (ctrl-z, ctrl-x <-! sorry, shift+ctrl doesn't work well with cmd-line)
- copy/paste (ctrl-c, ctrl-v)
- fuzzy jump to file or directory (ctrl-h)
- fancy save-file-as dialog (mostly to test if widgets work correctly)
- Multicursor: ctrl-w switches to "drop cursor" mode, where you navigate with arrows and can add/remove cursors with
    Enter. Esc exits this mode.

Inspiration for name:

https://en.wikipedia.org/wiki/Bernard_Gui

Bernardo Gui, an inquisitor, historian and for a period of time, Bishop of Tui :D. Portrayed as violent, unfair man in
"The Name of the Rose" novel by Umberto Eco. Some sources claim that this image is overly negative. I consider using
mouse for programming a heresy, so I figured I need a help from professional inquisition that no one expects.

### Run

```
git submodule update --init
cargo run
```
