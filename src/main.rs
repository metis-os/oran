use std::env::args;
use std::process::Command;

mod utils;

fn main() {
    let (conn, _) = xcb::Connection::connect(None).unwrap();
    let focused = xcb::get_input_focus(&conn).get_reply().unwrap().focus();

    xcb::unmap_window_checked(&conn, focused)
        .request_check()
        .unwrap();

    conn.flush();

    let command = args().map(utils::ignore).skip(1).collect::<Vec<_>>().join(" ");
    Command::new("sh")
        .arg("-c")
        .arg(command)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    xcb::map_window_checked(&conn, focused)
        .request_check()
        .unwrap();

    conn.flush();
}

