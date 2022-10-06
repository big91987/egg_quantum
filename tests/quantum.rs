use std::{str::FromStr, fmt::Display};
use std::fmt;
use egg::{rewrite as rw, *};
use fxhash::FxHashSet as HashSet;
use regex::Regex;
use array_tool::vec::*;




pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[derive(Debug)]
pub struct ParseError {
    // kind: String,
    message: String,
}
 
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"Parse Error {}", self.message)
    }

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

    fn get_qubits(&self) -> Vec<u32> {
        self.get_ctrl().union(self.get_occ())
    }

    fn get_type(&self) -> String;
    fn set_param(&mut self, param_str: &str) -> Result<(), ParseError> {

        let (occ, ctrl, dag) = get_param(param_str)?;
        self.set_occ(occ);
        self.set_ctrl(ctrl);
        self.set_dag(dag);
        Ok(())
    }
    fn to_string(&self) -> String {
        let occ: String = self.get_occ().iter().map( |&x| x.to_string() + ",").collect();
        let ctrl: String = self.get_ctrl().iter().map( |&x| x.to_string() + ",").collect();
        let gate_type = self.get_type();
        // let dagger = self.get_dagger().to_string();
        let mut d = String::from("");
        if self.get_dagger() {
            d = String::from("d");
        }
        format!("{}[{};{};{}]", gate_type, occ, ctrl, d)
    }
}


// 这种每个门都要定义一堆trait的情况用宏解决
#[derive(PartialOrd, PartialEq, Debug, Clone, Eq, Ord, Hash)]
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

fn parse_string_list<T: FromStr>(s: &str, sep: &str) -> Result<Vec<T>, ParseError> {
    let mut ret:Vec<T> = vec![];
    let s_list: Vec<&str> = s.split(sep).collect();
    for i in 0..s_list.len() {
        // let a = s_list[i];
        match s_list[i].parse::<T>() {
            Ok(n) => ret.push(n),
            Err(_) => {
                // 因为不是同一个类型，所以不能？链式调用
                return Err(ParseError { 
                    message: String::from("list param format error")
                });
            },
        }
    }
    Ok(ret)
}

fn check_dagger(s: &str) -> bool {
    let s_low = s.to_lowercase();
    if (s.to_lowercase() == "y") ||  (s.to_lowercase() == "true") || (s_low == "dagger") || (s_low == "d") {
        true
    } else {
        false
    }

}

fn get_param(param_str: &str) -> Result<(Vec<u32>, Vec<u32>, bool), ParseError> {
    let mut occ_list: Vec<u32> = vec![];
    let mut ctrl_list: Vec<u32>= vec![];
    let mut dagger = false;

    let param_list: Vec<&str> = param_str.split(';').collect();

    if param_list.len() < 1 || param_list.len() > 3 {
        // panic!("gate param format error !! ==> {}", param_str)
        return Err(ParseError { 
            message: String::from("gate param format error")
        });
    } else if param_list.len() == 1 {
        occ_list = parse_string_list::<u32>(param_list[0], ",")?;
    } else if param_list.len() == 2 {
        occ_list = parse_string_list::<u32>(param_list[0], ",")?;
        ctrl_list = parse_string_list::<u32>(param_list[1], ",")?;
    } else {
        occ_list = parse_string_list::<u32>(param_list[0], ",")?;
        ctrl_list = parse_string_list::<u32>(param_list[1], ",")?;
        dagger = check_dagger(param_list[2]);
    }

    Ok((occ_list, ctrl_list, dagger))
}

fn get_gate_type_param(s: &str) -> Result<(String, String), ParseError> {

    let re = Regex::new(r"([a-zA-Z]+)\[(.*)\]").unwrap();

    if !re.is_match(s) {
        return Err(ParseError { 
            message: String::from("gate re format error")
        });
    }
    
    let captures = re.captures(s).unwrap();
    let gate_type = String::from(&captures[1]);
    let gate_param = String::from(&captures[2]);
    Ok((gate_type, gate_param))
}


impl FromStr for X {
    type Err =  ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (gate_type, gate_param) = get_gate_type_param(s)?;
        if "X" != gate_type {
            return Err(ParseError { 
                message: String::from("gate type not match")
            });
        } else {
            let mut x = X::new();
            if let Err(e) = x.set_param(&gate_param) {
                return Err(e);
            }
            Ok(x)
        }
    }
}

