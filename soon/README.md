# Soon<T>

A little rust lib for filling struct fields after creation.
You could use a `Refcell` or `RwLock`, but those have unnecessary overhead for this situation.
Useful for giving fields of a struct references to the parent struct.

When running in debug mode, `Soon`s will panic if they are replaced from a different thread, replaced while being non-empty, or dereferenced while being empty.
For _maximum performance_, these safety features are disabled in release modes.

## Example

```rust
struct App {
    item: Soon<Item>
}

struct Item {
    app: Arc<App>
}

fn main() {
    let app = Arc::new(App {
        item: Soon::empty(),
    });
    let item = Item { app: app.clone() };
    app.item.replace(item);
}
```
