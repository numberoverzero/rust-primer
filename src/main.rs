use std::{env, process::ExitCode};

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let Some(input) = args.next() else {
        eprintln!("usage: solve <unsigned-64-bit-integer>");
        return ExitCode::from(2);
    };
    if input == "--help" || input == "-h" {
        println!("usage: solve <unsigned-64-bit-integer>");
        return ExitCode::SUCCESS;
    }
    let Ok(target) = input.parse::<u64>() else {
        eprintln!("invalid target: expected an unsigned 64-bit integer");
        return ExitCode::from(2);
    };
    if args.next().is_some() {
        eprintln!("usage: solve <unsigned-64-bit-integer>");
        return ExitCode::from(2);
    }
    match primer::solver::solve(target) {
        Some(factors) => {
            println!("p:{}\nq:{}", factors.p, factors.q);
            ExitCode::SUCCESS
        }
        None => {
            eprintln!("no nontrivial factors for {target}");
            ExitCode::FAILURE
        }
    }
}
