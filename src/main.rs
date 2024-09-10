use colored::{Color, ColoredString, Colorize};
use rand::prelude::SliceRandom;
use std::thread;
use std::time::Duration;

fn main() {
    loop {
        // The column and padding calculations are performed every time so that we can resize the
        // output as the terminal gets resized. These are done in a different thread while the
        // current thread sleeps.
        let calc_thread_handle = thread::spawn(|| {
            let (terminal_width, _terminal_height) = term_size::dimensions().unwrap();
            let num_cols = terminal_width / (consts::IN_RAINBOWS.len() + 2);
            let left_pad = (terminal_width % (consts::IN_RAINBOWS.len() + 2)) / 2 + 1;
            (num_cols, left_pad)
        });

        thread::sleep(Duration::from_millis(300));

        let (num_cols, left_pad) = calc_thread_handle.join().unwrap();

        for _ in 0..left_pad {
            print!(" ");
        }
        for _ in 0..num_cols {
            print!(
                "{}",
                &mutate_str(consts::IN_RAINBOWS, &consts::SYMBOLS, &consts::COLORS)
            );
        }
        println!();
    }
}

fn mutate_str(s: &str, symbols: &[char], colors: &[Color]) -> ColoredString {
    let (left, right) = s.split_at(rand::random::<usize>() % s.len());
    format!(
        "{}{}{} ",
        left,
        symbols.choose(&mut rand::thread_rng()).unwrap(),
        right
    )
    .color(*colors.choose(&mut rand::thread_rng()).unwrap())
}

mod consts {
    use colored::Color;

    pub(crate) const SYMBOLS: [char; 2] = ['/', '_'];
    pub(crate) const IN_RAINBOWS: &str = "in rainbows";
    pub(crate) const COLORS: [Color; 7] = [
        // Yellow
        Color::TrueColor {
            r: 246,
            g: 237,
            b: 75,
        },
        // Blue
        Color::TrueColor {
            r: 70,
            g: 132,
            b: 196,
        },
        // Orange
        Color::TrueColor {
            r: 243,
            g: 102,
            b: 40,
        },
        // Green
        Color::TrueColor {
            r: 72,
            g: 182,
            b: 76,
        },
        // Darker yellow
        Color::TrueColor {
            r: 233,
            g: 176,
            b: 24,
        },
        // Red
        Color::TrueColor {
            r: 230,
            g: 32,
            b: 33,
        },
        // Lighter blue
        Color::TrueColor {
            r: 164,
            g: 222,
            b: 227,
        },
    ];
}
