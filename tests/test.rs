use egg::{rewrite as rw, *};
use ordered_float::NotNan;

// 第一个是枚举类， 第二个用来分析语法的结构
pub type EGraph = egg::EGraph<Math, ConstantFold>;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;

pub type Constant = NotNan<f64>;


// fn print_type_of<T>(_: &T) {

//     println!("{}", std::any::type_name::<T>())

// } 


fn make_a_string(var: &str) -> String {
    String::from("1111")
}

fn make_a_str(var: &str) -> &str {
    var
}

define_language! {
    pub enum Math {
        // 左边是要map的符号，右边是对应的方法，里面list是 入参类型和数量
        "d" = Diff([Id; 2]),
        "i" = Integral([Id; 2]),

        // "+" = Add([Id; 2]),
        // "-" = Sub([Id; 2]),
        "*" = Mul([Id; 2]),
        "/" = Div([Id; 2]),
        // "pow" = Pow([Id; 2]),
        // "ln" = Ln(Id),
        // "sqrt" = Sqrt(Id),

        // "sin" = Sin(Id),
        // "cos" = Cos(Id),

        Constant(Constant),
        Symbol(Symbol),
    }
}

// You could use egg::AstSize, but this is useful for debugging, since
// it will really try to get rid of the Diff operator
// 用来计算代价
pub struct MathCostFn;
impl egg::CostFunction<Math> for MathCostFn {
    type Cost = usize;
    fn cost<C>(&mut self, enode: &Math, mut costs: C) -> Self::Cost
    where
        C: FnMut(Id) -> Self::Cost,
    {
        let op_cost = match enode {
            Math::Diff(..) => 100,
            Math::Integral(..) => 100,
            _ => 1,
        };
        enode.fold(op_cost, |sum, i| sum + costs(i))
    }
}

// 用来分析language
#[derive(Default)]
pub struct ConstantFold;
impl Analysis<Math> for ConstantFold {
    type Data = Option<(Constant, PatternAst<Math>)>;

    fn make(egraph: &EGraph, enode: &Math) -> Self::Data {
        // x是个闭包，将 i 映射为 egraph[i].data[0]，即取i对应的数据
        let x = |i: &Id| egraph[*i].data.as_ref().map(|d| d.0);
        Some(match enode {
            Math::Constant(c) => (*c, format!("{}", c).parse().unwrap()),
            // Math::Add([a, b]) => (
            //     x(a)? + x(b)?,
            //     format!("(+ {} {})", x(a)?, x(b)?).parse().unwrap(),
            // ),
            // Math::Sub([a, b]) => (
            //     x(a)? - x(b)?,
            //     format!("(- {} {})", x(a)?, x(b)?).parse().unwrap(),
            // ),
            Math::Mul([a, b]) => (
                x(a)? * x(b)?,
                format!("(* {} {})", x(a)?, x(b)?).parse().unwrap(),
            ),
            // Math::Div([a, b]) if x(b) != Some(NotNan::new(0.0).unwrap()) => (
            //     x(a)? / x(b)?,
            //     format!("(/ {} {})", x(a)?, x(b)?).parse().unwrap(),
            // ),
            Math::Div([a, b]) => (
                x(a)? / x(b)?,
                format!("(/ {} {})", x(a)?, x(b)?).parse().unwrap(),
            ),
            _ => return None,
        })
    }

    fn merge(&mut self, to: &mut Self::Data, from: Self::Data) -> DidMerge {
        merge_option(to, from, |a, b| {
            assert_eq!(a.0, b.0, "Merged non-equal constants");
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
                    "constant_fold".to_string(),
                );
            } else {
                let added = egraph.add(Math::Constant(c));
                egraph.union(id, added);
            }
            // to not prune, comment this out
            egraph[id].nodes.retain(|n| n.is_leaf());

            #[cfg(debug_assertions)]
            egraph[id].assert_unique_leaves();
        }
    }
}

// fn is_const_or_distinct_var(v: &str, w: &str) -> impl Fn(&mut EGraph, Id, &Subst) -> bool {
//     let v = v.parse().unwrap();
//     let w = w.parse().unwrap();
//     move |egraph, _, subst| {
//         egraph.find(subst[v]) != egraph.find(subst[w])
//             && (egraph[subst[v]].data.is_some()
//                 || egraph[subst[v]]
//                     .nodes
//                     .iter()
//                     .any(|n| matches!(n, Math::Symbol(..))))
//     }
// }

// fn is_const(var: &str) -> impl Fn(&mut EGraph, Id, &Subst) -> bool {
//     let var = var.parse().unwrap();
//     move |egraph, _, subst| egraph[subst[var]].data.is_some()
// }


fn fooo(aaa: &str) -> &str{
    aaa
}

