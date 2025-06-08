/// A cute friend who's there for you when you make a typo.
use std::{thread::sleep, time::Duration};

use std::io::{Write, stdout};

static FRAMES: [(&str, u8); 15] = [
    ("(\\°-°)\\ ┬─┬", 23),
    ("(\\°□°)\\  ┬─┬", 2),
    ("(-°□°)-  ┬─┬", 2),
    ("(╯°□°)╯    ]", 4),
    ("(╯°□°)╯  ︵  ┻━┻", 4),
    ("(╯°□°)╯       [", 4),
    ("(╯°□°)╯       ︵  ┬─┬", 4),
    ("(╯°□°)╯                 ]", 4),
    ("(╯°□°)╯               ︵  ┻━┻", 4),
    ("(╯°□°)╯                         [", 4),
    ("(\\°-°)\\                            ︵  ┬─┬", 4),
    ("(\\°-°)\\                                     ]", 4),
    ("(\\°-°)\\                                     ︵ ┻━┻", 4),
    (
        "(\\°-°)\\                                               [",
        4,
    ),
    (
        "(\\°-°)\\                                              ┬─┬",
        10,
    ),
];

fn main() {
    let mut lock = stdout().lock();
    for (frame, time) in FRAMES {
        write!(lock, "\r{}", frame).unwrap();
        lock.flush().unwrap();
        sleep(Duration::from_millis((time * 25).into()));
    }
    writeln!(lock).unwrap();
}
