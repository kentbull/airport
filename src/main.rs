use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

mod random;

mod geo;
use geo::distance;

fn main() {
    calculate_total_distance();

    threaded_distance_calculation();

    threaded_chat();

    crate::random::gen_random();
}

fn threaded_chat() {
    let (john_tx, john_rx) = mpsc::channel();
    let (sarah_tx, sarah_rx) = mpsc::channel();

    let john_handle = thread::spawn(move || {
        john_chat(sarah_tx, john_rx);
    });

    let sarah_handle = thread::spawn(move || {
        sarah_chat(john_tx, sarah_rx);
    });

    match john_handle.join() {
        Ok(_) => {}
        Err(_) => {}
    }
    match sarah_handle.join() {
        Ok(_) => {}
        Err(_) => {}
    }
}

fn sarah_chat(john_tx: Sender<&str>, sarah_rx: Receiver<&str>) {
    let result = sarah_rx.recv();
    println!("SARAH RECEIVE: {}", result.unwrap());

    let _send_result = john_tx.send("Hello my dear Johnny!");
}

fn john_chat(sarah_tx: Sender<&str>, john_rx: Receiver<&str>) {
    let _send_result = sarah_tx.send("Why hello, beautiful...");

    let result = john_rx.recv();
    println!("JOHN RECEIVE: {}", result.unwrap());
}

fn threaded_distance_calculation() {
    let outer_distance = 412;

    let join_handle = thread::spawn(move || outer_distance * 2);

    let result = join_handle.join();

    match result {
        Ok(value) => {
            println!("Result from within thread {}", value)
        }
        Err(_) => {}
    }
}

fn calculate_total_distance() {
    let kcle_latitude_degrees: f64 = 41.4075;
    let kcle_longitude_degrees: f64 = -81.85111;
    let kslc_latitude_degrees: f64 = 40.7861;
    let kslc_longitude_degrees: f64 = -111.9822;

    let distance = distance(
        kcle_latitude_degrees,
        kcle_longitude_degrees,
        kslc_latitude_degrees,
        kslc_longitude_degrees,
    );

    println!(
        "Total distance between all waypoints is {:.1} kilometers.",
        distance
    );
}
