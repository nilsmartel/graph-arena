# Graph Arena

Collection type to store immutable graph data efficiently.
Avoids heap clusturing, enforces locality and reduces the need for copying all together.

## When should I use this?

In recursive data structures, one often encounters code like this:

```rust
struct Expression {
    Add(Box<Expression>, Box<Expression>),
    // ...
}
```

In the long run one encounters several problems with this approach:
- Most data lives on the heap, but related data aren't necessarily near each other.
- A lot of allocations are required to build these structures.
- Cloning becomes really expensive.

Now, allocations are something we'd like to avoid as much as possible;
`alloc` is an expensive operation.
That's why a `Vec<T>` allocates more memory, than it (might) use. So no reallocation of data is required each time we push more data.

Furthermore we desire related data to live in near by memory locations: Modern CPUs are heavily modified to take advantage of locality, that makes `Vec<T>` an insanely fast data strucutre!


