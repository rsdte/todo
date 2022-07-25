use std::vec;

use druid::{WindowDesc, AppLauncher};
use todo::TodoItem;

mod window;
mod todo;

fn main() {
    let data = vec![
        TodoItem::new("测试1"),
        TodoItem::new("测试2"),
        TodoItem::new("测试3"),
    ];
    let state = window::TodoWindowState::from_vec(data);
    let win = WindowDesc::new(window::TodoWindow::new())
        .title("title");
    AppLauncher::with_window(win)
        .delegate(window::WindowDelegate{})
        .launch(state).expect("running failed.");
}
