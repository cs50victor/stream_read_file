use std::fs::File;
use std::io::{Seek, SeekFrom};
use std::os::unix::io::AsRawFd;
use std::path::Path;
use std::time::Duration;

use memmap2::{Mmap, MmapOptions};
use mio::unix::SourceFd;
use mio::{Events, Interest, Poll, Token};

fn main() {
    let file_path = Path::new("./test.txt");
    stream_file_changes(file_path);
}

pub fn stream_file_changes(file_path: &Path) {
    let file = File::open(file_path).expect("Failed to syslog file");
    let mmap = unsafe { Mmap::map(&file).expect("Failed to map file") };

    let mut poll = Poll::new().expect("Failed to create poll");
    let token = Token(0);

    let raw_fd = file.as_raw_fd();
    if raw_fd < 0 {
        eprintln!("Invalid file descriptor");
        return;
    }

    poll.registry()
        .register(&mut SourceFd(&raw_fd), token, Interest::READABLE)
        .expect("Failed to register file");

    let mut events = Events::with_capacity(1024);
    // let mut position = mmap.len();
    let mut position = 0;

    loop {
        poll.poll(&mut events, None).expect("Failed to poll");

        let event = events.iter().next().unwrap();
        if event.is_readable() {
            let file_size = mmap.len();
            println!("file_size - {}", file_size);
            if position > file_size {
                position = 0;
            }
            let lastest_syslog_content =
                std::str::from_utf8(&mmap[position..]).expect("Invalid UTF-8 content");
            // send_queue_tx.send(lastest_syslog_content.to_owned());
            println!("> {}", lastest_syslog_content);
            position = mmap.len();
        }
    }
}
