use std::time::Duration;
use signal_hook::consts::*;
use signal_hook::iterator::Signals;
use crossbeam::channel::{select, self, Sender, Receiver, after};

fn await_interrupt(interrupt_notification_channel: Sender<()>) {
    let mut signals = Signals::new(&[
        SIGINT,
    ]).unwrap();

    for _ in &mut signals {
        interrupt_notification_channel.send(());
    }
}
fn main() {
    let (interrupt_tx, interrupt_rx) = channel::unbounded();
    std::thread::spawn(move || { await_interrupt(interrupt_tx) });

    let timeout = after(Duration::from_secs(5));
    loop {
        select! {
            recv(interrupt_rx) -> _ => {
                println!("Received interrupt notification");
                // break;
            },
            recv(timeout) -> _ => {
                println!("Finally finished the long task");
                break;
            }
        }
    }
}
