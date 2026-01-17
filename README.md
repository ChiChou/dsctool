# Dyld Shared Cache Utilities

A command-line utility for inspecting Dyld Shared Caches.

## Build

```bash
cargo build --release
```

## Usage

### List Images

List all images contained in the dyld shared cache:

```bash
./dsc images <path-to-dyld-cache>
```

### List Sections

Display sections for images in the cache, optionally filtered by module:

```bash
./dsc sections <path-to-dyld-cache> [--module <module-name>]
```

### List Symbols

Display symbols for images in the cache, optionally filtered by module:

```bash
./dsc symbols <path-to-dyld-cache> [--module <module-name>]
```

### Dump Contents

Dump memory at a specific virtual address:

```bash
./dsc dump <path-to-dyld-cache> <address> [size]
```

- `address` can be in decimal or hexadecimal (prefix with `0x`)
- `size` defaults to 256 bytes if not specified

## Examples

```bash
# List all images
./dsc images dyld_shared_cache_arm64e

# List sections for specific module
./dsc sections dyld_shared_cache_arm64e --module /usr/lib/libSystem.B.dylib

# List symbols for specific module  
./dsc symbols dyld_shared_cache_arm64e --module /usr/lib/libSystem.B.dylib

# Dump memory at address
./dsc dump dyld_shared_cache_arm64e 0x180000000
./dsc dump dyld_shared_cache_arm64e 0x180000000 512
./dsc dump dyld_shared_cache_arm64e 4294967296
```
