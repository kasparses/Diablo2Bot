use std::collections::{hash_map::Entry, HashMap};
use std::process::exit;
use std::thread;
use std::time::Duration;

use enigo::{Enigo, MouseControllable};

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct MousePoint {
    row: i32,
    col: i32,
}

impl MousePoint {
    fn new(point: (i32, i32)) -> Self {
        Self {
            row: point.1,
            col: point.0,
        }
    }
}

pub fn setup_mouse_movement_program_stopper() {
    const SIZE: usize = 100;

    let enigo = Enigo::new();

    let mp = MousePoint::new(enigo.mouse_location());

    let mut mouse_points = vec![mp; SIZE];

    let mut counts: HashMap<MousePoint, u32> = HashMap::new();
    let mut num_unique_mouse_points: u32 = 0;

    for &mp in &mouse_points {
        increment_count(&mut counts, &mut num_unique_mouse_points, mp);
    }

    let mut i = 0;

    thread::spawn(move || loop {
        if i == SIZE {
            i = 0;
        }

        decrement_count(&mut counts, &mut num_unique_mouse_points, mouse_points[i]);

        let mp = MousePoint::new(enigo.mouse_location());

        mouse_points[i] = mp;

        increment_count(&mut counts, &mut num_unique_mouse_points, mp);

        if num_unique_mouse_points > 20 {
            println!("Stopping program due to manual mouse movement from player!");
            exit(0);
        }

        i += 1;

        thread::sleep(Duration::from_millis(10));
    });
}

fn increment_count(
    counts: &mut HashMap<MousePoint, u32>,
    num_unique_mouse_points: &mut u32,
    mp: MousePoint,
) {
    match counts.entry(mp) {
        Entry::Occupied(mut entry) => {
            *entry.get_mut() += 1;
        }
        Entry::Vacant(entry) => {
            entry.insert(1);
            *num_unique_mouse_points += 1;
        }
    }
}

fn decrement_count(
    counts: &mut HashMap<MousePoint, u32>,
    num_unique_mouse_points: &mut u32,
    mp: MousePoint,
) {
    if let Entry::Occupied(mut entry) = counts.entry(mp) {
        if *entry.get() == 1 {
            entry.remove();
            *num_unique_mouse_points -= 1;
        } else {
            *entry.get_mut() -= 1;
        }
    }
}
