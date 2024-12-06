use aoc::solve_input_file_to_int;
use day5::model::RuleMap;

pub fn fix_line<'a>(vals: Vec<&'a str>, rulemap: &RuleMap) -> Vec<&'a str> {
    let mut new_line = vals.clone();
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
