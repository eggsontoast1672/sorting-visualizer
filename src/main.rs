mod sorters;

use ::rand::seq::SliceRandom;
use clap::{Parser, ValueEnum};
use macroquad::prelude::*;

use crate::sorters::{BubbleSorter, Sorter};

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;

fn window_conf() -> Conf {
    Conf {
        window_title: "Sorting Visualizer".into(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
}

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
    #[arg(short, long, default_value_t = SortingAlgorithm::Quick)]
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

fn draw_data(data: &[usize]) {
    for (index, number) in data.iter().enumerate() {
        let block_width = WINDOW_WIDTH as f32 / data.len() as f32;
        let block_height = *number as f32 / data.len() as f32 * WINDOW_HEIGHT as f32;
        draw_rectangle(
            index as f32 * block_width,
            WINDOW_HEIGHT as f32 - block_height,
            block_width,
            block_height,
            WHITE,
        );
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let args = Args::parse();
    let mut data = create_data(args.num_elements);
    let mut sorter = create_sorter(args.algorithm, data.len());

    loop {
        draw_data(&data);
        sorter.step(&mut data);
        next_frame().await;
    }
}
