use crate::{lexer, parse};
use lexer::*;
use parse::*;

pub fn do_calc(str: String) {
    let fstr = format_string(str);
    let lexer = Lexer::new(fstr.clone());
    let mut parser = Parser::new(lexer.clone());
    let root = parser.parse();
    //println!("lexer is {:?}", parser.lexer);
    // let root: Option<Box<Tree>> = parser.parse();
    //println!("{:?}", root);
    // let mut vec: Vec<Vec<i32>> = Vec::new();

    // let (n, r) = (2, 5);
    // let mut buf = vec![0; r];
    // dfs(&mut buf, 0, 0, n);
    let n = parser.lexer.vnum as usize;
    let mut vec: Vec<bool> = vec![true; n];
    let mut vvec: Vec<Vec<bool>> = Vec::new();
    rec(&mut vvec, &mut vec, n, n);
    let tmp = parser.lexer.clone();

    for var in &tmp.vars {
        print!("|{}", var);
    }
    println!("||f|");

    for v in vvec {
        let mut cnt = 0;
        for belm in v.clone() {
            let elm;
            if belm {
                elm = 1;
            } else {
                elm = 0;
            }
            print!("|");
            for _i in 1..tmp.vars[cnt].len() {
                print!(" ");
            }
            print!("{}", elm);
            cnt += 1;
        }
        let ret = calc(v, root.clone()).expect("Null Expression");
        let f;
        if ret {
            f = 1;
        } else {
            f = 0;
        }
        println!("||{}|", f);
    }

    //calc(root);
}

fn rec(vvec: &mut Vec<Vec<bool>>, vec: &mut Vec<bool>, n: usize, len: usize) {
    if n == 0 {
        //println!("{:?}", vec.clone());
        vvec.push(vec.to_vec());
        return;
    }
    for i in [true, false] {
        vec[len - n] = i;
        rec(vvec, vec, n - 1, len);
    }
}

fn calc(vec: Vec<bool>, root: Option<Box<Tree>>) -> Option<bool> {
    use Token::*;
    match root {
        None => None,
        Some(root) => match root.token {
            Var(var) => Some(vec[var.id as usize]),
            True => Some(true),
            False => Some(false),
            Op(Operator::Not) => not(calc(vec.clone(), root.left)),
            Op(Operator::Or) => or(calc(vec.clone(), root.left), calc(vec.clone(), root.right)),
            Op(Operator::Nor) => nor(calc(vec.clone(), root.left), calc(vec.clone(), root.right)),
            Op(Operator::Xor) => xor(calc(vec.clone(), root.left), calc(vec.clone(), root.right)),
            Op(Operator::And) => and(calc(vec.clone(), root.left), calc(vec.clone(), root.right)),
            Op(Operator::Nand) => nand(calc(vec.clone(), root.left), calc(vec.clone(), root.right)),
            Op(Operator::Is) => is(calc(vec.clone(), root.left), calc(vec.clone(), root.right)),
            _ => None,
        },
    }
}

fn not(x: Option<bool>) -> Option<bool> {
    Some(!(x.expect("No argument")))
}
fn or(x: Option<bool>, y: Option<bool>) -> Option<bool> {
    Some(x.expect("No argument") | y.expect("No argument"))
}
fn nor(x: Option<bool>, y: Option<bool>) -> Option<bool> {
    Some(!(x.expect("No argument") | y.expect("No argument")))
}
fn xor(x: Option<bool>, y: Option<bool>) -> Option<bool> {
    Some(x.expect("No argument") != y.expect("No argument"))
}
fn and(x: Option<bool>, y: Option<bool>) -> Option<bool> {
    Some(x.expect("No argument") & y.expect("No argument"))
}
fn nand(x: Option<bool>, y: Option<bool>) -> Option<bool> {
    Some(!(x.expect("No argument") & y.expect("No argument")))
}
fn is(x: Option<bool>, y: Option<bool>) -> Option<bool> {
    Some(!(x.expect("No argument")) | y.expect("No argument"))
}