# 8. Traits and Generics

## Trait Definitions

```xin
trait Printable {
    fn Format(Self) -> String;

    fn Print(Self) {
        IO.Println(Self.Format());
    }
}

trait Comparable<T> {
    fn Compare(Self, Other: T) -> i32;  // -1, 0, 1
}
```

## Implementing Traits

```xin
struct Point { X: i32, Y: i32 }

impl Printable for Point {
    fn Format(Self) -> String {
        f"Point({Self.X}, {Self.Y})"
    }
}
```

## Trait Bounds

```xin
fn PrintAll<T: Printable>(Items: []T) {
    for Item in Items {
        Item.Print();
    }
}

fn Process<T: Clone + Printable>(Item: T) {
    let Cloned = Item.Clone();
    Cloned.Print();
}

fn Complex<T, U>(A: T, B: U) -> i32
where
    T: Clone + Printable,
    U: Comparable<T>,
{
    // ...
}
```

## Composable Traits

```xin
trait Serializable: Printable + Clone {}

trait Iterator {
    type Item;

    fn Next(mut self) -> Option<Self.Item>;

    fn Collect<B: FromIterator<Self::Item>>(Self) -> B {
        let mut Iter = Self;
        Iter.FromFn(|| Iter.Next()).Collect()
    }
}
```

## Generic Functions

```xin
fn First<T>(Slice: []T) -> Option<T> {
    Slice.First()
}

fn Pair<T, U>(T: T, U: U) -> (T, U) {
    (T, U)
}
```
