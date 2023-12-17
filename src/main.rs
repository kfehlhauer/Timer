use clap::{value_parser, Arg, Command};
use signal_hook::{consts::signal::SIGINT, iterator::Signals};
use std::{
    io::{self, Write},
    process, thread, time,
};

fn main() {
    let matches = Command::new("Countdown Timer")
        .version("1.0")
        .author("Kurt Fehlhauer")
        .about("A simple countdown timer CLI app")
        .arg(
            Arg::new("minutes")
                .value_parser(value_parser!(usize))
                .short('m')
                .help("Minutes for the countdown")
                .default_value("0"),
        )
        .arg(
            Arg::new("seconds")
                .value_parser(value_parser!(usize))
                .short('s')
                .help("Seconds for the countdown")
                .default_value("0"),
        )
        .arg(
            Arg::new("reset")
                .value_parser(value_parser!(usize))
                .short('r')
                .required(false)
                .help("Reset time in seconds"),
        )
        .arg( 
            Arg::new("repeat")
                .value_parser(value_parser!(usize))
                .short('n')
                .required(false)
                .help("Number of times to repeat. If not specified the timer repeats forever if the reset flag is set")
        )
        .get_matches();

    let minutes: usize = matches.get_one("minutes").cloned().unwrap();

    let seconds: usize = matches.get_one("seconds").cloned().unwrap();

    if minutes + seconds == 0 {
        eprintln!("Please specify --minutes (-m) and/or --seconds (-s)");
        std::process::exit(1);
    }

    let mut signals = Signals::new(&[SIGINT]).expect("Error handling signals");
    thread::spawn(move || {
        for _ in signals.forever() {
            println!("\nTimer cancelled.");
            process::exit(0);
        }
    });

    let total_seconds = minutes * 60 + seconds;

    let mut reset_interval = matches
        .get_one::<usize>("reset")
        .cloned()
        .filter(|&r_i| r_i > 0)
        .map(|r_i| time::Duration::from_secs(r_i as u64));

    let mut repeat = matches
        .get_one::<usize>("repeat")
        .cloned()
        .filter(|&r| r > 0);

    loop {
    repeat = match repeat {
            Some(r) => {
                    println!("\n{} reps left", r);
                    if r > 1 {
                        Some(r - 1)
                    }
                    else{
                        println!("Last rep!");
                        reset_interval = None;
                        None
                    }
                },   
             None => None,
        
        };
        countdown(total_seconds);
        if let Some(interval) = reset_interval {
            print!("\nTimer reset in {:?}", interval);
            io::stdout().flush().unwrap();
            thread::sleep(interval);
        } else {
            break;
        }
    }
    println!("\nTime's up!");
}

fn countdown(seconds: usize) {
    for remaining in (0..=seconds).rev() {
        io::stdout().flush().unwrap();
        print!("\r                    ");
        print!("\r{} seconds remaining", remaining);
        io::stdout().flush().unwrap();
        thread::sleep(time::Duration::from_secs(1));
    }
}
