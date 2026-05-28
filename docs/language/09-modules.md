# 10. Modules and Visibility

## Module System

```xin
mod Utils {
    pub fn Helper() { }

    mod Internal {
        pub(super) fn InternalHelper() { }
    }
}

use Utils.Helper;
```

## Visibility Modifiers

| Modifier | Description |
|----------|-------------|
| `pub` | Public - accessible everywhere |
| `pub(crate)` | Visible within current crate |
| `pub(super)` | Visible within parent module |
| `pub(in path)` | Visible within specific path |
| (none) | Private - visible within parent module |

## Imports

```xin
use std.Collections.HashMap;
use std.IO.{Self, Read};
use FS.File as StdFile;

pub use std.Collections.HashMap;

use {
    IO::{Self, Write},
    FS,
};
```
