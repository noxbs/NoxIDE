# Noxide

Noxide is a small D command-line diagnostic project built with Nox. It is a real integration project for the D Rider: it has its own version, project metadata, source, installable executable, and runtime arguments.

Build it from this directory with Nox:

```sh
nox setup build
nox build build
nox run noxide hello from Nox
```

The executable is produced at `build/debug/noxide/noxide`. To install it under `/usr/local/bin`:

```sh
nox install
```

Noxide intentionally has no Cargo or D package-manager metadata. Nox owns the build graph and invokes the D compiler directly.

The D Rider looks for one of these compilers on `PATH`:

- `ldc2`
- `dmd`
- `gdc`
