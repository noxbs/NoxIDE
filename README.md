# NoxIDE

NoxIDE is a focused Rust code editor built with eframe. Nox is both its build system and
the project workflow it integrates with: NoxIDE invokes `nox` for configure,
build, rebuild, clean, and run operations instead of implementing a second
build graph.

## Current foundation

NoxIDE launches as a graphical desktop application, not a terminal menu. The
current window includes:

- a native eframe desktop window with a project pane and output pane;
- a real editable egui text editor with tabs, scrolling, selection, undo, and
  clipboard support;
- a line-number gutter and file-backed document buffers;
- graphical Open and Save actions using native file dialogs; and
- a Build action that invokes Nox and displays its output in the application.

The GUI consumes separate services rather than owning their logic:

- `document` owns file-backed document state and saving;
- `workspace` discovers a project root from `nox.build` or `noxfile`;
- `noxide::nox` invokes Nox in the discovered workspace and preserves its output
  and exit status.

eframe, egui, and rfd are resolved through Cargo in the Nix development
environment. Syntax-aware editing, project tree operations, LSP services, and
richer Nox diagnostics are the next layers on top of this GUI foundation.

## Build with Nox

NoxIDE requires Rust and Cargo. Nox owns the project target and delegates Rust
dependency resolution and compilation to Cargo.

```sh
nox setup build
nox build build
```

Launch the graphical application from the repository root:

```sh
build/debug/noxide/noxide
```

Optional file arguments are passed to the graphical launcher and opened as
editor tabs:

```sh
build/debug/noxide/noxide src/main.rs
```

The repository uses Cargo for Rust dependencies, but Nox remains the primary
project and build workflow.

## Project direction

NoxIDE will remain a small editor rather than a general-purpose IDE. Planned
editor layers are syntax highlighting, search and navigation, LSP
diagnostics/completion/hover, file watching, project tree operations, and
clickable build diagnostics backed by `NoxRunner`.