impl Display for X {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}",  GeneralGateTrait::to_string(self))
    }
}


#[test]
fn test_parse() {
    let test_str = "X[0,1,2;0;d]";

    let x:X = test_str.parse().unwrap();
    println!("found X gate {:?}", x);
    println!("found11 X gate {}", x);
}



// #[derive(Debug)]
#[derive(PartialOrd, PartialEq, Debug, Clone,Eq, Ord, Hash)]
pub enum Gate{
    X(X),
    I,
    // Zero,
    // Barrier,
    Other(String),
}

#[derive(PartialOrd, PartialEq, Debug, Clone,Eq, Ord, Hash)]
pub struct GateAttr {
    t: String,
    o: Vec<u32>,
    c: Vec<u32>,
    q: Vec<u32>,
    d: bool,
    // u: bool,
    // r: bool,
    // r_v: f64,
}


impl FromStr for Gate {
    type Err =  ParseError;
    fn from_str(s: &str) -> Result<Self, ParseError> {

        let (gate_type, gate_param) = get_gate_type_param(s)?;

        match gate_type.as_str() {
            "X" => {
                let mut x = X::new();
                if let Err(e) = x.set_param(&gate_param) {
                    return Err(e);
                }
                Ok(Gate::X(x))
            },
            "I" => {
                Ok(Gate::I)
            },
            _ => {
                Ok(Gate::Other(String::from(s)))
            }
        }
    }
}

impl Display for Gate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        match self {
            Gate::X(x) => {
                write!(f, "{}", GeneralGateTrait::to_string(x))
            },
            Gate::I => {
                write!(f, "I")
            },
            Gate::Other(s) => {
                write!(f, "{}", s)
            }
        }
    }
}

impl Gate {
    fn get_type(&self) -> String {
        match self {
            Gate::X(x) => {
                String::from(x.get_type())
            },
            Gate::I => {
                String::from("I")
            },
            Gate::Other(s) => {
                String::from("Other")
            },
            _ => {
                String::from("Unknown Gate")
            }
        }
    }

    fn get_occ(&self) -> Vec<u32> {
        match self {
            Gate::X(x) => {
                x.get_occ()
            },
            Gate::I => {
                vec![]
            },
            Gate::Other(s) => {
                vec![]
            },
            _ => {
                vec![]
            }
        }
    }

    fn is_unitary(&self) -> bool {
        match self {
            Gate::X(x) => {
                x.is_unitary()
            },
            Gate::I => {
                true
            },
            Gate::Other(s) => {
                false
            },
            _ => {
                false
            }
        }
    }


    fn get_qubits(&self) -> Vec<u32> {
        match self {
            Gate::X(x) => {
                x.get_qubits()
            },
            Gate::I => {
                vec![]
            },
            Gate::Other(s) => {
                vec![]
            },
            _ => {
                vec![]
            }
        }
    }
    

    fn get_ctrl(&self) -> Vec<u32> {
        match self {
            Gate::X(x) => {
                x.get_ctrl()
            },
            Gate::I => {
                vec![]
            },
            Gate::Other(s) => {
                vec![]
            },
            _ => {
                vec![]
            }
        }
    }

    fn get_dagger(&self) -> bool {
        match self {
            Gate::X(x) => {
                x.get_dagger()
            },
            Gate::I => {
                false
            },
            Gate::Other(s) => {
                false
            }
        }
    }

    fn get_attr(&self) -> GateAttr {
        GateAttr { t: self.get_type(), o: self.get_occ(), c: self.get_ctrl(), q: self.get_qubits(), d: self.get_dagger() }
    }
}


#[test]
fn test_gate_parse() {
    let test_str = "X[0,1,2;0;d]";

    let g:Gate = test_str.parse().unwrap();
    println!("found gate ==> {:?}", g);
    println!("found11  gate ==> {}", g);
}

/* ----- egg impl ----*/


define_language! {
    enum QuantumCirc {
        "*" = Mul([Id; 2]),
        Gate(Gate),
        Symbol(egg::Symbol),
    }
}

// #[derive(Default)]
pub struct QuantumGateReduce;
pub type EGraph = egg::EGraph<QuantumCirc, QuantumGateReduce>;
pub type Rewrite = egg::Rewrite<QuantumCirc, QuantumGateReduce>;

