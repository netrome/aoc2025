pub fn run() -> anyhow::Result<()> {
    let args = Args::parse();

    let input = read_input(&args)?;
    let solution = pick_solution(&args)?;

    let ans = (solution.run)(&input);
    println!("{}", ans);

    Ok(())
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    day: usize,

    #[arg(short, long)]
    part: usize,

    /// Input override, otherwise defaults to ./input/d{day}.txt
    #[arg(short, long)]
    input_file: Option<String>,
}

fn read_input(args: &Args) -> anyhow::Result<String> {
    let default_path = format!("./input/d{}.txt", args.day);
    let input_file = args.input_file.as_ref().unwrap_or(&default_path);

    Ok(std::fs::read_to_string(input_file)?)
}

fn pick_solution(args: &Args) -> Result<&'static solution::Solution, anyhow::Error> {
    let solution = inventory::iter::<solution::Solution>
        .into_iter()
        .find(|solution| solution.day == args.day && solution.part == args.part)
        .ok_or_else(|| {
            anyhow::anyhow!("No solution found for day {} part {}", args.day, args.part)
        })?;
    Ok(solution)
}

inventory::collect!(solution::Solution);

mod solution;
mod utils;

mod day1;
mod day2;

use clap::Parser;
