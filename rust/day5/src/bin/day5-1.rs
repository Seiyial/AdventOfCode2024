use aoc::solve_input_file_to_int;
use day5::model::RuleMap;

fn solve(input: String) -> u64 {
    let (rules_str, input_str) = input.split_once("\n\n").unwrap();
    let rules = rules_str.lines().collect::<Vec<&str>>();
    let rule_map = RuleMap::from_rules(rules).ensure_has_all_relations();

    // for every item in input_str
    input_str.lines().fold(0u64, |ok_lines, line| {
        let vals = line.split(",").collect::<Vec<&str>>();
        let mid_val = vals[(vals.len() - 1) / 2];
        match vals
            .clone()
            .iter()
            .enumerate()
            .all(|(tester_idx, tester_val)| {
                (0..vals.len() as usize).all(|idx| match vals[idx] {
                    v if idx < tester_idx => rule_map.is_valid_order(v, tester_val),
                    v if tester_idx < idx => rule_map.is_valid_order(tester_val, v),
                    _ => true,
                })
            }) {
            true => ok_lines + mid_val.parse::<u64>().unwrap(),
            _ => ok_lines,
        }
    })
}

fn main() {
    solve_input_file_to_int(solve)
}
