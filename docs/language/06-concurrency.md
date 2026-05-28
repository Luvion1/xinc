# 7. Concurrency Model

## Structured Concurrency

All spawned tasks must complete or be explicitly scoped:

```xin
let Handle = Spawn(async || {
    ComputeHeavyWork();
});

let Result = Handle.Await();

scope(|S| {
    S.Spawn(|| TaskA());
    S.Spawn(|| TaskB());
});  // All tasks complete before scope returns
```

## Fibers

Lightweight concurrency units similar to goroutines:

```xin
let Fiber = Fiber.Spawn(|| {
    DoWork();
});

let (Tx, Rx) = Channel.New();

Spawn(async || {
    Tx.Send("hello").Unwrap();
});

let Msg = Rx.Recv().Unwrap();
```

## Async/Await

```xin
async fn FetchUrl(Url: String) -> Result<String, Error> {
    let Response = Http.Get(Url).Await?;
    let Body = Response.Text().Await?;
    Ok(Body)
}
```

## Synchronization

```xin
let Counter = Mutex.New(0);

Spawn(async || {
    let mut Guard = Counter.Lock().Unwrap();
    *Guard += 1;
});

let Shared = Arc.New(Mutex.New(Vec.New()));
let Clone = Arc.Clone(&Shared);
```