fn is_sym(var: &str) -> impl Fn(&mut EGraph, Id, &Subst) -> bool {
    let var = var.parse().unwrap();
    // println!("2222222222222222222222");
    // println!("22var: {} ", var);
    // 转移所有权
    move |egraph, _, subst| {
        
        // // subst[var] ======> e-class id
        // // egraph[subst[var]] ===> spec class
        // println!("var2: {} ", var);
        // // print_type_of(var);
        // println!("222????subst: {:?}", subst);
        // println!("222????subst[var]: {:?}", subst[var]);
        // println!("222?????egraph: {:?} ", &egraph);
        // println!("222?????egraph[subst[var]]: {:?} ", &egraph[subst[var]]);
        // println!("222?????&egraph[subst[var]].nodes: {:?} ", &egraph[subst[var]].nodes);
        
        // 如果这个 class中的node有一个是 symbol，则返回true
        egraph[subst[var]]
            .nodes
            .iter()
            .any(|n| matches!(n, Math::Symbol(..)))
    }
}



fn is_gate(var: &str) -> impl Fn(&mut EGraph, Id, &Subst) -> bool {
    let var = var.parse().unwrap();
    // println!("2222222222222222222222");
    // println!("22var: {} ", var);
    // 转移所有权
    move |egraph, _, subst| {
        
        // // subst[var] ======> e-class id
        // // egraph[subst[var]] ===> spec class
        // println!("var2: {} ", var);
        // // print_type_of(var);
        // println!("222????subst: {:?}", subst);
        // println!("222????subst[var]: {:?}", subst[var]);
        // println!("222?????egraph: {:?} ", &egraph);
        // println!("222?????egraph[subst[var]]: {:?} ", &egraph[subst[var]]);
        // println!("222?????&egraph[subst[var]].nodes: {:?} ", &egraph[subst[var]].nodes);
        
        // 如果这个 class中的node有一个是 symbol，则返回true
        // egraph[subst[var]]
        //     .nodes
        //     .iter()
        //     .any(|n| matches!(n, Math::Symbol(..)))

        for aa in egraph[subst[var]].nodes.iter() {
            
        }
    }
}

// 返回判别函数
fn is_not_zero(var: &str) -> impl Fn(&mut EGraph, Id, &Subst) -> bool {
    println!("11111111111111111111111");
    // print_type_of(var);

    // subst 一个map，用于讲 ?a 替换为 某一个 e-node id，该 id的映射位于 eclass的memo中；
    let var = var.parse().unwrap(); //将字符串转为对象, 根据下面引用的type推断 var parse后的type
    println!("var: {} ", var);
    // print_type_of(var);
    // 关键字move的作用是将所引用的变量的所有权转移至闭包内
    // Pattern
    move |egraph, _, subst| {
        println!("var2: {} ", var); 
        // print_type_of(var);
        println!("????subst: {:?}", subst);
        println!("????subst[var]: {:?}", subst[var]); // ===> e_node id
        // println!("??? egraph.memo: {:?}", egraph.memo);
        // println!("??? egraph.memo[subst[var]]: {:?}", egraph.memo[subst[var]]);
        // println!("?????egraph: {:?} ", &egraph);
        // println!("?????egraph[subst[var]]: {:?} ", &egraph[subst[var]]);
        // println!("?????&egraph[subst[var]].nodes: {:?} ", &egraph[subst[var]].nodes);
        // println!("?????&egraph[subst[var]].nodes: {:?} ", &egraph[subst[var]].nodes);
        if let Some(n) = &egraph[subst[var]].data {
            println!("n.0: {} ", *(n.0));
            *(n.0) != 0.0
        } else {
            true
        }
    }
}

