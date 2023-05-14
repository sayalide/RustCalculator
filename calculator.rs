use std::io::{self, BufRead, Write};
use std::ops::{Add, Sub, Mul, Div, Neg, Not};
use std::collections::VecDeque;
use std::ops::Rem;
use std::ops::BitAnd;
use std::ops::BitOr;
use std::ops::Shr;
use std::ops::Shl;
type Stack = VecDeque<i32>;
enum StackOp 
{
   Add, Sub, Mul, Div, Rem, Neg, Not, BitAnd, BitOr, Shr, Shl, Clear, NoOp, Num(i32), 
}
fn string_parseevaluate(input: &str) -> StackOp 
{
    use StackOp::*;
    match input.trim() {
        "+" | "addition" => Add,
        "-" | "subtraction" | "subtract" => Sub,
        "*" | "multiplication" | "multiply" => Mul,
        "/" | "division" | "divide" => Div,
        "%" | "Modulas" | "remainder" => Rem,
        "neg" | "negation" | "~" => Neg,
        "not" | "not" | "!" => Not,
        "&" | "bitand" => BitAnd,
        "|" | "bitor" => BitOr,
        ">>"| "shiftright" => Shr,
        "<<"| "shiftleft" => Shl,
        "clear" | "cls" => Clear,
        "quit" | "q" | "end" => {
            println!("Program Ended Successfully, press ctrl+c");
            NoOp
        },   
        str => {
            if let Ok(Number) = str.parse::<i32>() {
                Num(Number)
            } else 
            {
                println!("ERROR while resulting!Could not parse{}", str);
                NoOp
            }
        }
    }
}
fn input_values() -> io::Result<StackOp> {
    let mut buff = String::new();
    let stdin = io::stdin();
    print!("> ");
    io::stdout().flush()?;
    stdin.lock().read_line(&mut buff)?;
    buff = buff.to_lowercase(); 
    Ok(string_parseevaluate(&buff))
}
fn binaryoperands<F>(stack: &mut Stack, fun: F)
where
    F: FnOnce(i32, i32) -> i32,
{
    if stack.len() >= 2 {
        let m = stack.pop_back().unwrap();
        let n = stack.pop_back().unwrap();
        stack.push_back(fun(n, m));
    }
}
fn stackunoperands_evaluation<F>(stack: &mut Stack, fun: F)
where
    F: FnOnce(i32) -> i32,
{
    if let Some(m) = stack.pop_back() {
        stack.push_back(fun(m));
    }
}
fn stackoperands_evaluation<F>(stack: &mut Stack, start: i32, fun: F)
where
    F: FnMut(i32, &i32) -> i32,
{
    let result = stack.iter().fold(start, fun);
    stack.clear();
    stack.push_back(result);
}
fn evaluation(stack: &mut Stack, last_op: StackOp) {
    use StackOp::*;
    match last_op {
        Add => binaryoperands(stack, i32::add),
        Sub => binaryoperands(stack, i32::sub),
        Mul => binaryoperands(stack, i32::mul),
        Div => binaryoperands(stack, i32::div),
        Rem => binaryoperands(stack, i32::rem),
        Neg   => stackunoperands_evaluation(stack, i32::neg),
        Not   => stackunoperands_evaluation(stack, i32::not),
        BitAnd => binaryoperands(stack,i32::bitand),
        BitOr => binaryoperands(stack,i32::bitor),
        Shr => binaryoperands(stack,i32::shr),
        Shl => binaryoperands(stack,i32::shl),
        Clear => stack.clear(),
        Num(n) => stack.push_back(n),
        NoOp => return, 
    }
}
fn main() -> io::Result<()> {
    println!("Enter the values and commands");
    let mut stack = VecDeque::new();
    loop {
        let input = input_values()?;
        evaluation(&mut stack, input);
        if stack.len() >= 1 {
            println!("values in stack:{:.2?}", stack);
        }
    }
}