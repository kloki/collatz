use clap::Parser;
use plotters::prelude::*;
use std::{error::Error, vec};
/// Graph collatz.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Start value
    start: usize,
    /// End value
    end: usize,
    ///
    #[arg(short, long, default_value = "./output.png")]
    output_file: String,
}
fn main() {
    let args = Args::parse();
    graph(args.start, args.end, args.output_file).unwrap();
}

fn generate_data_set(start: usize, end: usize) -> Vec<Vec<usize>> {
    (start..end).map(|x| collatz_run(x)).collect()
}

fn max_height(data_set: &Vec<Vec<usize>>) -> usize {
    *data_set
        .iter()
        .map(|data| data.iter().max().unwrap())
        .max()
        .unwrap()
}

fn max_iterations(data_set: &Vec<Vec<usize>>) -> usize {
    data_set.iter().map(|data| data.len()).max().unwrap()
}
fn graph(start: usize, end: usize, file_name: String) -> Result<(), Box<dyn Error>> {
    let data_set = generate_data_set(start, end);
    let root = BitMapBackend::new(&file_name, (1024, 768)).into_drawing_area();

    root.fill(&WHITE)?;

    let max_height = max_height(&data_set);
    let max_iterations = max_iterations(&data_set);
    let mut chart = ChartBuilder::on(&root)
        .caption(
            &format!("Collatz sequence for {} to {}", start, end),
            ("sans-serif", 20),
        )
        .set_label_area_size(LabelAreaPosition::Left, 100)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(0..max_iterations, 0..max_height)?;

    for (i, data) in data_set.into_iter().enumerate() {
        let line_data = LineSeries::new(data.into_iter().enumerate(), &Palette9999::pick(i));
        chart.draw_series(line_data)?;
    }
    chart
        .configure_mesh()
        .x_desc("Iteration")
        .y_desc("Value")
        .draw()?;
    Ok(())
}

fn collatz_run(mut input: usize) -> Vec<usize> {
    let mut output = vec![input];
    while input > 1 {
        input = collatz(input);
        output.push(input);
    }
    output
}

fn collatz(input: usize) -> usize {
    match input {
        x if x % 2 == 0 => x / 2,
        x => x * 3 + 1,
    }
}

#[cfg(test)]
mod tests {
    use super::graph;

    #[test]
    fn test_basic() {
        graph(0, 1000, "test.png".to_string()).unwrap();
    }
}
