use std::env;
use std::process;

// Just write out all the permutations i.s.o doing it dynamically
// + is always first
const PERMUTATIONS: [[char; 3]; 6] = [
    ['-', '*', '/'],
    ['-', '/', '*'],
    ['*', '-', '/'],
    ['*', '/', '-'],
    ['/', '-', '*'],
    ['/', '*', '-'],
];

// Returns None only when dividing by zero.
// We return f64 since fraction should be posible in intermediate steps
fn apply(a: f64, op: char, b: i64) -> Option<f64> {
    let b = b as f64;
    match op {
        '-' => Some(a - b),
        '*' => Some(a * b),
        '/' => {
            if b == 0.0 {
                None
            } else {
                Some(a / b)
            }
        }
        _ => None,
    }
}

fn parse_args(args: &[String]) -> Result<[i64; 4], String> {
    // A single 4-letter word
    if args.len() == 1 {
        let word = args[0].to_lowercase();
        if word.len() == 4 && word.chars().all(|c| c.is_ascii_lowercase()) {
            let mut nums = [0i64; 4];
            for (i, c) in word.chars().enumerate() {
                nums[i] = (c as u8 - b'a' + 1) as i64;
            }
            return Ok(nums);
        }
        return Err("A single argument must be a 4-letter word.".to_string());
    }

    // 4 separate integers
    if args.len() == 4 {
        let mut nums = [0i64; 4];
        for (i, a) in args.iter().enumerate() {
            nums[i] = a
                .parse::<i64>()
                .map_err(|_| format!("'{}' is not an integer", a))?;
        }
        return Ok(nums);
    }

    Err("Expected either 4 integers or a single 4-letter word.".to_string())
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let nums = match parse_args(&args) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("Usage: numeric_core <int> <int> <int> <int>");
            eprintln!("   or: numeric_core <word>");
            process::exit(1);
        }
    };

    println!("Numbers: {:?}", nums);

    let mut best: Option<(i64, [char; 3])> = None;

    for ops in PERMUTATIONS.iter() {
        let step1 = apply(nums[0] as f64, ops[0], nums[1]);
        let step2 = step1.and_then(|v| apply(v, ops[1], nums[2]));
        let step3 = step2.and_then(|v| apply(v, ops[2], nums[3]));

        if let Some(result) = step3 {
            // Little trick for checking whole number.
            let whole = result.round();
            if (result - whole).abs() > 1e-9 {
                continue;
            }

            let whole = whole as i64;

            // Result must not be negative and lowest "wins"
            if whole >= 0 && (best.is_none() || whole < best.unwrap().0) {
                best = Some((whole, *ops));
            }
        }
    }

    match best {
        Some((result, ops)) => println!(
            "Numeric Core: + {} {} {} {} {} {} {} = {}",
            nums[0], ops[0], nums[1], ops[1], nums[2], ops[2], nums[3], result
        ),
        None => println!("No combination gives a whole, non-negative result."),
    }
}
