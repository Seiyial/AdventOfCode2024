use aoc::solve_input_file_to_int;
use day5::model::RuleMap;

// this is an exported function (in a way I didn't need to export it) called fix_line
//
// <'a> is a lifetime parameter (different from type parameter),
// it is not needed in JS and not used in JS and most other high-level languages.
// it's to control the flow of memory use of the strings contained in `vals`.
// My use of it here says "the memory-use of values of the strings in vals
// is the same as the memory use of the returned value."
//
// this function accepts two parameters. the first is vals, a Vector (~ JS's Array) of string slices.
// the second is rulemap, an instance of a RuleMap struct (see the other file for RuleMap).
// The function returns a Vector (~ JS's Array) of string slices. (the fixed line)
pub fn fix_line<'a>(vals: Vec<&'a str>, rulemap: &RuleMap) -> Vec<&'a str> {
    // I make a copy of vals that I can now edit, calling it new_line.
    let mut new_line = vals.clone();
    // I sort this new_line using a comparison function.
    // |a, b| EXPRESSION is akin to (a, b) => EXPRESSION in JS.
    // However, in Rust, `match` returns a value, so I am allowed to return a match.
    // In JS, the compare function should return an integer to describe the ordering,
    // but in Rust I am required to return one of the values of the std::cmp::Ordering enum.
    // There are three possible values, Ordering::Greater, Ordering::Less and Ordering::Equal.
    // Either ways, this sorts the Vector based on the correct order of a and b.
    // if A -> B is correct, I order A and B this way; if A -> B is wrong, I order A and B that way.
    new_line.sort_by(|a, b| match rulemap.is_valid_order(a, b) {
        true => std::cmp::Ordering::Greater,
        false => std::cmp::Ordering::Less,
    });
    new_line
}

fn solve(input: String) -> u64 {
    let (rules_str, input_str) = input.split_once("\n\n").unwrap();
    let rules = rules_str.lines().collect::<Vec<&str>>();
    let rulemap = RuleMap::from_rules(rules).ensure_has_all_relations();

    // for every item in input_str
    input_str.lines().fold(0u64, |sum, line| {
        // println!("\ntesting {}", line);
        let vals = line.split(",").collect::<Vec<&str>>();
        match vals
            .clone()
            .iter()
            .enumerate()
            .all(|(tester_idx, tester_val)| {
                (0..vals.len() as usize).all(|idx| match vals[idx] {
                    v if idx < tester_idx => rulemap.is_valid_order(v, tester_val),
                    v if tester_idx < idx => rulemap.is_valid_order(tester_val, v),
                    _ => true,
                })
            }) {
            true => sum,
            _ => {
                let fixed_line = fix_line(vals, &rulemap);
                sum + fixed_line[(fixed_line.len() - 1) / 2]
                    .parse::<u64>()
                    .unwrap()
            }
        }
    })
}

fn main() {
    solve_input_file_to_int(solve)
}
