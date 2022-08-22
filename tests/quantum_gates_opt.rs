use std::str::FromStr;

// use egg::{rewrite as rw, *};
// use fxhash::FxHashSet as HashSet;
// use std::str::FromStr;
use regex::Regex;

pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

// GeneralGateTrait
trait GeneralGateTrait {
    fn is_unitary(&self) -> bool;
    fn is_rotate(&self) -> bool;
    fn get_occ(&self) -> Vec<u32> {
        vec![]
    }
    fn set_occ(&mut self, occ: Vec<u32>);
    fn get_ctrl(&self) -> Vec<u32>{
        vec![]
    }
    fn set_ctrl(&mut self, ctrl: Vec<u32>);
    fn get_dagger(&self) -> bool{
        false
    }
    fn set_dag(&mut self, dag:bool);

    fn get_type(&self) -> String;
    fn set_param(&mut self, param_str: &str) {
        let (occ, ctrl, dag) = get_param(param_str);
        self.set_occ(occ);
        self.set_ctrl(ctrl);
        self.set_dag(dag);

    }
    // fn to_string(&self) -> String {

    // }
}


// // trait RotateGateTrait {
// //     fn get_angle(&self) -> f32;
// //     fn set_angle(&mut self);
// // }

// 这种每个门都要定义一堆trait的情况用宏解决

#[derive(Debug)]
pub struct X {
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
    fn get_occ(&self) -> Vec<u32> {
        self.occ.clone()
    }
    fn get_ctrl(&self) -> Vec<u32> {
        self.ctrl.clone()
    }
    fn get_dagger(&self) -> bool {
        self.dagger
    }
    fn set_ctrl(&mut self, ctrl: Vec<u32>) {
        self.ctrl = ctrl;
    }
    fn set_dag(&mut self, dag:bool) {
        self.dagger = dag;
    }
    fn set_occ(&mut self, occ: Vec<u32>) {
        self.occ = occ;
    }

    fn set_param(&mut self, param_str: &str) {
        let (occ, ctrl, dag) = get_param(param_str);
        self.set_occ(occ);
        self.set_ctrl(ctrl);
        self.set_dag(dag);

    }
}


impl X {
    fn new() -> Self {
        X {
            occ: vec![],
            ctrl: vec![],
            dagger: false,
        }
    }
    fn form_str(param_str: &str) -> Self {
        let mut x = Self::new();
        x.set_param(param_str);
        x
    }
}

// // fn gate_from_str<T: GeneralGateTrait>(s: &str) {
    
// // }

pub enum Gate{
    X(X),
    Other,
}

fn parse_string_list<T: FromStr>(s: &str, sep: &str) -> Vec<T> {
    let mut ret:Vec<T> = vec![];
    let s_list: Vec<&str> = s.split(sep).collect();
    for i in 0..s_list.len() {
        let a = s_list[i];
        match s_list[i].parse::<T>() {
            Ok(n) => ret.push(n),
            Err(e) => {
                // print!();
                panic!("s {} ===> {}", i, s_list[i])
            },
          }
    }
    ret
}

fn check_dagger(s: &str) -> bool {

    let s_low = s.to_lowercase();
    if (s.to_lowercase() == "y") ||  (s.to_lowercase() == "true") || (s_low == "dagger") || (s_low == "d") {
        true
    } else {
        false
    }

}

fn get_param(param_str: &str) -> (Vec<u32>, Vec<u32>, bool) {
    let mut occ_list: Vec<u32> = vec![];
    let mut ctrl_list: Vec<u32>= vec![];
    let mut dagger = false;

    let param_list: Vec<&str> = param_str.split(';').collect();

    if param_list.len() < 1 || param_list.len() > 3 {
        panic!("gate param format error !! ==> {}", param_str)
    } else if param_list.len() == 1 {
        occ_list = parse_string_list::<u32>(param_list[0], ",");
    } else if param_list.len() == 2 {
        occ_list = parse_string_list::<u32>(param_list[0], ",");
        ctrl_list = parse_string_list::<u32>(param_list[1], ",");
    } else {
        occ_list = parse_string_list::<u32>(param_list[0], ",");
        ctrl_list = parse_string_list::<u32>(param_list[1], ",");
        dagger = check_dagger(param_list[2]);
    }

    (occ_list, ctrl_list, dagger)
}

fn get_gate_type_param(s: &str) -> (String, String) {

    let re = Regex::new(r"([a-zA-Z]+)\[(.*)\]").unwrap();

    if !re.is_match(s) {
        panic!("parse error");
    }
    
    let captures = re.captures(s).unwrap();
    let gate_type = String::from(&captures[1]);
    let gate_param = String::from(&captures[2]);
    (gate_type, gate_param)
}


#[test]
fn test_re() {
    let test_str = "H[0,1,2;0;d]";

    println!("input test_str ==> {}", test_str);
    // let (gate_type, gate_param, occ, ctrl, dag) = gate_info_str(test_str);


    // println!("gate_type ==> {:?}", gate_type);
    // println!("gate_param ==> {:?}", gate_param);
    // println!("occ ==> {:?}", occ);
    // println!("ctrl => {:?}", ctrl);
    // println!("dag ==> {:?}", dag);
}


fn parse_gate(s: &str) -> Gate {
    let (gate_type, gate_param) = get_gate_type_param(s);
    println!("gate_type ==> {}", gate_type);
    println!("gate_param==> {}", gate_param);
    match gate_type.as_str() {
        "X"=> Gate::X(X::form_str(gate_param.as_str())),
        _ => Gate::Other,
    }
}

#[test]
fn test_pare_gate() {
    let test_str = "X[0,1,2;0;d]";

    let g = parse_gate(test_str);

    match g {
        Gate::X(x) => {
            println!("found X gate {:?}", x)
        },
        Gate::Other => {
            println!("found Other")
        }
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





// define_language! {
//     enum QuantumCirc {
//         // Gate(Box<dyn GeneralGateTrait>),
//         "*" = Mul([Id; 2]),
//         "H" = HGate(Id),
//         "X" = XGate(Id), 
//         Symbol(egg::Symbol),
//     }
// }





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
