# Dyld Shared Cache Utilities

A command-line utility for inspecting Dyld Shared Caches.

## Build

```bash
cargo build --release
```

## Usage

### List Images

```bash
./dsc images <path-to-dyld-cache>
```

### List Sections

```bash
./dsc sections <path-to-dyld-cache>
```

### Dump Contents

```bash
./dsc dump <path-to-dyld-cache> <address> [size]
```

- `address` can be in decimal or hexadecimal (prefix with `0x`)
- `size` defaults to 256 bytes if not specified

Examples:

```bash
./target/release/dsc dump /System/Library/dyld/dyld_shared_cache_arm64e 0x180000000
./target/release/dsc dump /System/Library/dyld/dyld_shared_cache_arm64e 0x180000000 512
./target/release/dsc dump /System/Library/dyld/dyld_shared_cache_arm64e 4294967296
```