impl Analysis<QuantumCirc> for QuantumGateReduce {
    type Data = Option<(Gate, PatternAst<QuantumCirc>)>;
    fn make(egraph: &EGraph, enode: &QuantumCirc) -> Self::Data {
        // x是个闭包，将 i(class_id) 映射为 egraph[i].data[0]，即取i对应的数据
        // x(a)? 其中？ 是语法糖 将 option 或者 result的枚举类的值取出来，就是Data
        // *i是从引用取值 egraph[*i] 实际是 一个e-class的instance
        // EClass<L, D>, 其中data是一个D类型的变量 对应N::Data，在本例中D是Option<(Constant, PatternAst<Math>)> 包了一个tuple
        // as_ref 将 Option<XX> 转为 Option<&XX>
        // map 取出tuple的第一个元素
        // 因为 data是个 struct 且里面有 vec，所以没法直接 move，需要加 clone，需要优化
        let x = |i: &Id| egraph[*i].data.as_ref().map(|d| d.0.clone());
        Some(match enode {
            QuantumCirc::Gate(g) => (g.clone(), format!("{}", g).parse().unwrap()),
            QuantumCirc::Mul([a, b]) => (
                Gate::Other(String::from("WTF??")),
                format!("(* {} {})", x(a)?, x(b)?).parse().unwrap(),
            ),
            _ => return None,
        })
    }
        
    fn merge(&mut self, to: &mut Self::Data, from: Self::Data) -> DidMerge {
        merge_option(to, from, |a, b| {
            // assert_eq!(a.0, b.0, "Merged non-equal constants"); //TODO 实现 Gate == 
            DidMerge(false, false)
        })
    }
        
    fn modify(egraph: &mut EGraph, id: Id) {
        let class = egraph[id].clone();
        if let Some((c, pat)) = class.data {
            if egraph.are_explanations_enabled() {
                egraph.union_instantiations(
                    &pat,
                    &format!("{}", c).parse().unwrap(),
                    &Default::default(),
                    "quantum_gate_reduce".to_string(),
                );
            } else {
                let added = egraph.add(QuantumCirc::G(c));
                egraph.union(id, added);
            }
            // to not prune, comment this out
            egraph[id].nodes.retain(|n| n.is_leaf());

            #[cfg(debug_assertions)]
            egraph[id].assert_unique_leaves();
        }
    }
}

fn _is_commpute(v: &Gate, w: &Gate) -> bool {
    let v_attr = v.get_attr();
    let w_attr = w.get_attr();
    if v_attr.q.intersect(w_attr.q).len() > 0 {
        return false;
    }
    true
}

fn _is_occ_identity(v: &Gate, w: &Gate) -> bool {
    let a = v.get_attr().o;
    let b = w.get_attr().o;

    a.intersect(b).len() == a.len() && a.intersect(b).len() == b.len()
}


fn _is_ctrl_identity(v: &Gate, w: &Gate) -> bool {
    let a = v.get_attr().c;
    let b = w.get_attr().c;

    a.intersect(b).len() == a.len() && a.intersect(b).len() == b.len()
}


fn _is_qubit_identity(v: &Gate, w: &Gate) -> bool {
    let a = v.get_attr().q;
    let b = w.get_attr().q;

    a.intersect(b).len() == a.len() && a.intersect(b).len() == b.len()
}


fn _is_gate_identity(v: &Gate, w: &Gate) -> bool {
    _is_ctrl_identity(v, w) && _is_occ_identity(v, w) &&
        v.get_type() == w.get_type() && v.get_dagger() == w.get_dagger()
}


fn _is_gate_daggered(v: &Gate, w: &Gate) -> bool {
    _is_ctrl_identity(v, w) && _is_occ_identity(v, w) &&
        v.get_type() == w.get_type() && v.get_dagger() != w.get_dagger()
}


fn _is_cancel(v: &Gate, w: &Gate) -> bool {
    _is_gate_identity(v, w) && v.is_unitary() || _is_gate_daggered(v, w)
}

