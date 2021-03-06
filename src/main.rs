use clap::Parser;
use std::{
    sync::mpsc::{channel, Receiver, Sender},
    time::{Duration, Instant},
};

#[derive(Parser)]
struct Args {
    #[clap(long)]
    bench_workers: usize,
    #[clap(long)]
    workers: usize,
    #[clap(long)]
    run_for_seconds: u64,
}

fn make_communication_matrix<T>(n: usize) -> (Vec<Receiver<T>>, Vec<Sender<T>>)
where
    T: Send,
{
    let mut receivers = Vec::new();
    let mut senders = Vec::new();
    for _ in 0..n {
        let (sender, receiver) = channel();
        receivers.push(receiver);
        senders.push(sender);
    }

    (receivers, senders)
}

fn main() {
    let Args {
        bench_workers,
        workers,
        run_for_seconds,
    } = Args::parse();

    let run_for = Duration::from_secs(run_for_seconds);
    let start = Instant::now();

    let (receivers, senders) = make_communication_matrix::<_>(workers);

    let handles: Vec<_> = (0..bench_workers)
        .map(|_| {
            let senders = senders.clone();
            std::thread::spawn(move || {
                let mut i: usize = 0;
                while start.elapsed() < run_for {
                    let idx = i % senders.len();
                    senders[idx].send(i).unwrap();
                    i += 1;
                }
                i
            })
        })
        .collect();

    for receiver in receivers {
        std::thread::spawn(move || {
            while let Ok(msg) = receiver.recv() {
                // Just drop the message.
                let _ = msg;
            }
        });
    }

    let mut total_ops = 0;
    for handle in handles {
        total_ops += handle.join().unwrap();
    }

    println!("{},{},{}", bench_workers, workers, total_ops)
}
