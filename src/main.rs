#![feature(box_syntax, box_patterns, uniform_paths)]

extern crate clap;

mod defs;
mod paging;

// extern crates
use clap::{Arg, App};
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::unreachable;

// namespacing stuff bc i HATE DOUBLE COLONS ALL OVER THE PLACE
use defs::*;
use Replacement::*;
use paging::*;


fn main() -> std::io::Result<()> {
    let check_corruption;
    let replacement_options: [&str; 4] = ["random", "fifo", "clocksweep", "lru"];

    let matches = App::new("Virtual Memory Simulator")
                            .version("0.1.0")
                            .author("Jacob Meyers")
                            .arg(Arg::with_name("trace-file")
                                .short("i")
                                .long("trace")
                                .help("Input trace file for the simulation.")
                                .value_name("FILE")
                                .required(true))
                            .arg(Arg::with_name("corruption-check")
                                .short("c")
                                .long("corruption-check")
                                .help("Enables strict memory corruption checking."))
                            .arg(Arg::with_name("replacement")
                                .short("r")
                                .long("replacement")
                                .help("Eviction algorithm")
                                .possible_values(&replacement_options)
                                .default_value("random"))
                            .get_matches();


    let trace_fname = matches.value_of("trace-file").unwrap();
    let mut trace_file = File::open(trace_fname).expect(&format!("Couldn't open trace file {}", trace_fname)); 

    let replacement = match matches.value_of("replacement").unwrap() {
        "random" => RANDOM,
        "fifo" => FIFO,
        "clocksweep" => CLOCKSWEEP,
        "lru" => LRU,
        _ => unreachable!(), // Will never happen, if it's not one of the above it will be caught by the arg parser. Rust requires pattern matching to be exhaustive.
    };

    if matches.is_present("corruption-check") {
        println!("corruption check enabled");
        check_corruption = true;
    } else {
        println!("corruption check not enabled");
        check_corruption = false;
    }
    println!("Trace file: {}", trace_fname);
    println!("Replacement: {}", replacement);

    // Initialize variables for the simulator
    // Create procs
    let mut procs: [Process; MAX_PID as usize] = [Process::default(); MAX_PID as usize];
    let mut mem = Memory::new(replacement);

    // Start the simulation
    system_init(&mut mem);

    // Iterate over each line in the trace file and run the command

    for line in BufReader::new(trace_file).lines() {
        let command: String = line.unwrap();
        simulate_command(command);
    }



    println!("Made it!");


    Ok(())
}

fn simulate_command(command: String) {
    
}
