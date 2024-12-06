pub mod model {
    use std::collections::{HashMap, HashSet};

    #[derive(Debug)]
    pub struct OrderedItem {
        pub val: String,
        pre: HashSet<String>,
        post: HashSet<String>,
    }
    impl OrderedItem {
        pub fn new(val: &String) -> OrderedItem {
            OrderedItem {
                pre: HashSet::new(),
                post: HashSet::new(),
                val: val.clone(),
            }
        }
    }

    #[derive(Debug)]
    pub struct RuleMap {
        map: HashMap<String, OrderedItem>,
    }
    impl RuleMap {
        pub fn new() -> RuleMap {
            RuleMap {
                map: HashMap::new(),
            }
        }
        pub fn from_rules(rules: Vec<&str>) -> RuleMap {
            rules.into_iter().fold(RuleMap::new(), |mut map, rule_str| {
                let pair = rule_str.split_once("|").unwrap();
                map.add_rule(pair.0, pair.1);
                map
            })
        }
        pub fn add_rule(&mut self, before: &str, after: &str) {
            self.map
                .entry(before.to_string())
                .or_insert(OrderedItem::new(&before.to_string()))
                .post
                .insert(after.to_string());
            self.map
                .entry(after.to_string())
                .or_insert(OrderedItem::new(&after.to_string()))
                .pre
                .insert(before.to_string());
        }
        pub fn is_valid_order(&self, before: &str, after: &str) -> bool {
            self.map
                .get(&before.to_string())
                .and_then(|l| Some(!l.pre.contains(&after.to_string())))
                .unwrap_or(false)
        }

        pub fn ensure_has_all_relations(self) -> RuleMap {
            // broken_line
            let first = self.map.values().nth(0).expect("Rulemap is empty");
            let num_relations = first.pre.len() + first.post.len();
            if self
                .map
                .values()
                .all(|v| v.post.len() + v.pre.len() == num_relations)
            {
                println!("Yes, rulemap has all relations ✅");
                self
            } else {
                panic!("Rulemap does not have equal sized relations")
            }
        }
    }
}
