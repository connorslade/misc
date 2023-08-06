use std::{sync::Arc, thread};

use super::Soon;

#[test]
fn test_basic() {
    struct App {
        item: Soon<Item>,
    }

    struct Item {
        app: Arc<App>,
    }

    let app = Arc::new(App {
        item: Soon::empty(),
    });
    let item = Item { app: app.clone() };
    app.item.replace(item);

    assert_eq!(Arc::into_raw(app.item.app.clone()), Arc::into_raw(app));
}

#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn test_thread_safety() {
    let soon: Soon<i32> = Soon::empty();
    thread::scope(|s| {
        s.spawn(|| soon.replace(0));
    })
}

#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn test_has_value() {
    let soon: Soon<u32> = Soon::empty();
    assert_eq!(*soon, 0);
}