#[rustfmt::skip]
pub fn rules() -> Vec<Rewrite> { vec![
    // rw!("comm-add";  "(+ ?a ?b)"        => "(+ ?b ?a)"),
    rw!("comm-mul";  "(* ?a ?b)"        => "(* ?b ?a)"),
    // rw!("assoc-add"; "(+ ?a (+ ?b ?c))" => "(+ (+ ?a ?b) ?c)"),
    // rw!("assoc-mul"; "(* ?a (* ?b ?c))" => "(* (* ?a ?b) ?c)"),

    // rw!("sub-canon"; "(- ?a ?b)" => "(+ ?a (* -1 ?b))"),
    // rw!("div-canon"; "(/ ?a ?b)" => "(* ?a (pow ?b -1))" if is_not_zero("?b")),
    // rw!("canon-sub"; "(+ ?a (* -1 ?b))"   => "(- ?a ?b)"),
    // rw!("canon-div"; "(* ?a (pow ?b -1))" => "(/ ?a ?b)" if is_not_zero("?b")),

    // rw!("zero-add"; "(+ ?a 0)" => "?a"),
    rw!("zero-mul"; "(* ?a 0)" => "0"),
    rw!("one-mul";  "(* ?a 1)" => "?a"),

    // rw!("add-zero"; "?a" => "(+ ?a 0)"),
    rw!("mul-one";  "?a" => "(* ?a 1)"),

    // rw!("cancel-sub"; "(- ?a ?a)" => "0"),
    rw!("cancel-div"; "(/ ?a ?a)" => "1" if is_not_zero("?a")),

    // rw!("cancel-div"; "(/ ?a ?a)" => "foo(?a)" if is_not_zero("?a")),

    // rw!("distribute"; "(* ?a (+ ?b ?c))"        => "(+ (* ?a ?b) (* ?a ?c))"),
    // rw!("factor"    ; "(+ (* ?a ?b) (* ?a ?c))" => "(* ?a (+ ?b ?c))"),

    // rw!("pow-mul"; "(* (pow ?a ?b) (pow ?a ?c))" => "(pow ?a (+ ?b ?c))"),
    // rw!("pow0"; "(pow ?x 0)" => "1"
    //     if is_not_zero("?x")),
    // rw!("pow1"; "(pow ?x 1)" => "?x"),
    // rw!("pow2"; "(pow ?x 2)" => "(* ?x ?x)"),
    // rw!("pow-recip"; "(pow ?x -1)" => "(/ 1 ?x)"
    //     if is_not_zero("?x")),
    // rw!("recip-mul-div"; "(* ?x (/ 1 ?x))" => "1" if is_not_zero("?x")),

    rw!("d-variable"; "(d ?x ?x)" => "1" if is_sym("?x")),

    // rw!("d-variable"; "(d ?x ?x)" => make_a_str("?x") if is_sym("?x")),
    // rw!("d-constant"; "(d ?x ?c)" => "0" if is_sym("?x") if is_const_or_distinct_var("?c", "?x")),

    // rw!("d-add"; "(d ?x (+ ?a ?b))" => "(+ (d ?x ?a) (d ?x ?b))"),
    // rw!("d-mul"; "(d ?x (* ?a ?b))" => "(+ (* ?a (d ?x ?b)) (* ?b (d ?x ?a)))"),

    // rw!("d-sin"; "(d ?x (sin ?x))" => "(cos ?x)"),
    // rw!("d-cos"; "(d ?x (cos ?x))" => "(* -1 (sin ?x))"),

    // rw!("d-ln"; "(d ?x (ln ?x))" => "(/ 1 ?x)" if is_not_zero("?x")),

    // rw!("d-power";
    //     "(d ?x (pow ?f ?g))" =>
    //     "(* (pow ?f ?g)
    //         (+ (* (d ?x ?f)
    //               (/ ?g ?f))
    //            (* (d ?x ?g)
    //               (ln ?f))))"
    //     if is_not_zero("?f")
    //     if is_not_zero("?g")
    // ),

    // rw!("i-one"; "(i 1 ?x)" => "?x"),
    // rw!("i-power-const"; "(i (pow ?x ?c) ?x)" =>
    //     "(/ (pow ?x (+ ?c 1)) (+ ?c 1))" if is_const("?c")),
    // rw!("i-cos"; "(i (cos ?x) ?x)" => "(sin ?x)"),
    // rw!("i-sin"; "(i (sin ?x) ?x)" => "(* -1 (cos ?x))"),
    // rw!("i-sum"; "(i (+ ?f ?g) ?x)" => "(+ (i ?f ?x) (i ?g ?x))"),
    // rw!("i-dif"; "(i (- ?f ?g) ?x)" => "(- (i ?f ?x) (i ?g ?x))"),
    // rw!("i-parts"; "(i (* ?a ?b) ?x)" =>
    //     "(- (* ?a (i ?b ?x)) (i (* (d ?x ?a) (i ?b ?x)) ?x))"),
]}


egg::test_fn! {
    math_1111, rules(),
    "(/ H[0,1] H[0,1])"
    =>
    "1"
}







egg::test_fn! {
    math_2222, rules(),
    "(/ 123 123)"
    =>
    "1"
}


egg::test_fn! {
    math_3333, rules(),
    "(d H[0,1] H[0,1])"
    =>
    "1"
}


egg::test_fn! {
    math_4444, rules(),
    "(d 0.01 0.01)"
    =>
    "1"
}



#[test]
fn test_123() {
    let res = rules();
    let start = "(/ H[0,1] H[0,1])".parse().unwrap();
    let end = "1".parse().unwrap();
    let mut runner = Runner::default().with_explanations_enabled().with_expr(&start).run(&res);
    
    println!("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
    println!("{:?}", runner.explain_equivalence(&start, &end).get_flat_strings());
}
