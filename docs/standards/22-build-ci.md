# 23. Build and CI

## Build Requirements

```bash
# All platforms must build successfully
make                     # Build using Makefile (default)
make debug               # Debug build
make release             # Optimized release build
make test                # Run test suite
make install             # Install to /usr/local (or DESTDIR)
make clean               # Clean build artifacts
make docs                # Generate documentation

# CMake alternative (if using CMakeLists.txt)
mkdir build && cd build
cmake -DCMAKE_BUILD_TYPE=Release ..
make -j$(nproc)
make test
```

Build system tools required:
- C compiler: GCC 10+, Clang 12+, or MSVC 2019+
- LLVM 15+ (for LLVM IR generation)
- Make or Ninja
- CMake 3.20+ (optional)
- Git (for dependency fetching)

## Artifact Requirements

| Artifact | Requirement |
|----------|-------------|
| Executable | Statically linked where possible (use -static flag) |
| Debug info | Included in debug builds, stripped in release |
| Dependencies | Minimal, only libc and XGC runtime; audit for vulnerabilities |
| Source | Must match binary (reproducible builds encouraged) |
| Binaries | ELF (Linux), Mach-O (macOS), PE (Windows), WASM (web) |

## Development Commands

```bash
make                    # Build debug
make release            # Build release
make test               # Run tests
make check              # Type check only (fast)
make format             # Format source files (using xinc --format)
make lint               # Run linter (xinc --lint)
make clean              # Clean build
make docs               # Generate docs
make bench              # Run benchmarks
make install            # Install to system
```

## CI Pipeline

```yaml
stages:
  - style
  - static-analysis
  - test
  - security
  - performance
  - build
  - package
  - deploy
```

## Required CI Checks

| Check | Timeout | Fail on Warning |
|-------|---------|-----------------|
| Format check | 2 min | Yes |
| Lint check | 5 min | Yes |
| Type check | 5 min | Yes |
| Build (debug) | 10 min | Yes |
| Unit tests | 10 min | Yes |
| Integration tests | 20 min | Yes |
| Security audit | 10 min | Yes |
| Performance bench | 15 min | No (track regressions) |
| Doc build | 5 min | Yes |
| Release build | 30 min | Yes |
| Cross-compile (x86_64, aarch64) | 30 min | Yes |

## Branch Protection

| Branch | Requirements |
|--------|--------------|
| main | Admin only, all CI checks must pass |
| release | PR required, all CI checks must pass, version bump |
| develop | CI checks must pass |
| feature/* | CI checks must pass before merge to develop |

## Quality Gates

All code must pass through quality gates:

```
+-------------+    +--------------+    +-------------+    +-------------+
|   Commit    |--->|   Build      |--->|   Tests    |--->|   Review   |
+-------------+    +--------------+    +-------------+    +-------------+
      |                  |                  |                  |
      v                  v                  v                  v
 Code Style        Compilation        Coverage >=80%      Approval
 Check (100%)     Success (100%)     Pass               Required
                   TypeCheck Pass
```

## Cross-Platform Builds

CI must test on all supported platforms:
- Linux (Ubuntu 22.04+, Alpine for musl)
- macOS (12+)
- Windows (MSVC, MinGW)
- WASM (browser, WASI runtime)
- FreeBSD (optional)

Using matrix builds in CI:

```yaml
matrix:
  include:
    - os: linux
      compiler: gcc
    - os: linux
      compiler: clang
    - os: macos
      compiler: clang
    - os: windows
      compiler: msvc
```
