# 12. Standard Library

## Core Types

- `Option<T>` - Nullable values
- `Result<T, E>` - Error handling
- `Vec<T>` - Dynamic array (similar to C++ vector)
- `Box<T>` - Heap allocation (owned pointer)
- `Rc<T>` - Reference counting (non-atomic)
- `Arc<T>` - Atomic reference counting (thread-safe)
- `Cell<T>` / `RefCell<T>` - Interior mutability (runtime borrow checking)
- `Ptr<T>` - Raw pointer (unsafe, for FFI)
- `Slice<T>` - View into contiguous sequence

## Collections

```xin
use std.Collections.{Vec, HashMap, HashSet, BTreeMap, BTreeSet, LinkedList, VecDeque};

// Vec - growable array
let MutVec: Vec<i32> = Vec.New();
MutVec.Push(1);
MutVec.Push(2);
let Item = MutVec[0];  // Bounds-checked

// HashMap - hash map
let mut Map: HashMap<String, i32> = HashMap.New();
Map.Insert("key".Into(), 42);
let Value = Map.Get(&"key".Into());

// HashSet - hash set
let mut Set: HashSet<i32> = HashSet.New();
Set.Insert(1);
Set.Insert(2);
let Contains = Set.Contains(&1);
```

## I/O

```xin
use std.IO.{Self, Read, Write, Seek, BufRead, BufWriter};
use std.FS.{Self, File, OpenOptions, Metadata};

// File operations
let mut File = FS.File.Open("input.txt", FS.OpenOptions.Read)?;
let mut Contents = String.New();
File.ReadToString(&mut Contents)?;

// Writing
let mut OutFile = FS.File.Create("output.txt")?;
OutFile.WriteAll(b"Hello, World!")?;

// Standard I/O
IO.Println("Hello, world!");
let Input = IO.ReadLine()?;

// Buffered I/O for performance
let mut BufReader = IO.BufReader.New(File);
let mut Line = String.New();
while BufReader.ReadLine(&mut Line)? {
    // Process line
    Line.Clear();
}

// With refinement types for path safety
type ValidPath = { String | _.StartsWith('/') || _.Contains('\\') || _.EndsWith('.xin') };
fn ProcessFile(Path: ValidPath) -> Result<(), IOError> {
    let Content = FS.ReadToString(Path)?;
    // ...
    Ok(())
}
```

## Concurrency Primitives

```xin
use std.Sync.{Mutex, RwLock, Condvar, Arc, Barrier, Atomic};
use std.Channel.{Channel, Sender, Receiver};
use std.Fiber.{Fiber, Scope, Task};
use std.Thread.{Thread, JoinHandle};

// Mutex for shared mutable state
let Counter = Arc.New(Mutex.New(0));
let mut Handles = Vec.New();

for _ in 0..10 {
    let CounterClone = Arc.Clone(&Counter);
    let Handle = Thread.Spawn(|| {
        let mut Guard = CounterClone.Lock().Unwrap();
        *Guard += 1;
    });
    Handles.Push(Handle);
}

for H in Handles {
    H.Join().Unwrap();
}

// Channels for message passing
let (Tx, Rx) = Channel.New(100);  // Buffered channel, capacity 100

Spawn(async || {
    Tx.Send("message".Into()).Await?;
});

let Msg = Rx.Recv().Await?;

// Fiber-based lightweight concurrency
Fiber.Spawn(|| {
    // Fiber context
    DoWork();
});

// Atomic operations
let AtomicInt = Atomic.New(0);
AtomicInt.FetchAdd(1, Ordering.Relaxed);
```
