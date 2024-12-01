pub mod model {
    #[derive(PartialEq, Debug)]
    pub enum Next {
        M,
        U,
        L,
        Open,
        Num1_1,
        Num1_2,
        Num1_3,
        Comma,
        Num2_1,
        Num2_2,
        Num2_3,
        Close,
    }
}

pub mod debugger {
    use crate::model::Next;

    pub fn print_vals(char: &char, next: &Next) {
        if *next != Next::M {
            print!(
                "{}",
                if *char == 'u' {
                    "mu".to_string()
                } else {
                    char.to_string()
                },
                // next
            )
        };
    }
}
