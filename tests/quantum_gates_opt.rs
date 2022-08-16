use egg::{rewrite as rw, *};
use fxhash::FxHashSet as HashSet;
use std::str::FromStr;
use regex::{Regex, Captures};
use std::ops::Index;
use itertools::Itertools;

struct GateParam {
    occ: Vec<u32>,
    ctrl: Vec<u32>,
    dagger: bool,
}

// GeneralGateTrait
trait GeneralGateTrait {
    fn is_unitary(&self) -> bool;
    fn is_rotate(&self) -> bool;
    fn get_occ(&self) -> Vec<u32> {
        vec![]
    }
    fn get_ctrl(&self) -> Vec<u32>{
        vec![]
    }
    fn get_dagger(&self) -> bool{
        false
    }
    fn get_type(&self) -> String;
    fn get_params(input_str: &str) -> GateParam {
        // let r = "[X|H]\(\)"
    }
}


// impl FromStr for X {

//     type Err = ();

//     fn from_str(input: &str) -> Result<Foo, Self::Err> {
//         match input {
//             "Bar"  => Ok(Foo::Bar),
//             "Baz"  => Ok(Foo::Baz),
//             "Bat"  => Ok(Foo::Bat),
//             "Quux" => Ok(Foo::Quux),
//             _      => Err(()),
//         }
//     }
// }

struct X {
    dagger: bool,
    occ: Vec<u32>,
    ctrl: Vec<u32>,
}

impl GeneralGateTrait for X {
    fn is_unitary(&self) -> bool {
        true
    }
    fn is_rotate(&self) -> bool {
        false
    }
    fn get_type(&self) -> String {
        String::from("X")
    }
}



// impl X {
//     fn new(dagger: bool, ctrl: Vec<u32>, occ: Vec<u32>) -> X {
//         X { dagger, ctrl, occ }
//     }
// }

// struct H {
//     dagger: bool,
//     occ: Vec<u32>,
//     ctrl: Vec<u32>,
// }

// impl GeneralGateTrait for H {
//     fn is_unitary(&self) -> bool {
//         true
//     }
//     fn is_rotate(&self) -> bool {
//         false
//     }
//     fn get_type(&self) -> String {
//         String::from("H")
//     }
// }

// impl H {
//     fn new(dagger: bool, ctrl: Vec<u32>, occ: Vec<u32>) -> H {
//         H { dagger, ctrl, occ }
//     }
// }


// fn is_commute(x: Box<dyn GeneralGateTrait>, y: Box<dyn GeneralGateTrait>) -> bool {
//     // TODO
//     let not_overlap = true;
//     if not_overlap {
//         true
//     } else {
//         false
//     }
// }





define_language! {
    enum QuantumCirc {
        // Gate(Box<dyn GeneralGateTrait>),
        "*" = Mul([Id; 2]),
        "H" = HGate(Id),
        "X" = XGate(Id), 
        Symbol(egg::Symbol),
    }
}





// pub enum Math {
//     // 左边是要map的符号，右边是对应的方法，里面list是 入参类型和数量
//     "d" = Diff([Id; 2]),
//     "i" = Integral([Id; 2]),

//     "+" = Add([Id; 2]),
//     "-" = Sub([Id; 2]),
//     "*" = Mul([Id; 2]),
//     "/" = Div([Id; 2]),
//     "pow" = Pow([Id; 2]),
//     "ln" = Ln(Id),
//     "sqrt" = Sqrt(Id),

//     "sin" = Sin(Id),
//     "cos" = Cos(Id),

//     Constant(Constant),
//     Symbol(Symbol),
// }
