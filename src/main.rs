mod sorters;

use clap::{Parser, ValueEnum};
use rand::seq::SliceRandom;
use raylib::prelude::*;

use crate::sorters::{BubbleSorter, Sorter};

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;

#[derive(Clone, Debug, ValueEnum)]
enum SortingAlgorithm {
    Bogo,
    Bubble,
    Quick,
}

impl std::fmt::Display for SortingAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", s.to_lowercase())
    }
}

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = SortingAlgorithm::Bubble)]
    algorithm: SortingAlgorithm,

    #[arg(short, long, default_value_t = 100)]
    num_elements: usize,
}

fn create_data(num_elements: usize) -> Vec<usize> {
    let mut data: Vec<_> = (1..=num_elements).collect();
    data.shuffle(&mut ::rand::rng());
    data
}

fn create_sorter(alg: SortingAlgorithm, data_length: usize) -> impl Sorter {
    match alg {
        SortingAlgorithm::Bogo => todo!(),
        SortingAlgorithm::Bubble => BubbleSorter::new(data_length),
        SortingAlgorithm::Quick => todo!(),
    }
}

/// Draw the numeric data to the screen.
///
/// Each number in the slice is rendered as a block. The height of each block is relative to the
/// size of the largest number in the data set. The number currently being sorted is rendered as a
/// red block which takes up the entire height of the window.
fn draw_data(d: &mut RaylibDrawHandle, data: &[usize], current: usize, is_done: bool) {
    for (index, number) in data.iter().enumerate() {
        let block_width = WINDOW_WIDTH as f32 / data.len() as f32;
        let (block_height, color) = if index == current && !is_done {
            (WINDOW_HEIGHT as f32, Color::RED)
        } else {
            (
                *number as f32 / data.len() as f32 * WINDOW_HEIGHT as f32,
                Color::WHITE,
            )
        };

        d.draw_rectangle_rec(
            Rectangle {
                x: index as f32 * block_width,
                y: WINDOW_HEIGHT as f32 - block_height,
                width: block_width,
                height: block_height,
            },
            color,
        );
    }
}

fn main() {
    let args = Args::parse();
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Sorting Visualizer")
        .build();

    rl.set_target_fps(240);

    let mut data = create_data(args.num_elements);
    let mut sorter = create_sorter(args.algorithm, data.len());

    while !rl.window_should_close() {
        let is_done = sorter.step(&mut data);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        draw_data(&mut d, &data, sorter.current(), is_done);
    }
}
