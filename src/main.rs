use colored::{Color, ColoredString, Colorize};
use crossterm::terminal;
use crossterm::terminal::ClearType;
use rand::prelude::SliceRandom;
use std::io::stdout;
use std::time::Duration;
use std::{io, thread};

fn main() {
    // Set some global states to keep track of the previous row iteration.
    let (mut prev_num_cols, mut prev_num_rows, mut prev_left_pad) = (0usize, 0usize, 0usize);
    let mut prev_colors: Vec<Color> = Vec::new();

    loop {
        // The parameter calculations are performed every time so that we can resize the
        // output as the terminal gets resized. These are done in a different thread while the
        // current thread sleeps.
        let calc_thread_handle = thread::spawn(move || {
            let (cur_num_cols, cur_num_rows, cur_left_pad) = calculate_term_params().unwrap();
            if (cur_num_cols, cur_num_rows, cur_left_pad)
                != (prev_num_cols, prev_num_rows, prev_left_pad)
            {
                spawn_full_rect(cur_num_cols, cur_num_rows, cur_left_pad);
            }

            (cur_num_cols, cur_num_rows, cur_left_pad)
        });

        thread::sleep(Duration::from_millis(250));

        let (num_cols, num_rows, left_pad) = calc_thread_handle.join().unwrap();

        // If we have adjacent colors in a column, we have a high chance that
        // the current one is swapped for a new one. We only want to check this
        // if the number of columns actually match.
        let mut current_colors = get_random_colors(&consts::COLORS, num_cols);
        if current_colors == prev_colors {
            for i in 0..current_colors.len() {
                if current_colors[i] == prev_colors[i] {
                    current_colors[i] = *consts::COLORS.choose(&mut rand::thread_rng()).unwrap()
                }
            }
        }

        print_row(
            &consts::IN_RAINBOWS,
            &consts::SYMBOLS,
            &current_colors,
            left_pad,
        );

        (prev_num_cols, prev_num_rows, prev_left_pad) = (num_cols, num_rows, left_pad);
        prev_colors = current_colors;
    }
}

/// Print a full colored row across the width of the terminal
fn print_row(s: &str, symbols: &[char], row_colors: &Vec<Color>, left_pad: usize) {
    for _ in 0..left_pad {
        print!(" ");
    }
    for i in 0..row_colors.len() {
        print!("{}", &mutate_str(s, symbols, &row_colors[i]));
    }
    println!();
}

/// Print a full rectangle covering the entire terminal.
fn spawn_full_rect(num_cols: usize, num_rows: usize, left_pad: usize) {
    let mut prev_colors: Vec<Color> = Vec::new();
    // Clear terminal to prepare to fill whole terminal again
    crossterm::execute!(stdout(), terminal::Clear(ClearType::All)).unwrap();

    for _ in 0..num_rows {
        let mut current_colors = get_random_colors(&consts::COLORS, num_cols);

        if !prev_colors.is_empty() {
            assert_eq!(current_colors.len(), prev_colors.len());
            for i in 0..current_colors.len() {
                if current_colors[i] == prev_colors[i] {
                    current_colors[i] = *consts::COLORS.choose(&mut rand::thread_rng()).unwrap()
                }
            }
        }

        print_row(
            &consts::IN_RAINBOWS,
            &consts::SYMBOLS,
            &current_colors,
            left_pad,
        );

        prev_colors = current_colors;
    }
}

/// Add the "in_rainbows" styling to an `&str` by adding a symbol and a color.
/// Returns a `colored::ColoredString`.
fn mutate_str(s: &str, symbols: &[char], color: &Color) -> ColoredString {
    let (left, right) = s.split_at(rand::random::<usize>() % s.len());
    format!(
        "{}{}{} ",
        left,
        symbols.choose(&mut rand::thread_rng()).unwrap(),
        right
    )
    .color(*color)
}

/// Calculate the number of columns and rows needed to output to the terminal as well as the
/// necessary left padding.
fn calculate_term_params() -> io::Result<(usize, usize, usize)> {
    let (terminal_width, terminal_height) = terminal::size()?;
    let num_cols = terminal_width as usize / (consts::IN_RAINBOWS.len() + 2);
    let num_rows = terminal_height as usize - 1;
    let left_pad = (terminal_width as usize % (consts::IN_RAINBOWS.len() + 2)) / 2;
    Ok((num_cols, num_rows, left_pad))
}

/// Make a vector of random colors from the passed color array.
fn get_random_colors(colors: &[Color], length: usize) -> Vec<Color> {
    let mut color_vec: Vec<Color> = Vec::new();
    for _ in 0..length {
        color_vec.push(*colors.choose(&mut rand::thread_rng()).unwrap());
    }
    color_vec
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
