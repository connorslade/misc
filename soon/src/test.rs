use std::sync::Arc;

use super::Soon;

#[test]
fn main() {
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
}
