// export a new module, model. in JS, file == module,
// but in rust I can declare custom module names iwthin a file
pub mod model {
    // import { Object, Set } from 'built-in-library/collections'
    // Object/Map in Rust is not a primitive type (so as to keep the program small)
    // and needs to be imported from the standard library
    use std::collections::{HashMap, HashSet};

    // auto-implement support for this "class" to print itself out in full detail
    #[derive(Debug)]
    // there are no classes in Rust (we do it similarly to Elixir but in reverse);
    // but we can see OrderedItem as a "class" for now
    pub struct OrderedItem {
        pub val: String,
        pre: HashSet<String>,
        post: HashSet<String>,
    }
    // in Rust a "class"'s methods are separately declared.
    // ie. we can have a "class" that's simply a structured container of data.
    // ie. a struct.
    // But I add a set of methods to OrderedItem, so this block is impl OrderedItem
    impl OrderedItem {
        // This function accepts a string (ignore the & for now), and returns an OrderedItem.
        pub fn new(val: &String) -> OrderedItem {
            // return { pre: new Set(), post: new Set(), val: val }
            OrderedItem {
                pre: HashSet::new(),
                post: HashSet::new(),
                val: val.clone(),
            }
        }
    }

    #[derive(Debug)]
    // this is another struct, RuleMap
    pub struct RuleMap {
        map: HashMap<String, OrderedItem>,
    }
    impl RuleMap {
        // this is a function to create a new RuleMap.
        // btw, in JS we have static class functions and class instance methods.
        // This is a static function that is called using RuleMap::new().
        pub fn new() -> RuleMap {
            RuleMap {
                map: HashMap::new(),
            }
        }
        // this is another static class function to create a new RuleMap from an Array of Strings.
        pub fn from_rules(rules: Vec<&str>) -> RuleMap {
            rules.into_iter().fold(RuleMap::new(), |mut map, rule_str| {
                let pair = rule_str.split_once("|").unwrap();
                map.add_rule(pair.0, pair.1);
                map
            })
        }
        // this is a method, not a static function, because its first argument is `&mut self`.
        // This is similar to `this` in JavaScript.
        // So this method allows me to add a rule to the RuleMap.
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
        // This is also another method but it takes `&self` instead of `&mut self`.
        // This means this is a "read-only" function, I cannot modify the Rulemap in this function.
        pub fn is_valid_order(&self, before: &str, after: &str) -> bool {
            self.map
                .get(&before.to_string())
                .and_then(|l| Some(!l.pre.contains(&after.to_string())))
                .unwrap_or(false)
        }

        // this is a "validation" function cause I wasn't sure if
        // the input data had every single relation between every
        // single number and I wanted to check.
        // panic!() is like throw in JavaScript.
        pub fn ensure_has_all_relations(self) -> RuleMap {
            // this.map is a Map
            // this.map.values() -> get item at index 0
            // expect it to exist, otherwise throw an error saying Rulemap is empty.
            // because self.map is a Map where keys are strings
            // and values are OrderedItems as described in line 37,
            // the first value is therefore an OrderedItem.
            // It is labelled as a &OrderedItem because it's a read-only copy of OrderedItem.
            // We are accessing it without the permission to edit it.
            let first: &OrderedItem = self.map.values().nth(0).expect("Rulemap is empty");
            // usize is a 0 or positive integer. u8, u16, u32, u64 are also 0-or-positive integers.
            // The suffix after u determines how many bytes this value can contain, which determines the
            // maximum possible number the value can carry before overflowing.
            // in JS this is simply a number.
            // first is an OrderedItem, we access the `pre` and `post`
            // which are HashSets (ie. Sets), and for each we get
            // their length and add them together.
            let num_relations: usize = first.pre.len() + first.post.len();

            // if all the values of the map (ie. all the OrderedItems),
            // the number of items in Pre + the number of items in Post,
            // is equal to that of the first OrderedItem in the map (num_relations):
            if self
                .map
                .values()
                .all(|v| v.post.len() + v.pre.len() == num_relations)
            {
                // print as such and return the thing itself (the rulemap)
                println!("Yes, rulemap has all relations ✅");
                self
            } else {
                // else throw an error and abort the program
                panic!("Rulemap does not have equal sized relations")
            }
        }
    }
}
