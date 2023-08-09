use clap::Parser;
use termplot::*;
/// Graph collatz.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Start value
    start: usize,
    /// Width of graph
    #[arg(long, default_value_t = 80)]
    width: usize,
    /// Height of graph
    #[arg(long, default_value_t = 40)]
    height: usize,
}
fn main() {
    let args = Args::parse();
    let output = run(args.start);
    let mut plot = Plot::default();
    let max_value = output.iter().max().unwrap();
    plot.set_domain(Domain(0.0..output.len() as f64))
        .set_codomain(Domain(0.0..*max_value as f64))
        .set_title(&format!("Collatz Graph: {}", args.start))
        .set_x_label(&format!("Steps: {}", output.len() - 1))
        .set_y_label(&format!("Max value: {}", max_value))
        .set_size(Size::new(args.width, args.height))
        .add_plot(Box::new(plot::Bars::new(
            output.iter().map(|x| *x as f64).collect(),
        )));
    println!("{plot}");
}

fn run(mut input: usize) -> Vec<usize> {
    let mut output = vec![input];
    while input != 1 {
        input = colatz(input);
        output.push(input);
    }
    output
}

fn colatz(input: usize) -> usize {
    match input {
        x if x % 2 == 0 => x / 2,
        x => x * 3 + 1,
    }
}