fn is_commpute(v: &str, w: &str) -> impl Fn(&mut EGraph, Id, &Subst) -> bool {
    let v = v.parse().unwrap();
    let w = w.parse().unwrap();
    move |egraph, _, subst| {
        // 如果两个 e-class重叠，则返回false
        if  egraph.find(subst[v]) == egraph.find(subst[w]) { // 两个e-class不重叠
            return false;
        }
        
        // // 一个 是 gate 一个 是 None，返回false
        // if egraph[subst[v]].data.is_some() && !egraph[subst[w]].data.is_some() {
        //     return false;
        // }
        // if egraph[subst[w]].data.is_some() && !egraph[subst[v]].data.is_some() {
        //     return false;
        // }

        let v_gate: &Gate;
        let w_gate: &Gate;

        if let Some((v_g, v_e)) = egraph[subst[v]].data.as_ref() {
            v_gate = &v_g;
        } else {
            return false;
        }

        if let Some((w_g, w_e)) = egraph[subst[v]].data.as_ref() {
            w_gate = &w_g;
        } else {
            return false;
        }
        _is_commpute(v_gate, v_gate)
    }
}


fn is_cancel(v: &str, w: &str) -> impl Fn(&mut EGraph, Id, &Subst) -> bool {
    let v = v.parse().unwrap();
    let w = w.parse().unwrap();
    move |egraph, _, subst| {
        // 如果两个 e-class重叠，则返回false
        if  egraph.find(subst[v]) == egraph.find(subst[w]) { // 两个e-class不重叠
            return false;
        }
        
        // // 一个 是 gate 一个 是 None，返回false
        // if egraph[subst[v]].data.is_some() && !egraph[subst[w]].data.is_some() {
        //     return false;
        // }
        // if egraph[subst[w]].data.is_some() && !egraph[subst[v]].data.is_some() {
        //     return false;
        // }

        let v_gate: &Gate;
        let w_gate: &Gate;

        if let Some((v_g, v_e)) = egraph[subst[v]].data.as_ref() {
            v_gate = &v_g;
        } else {
            return false;
        }

        if let Some((w_g, w_e)) = egraph[subst[v]].data.as_ref() {
            w_gate = &w_g;
        } else {
            return false;
        }
        _is_cancel(v_gate, v_gate)
    }
}



// 用来分析language， 这段有特殊需求在需要实现 比如 公式中存在常量的时候，比如 5 * （2 + a） + 4 - d
// pub type EGraph = egg::EGraph<QuantumCirc, QuantumGateReduce>;
// pub type Rewrite = egg::Rewrite<QuantumCirc, QuantumGateReduce>;
// #[derive(Default)]
// pub struct QuantumGateReduce;
// impl Analysis<QuantumCirc> for QuantumGateReduce {
//     type Data = Option<(Gate, PatternAst<QuantumCirc>)>;

//     fn make(egraph: &EGraph, enode: &QuantumCirc) -> Self::Data {
//         // x是个闭包，将 i(class_id) 映射为 egraph[i].data[0]，即取i对应的数据
            // x(a)? 其中？ 是语法糖 将 option 或者 result的枚举类的值取出来，就是Data
            // *i是从引用取值 egraph[*i] 实际是 一个e-class的instance
            // EClass<L, D>, 其中data是一个D类型的变量 对应N::Data，在本例中D是Option<(Constant, PatternAst<Math>)> 包了一个tuple
            // as_ref 将 Option<XX> 转为 Option<&XX>
            // map 取出tuple的第一个元素
//         let x = |i: &Id| egraph[*i].data.as_ref().map(|d| d.0);
//         Some(match enode {
//             QuantumCirc::Gate(g) => (*g, format!("{}", g).parse().unwrap()),
//             // QuantumCirc::Mul([a, b]) => (
//             //     x(a)? * x(b)?,
//             //     format!("(* {} {})", x(a)?, x(b)?).parse().unwrap(),
//             // ),
//             _ => return None,
//         })
//     }

//     fn merge(&mut self, to: &mut Self::Data, from: Self::Data) -> DidMerge {
//         merge_option(to, from, |a, b| {
//             assert_eq!(a.0, b.0, "Merged non-equal constants");
//             DidMerge(false, false)
//         })
//     }

//     fn modify(egraph: &mut EGraph, id: Id) {
//         let class = egraph[id].clone();
//         if let Some((c, pat)) = class.data {
//             if egraph.are_explanations_enabled() {
//                 egraph.union_instantiations(
//                     &pat,
//                     &format!("{}", c).parse().unwrap(),
//                     &Default::default(),
//                     "constant_fold".to_string(),
//                 );
//             } else {
//                 let added = egraph.add(Math::Constant(c));
//                 egraph.union(id, added);
//             }
//             // to not prune, comment this out
//             egraph[id].nodes.retain(|n| n.is_leaf());

//             #[cfg(debug_assertions)]
//             egraph[id].assert_unique_leaves();
//         }
//     }
// }
