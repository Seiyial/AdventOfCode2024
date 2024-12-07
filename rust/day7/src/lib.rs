pub mod poss_seq {
    #[derive(PartialEq)]
    pub enum Op {
        Mul,
        Add,
    }
    impl Op {
        pub fn new(&mut self) -> Op {
            Op::Mul
        }
        pub fn next(&mut self) {
            if *self == Op::Mul {
                *self = Op::Add
            }
        }
    }
}
