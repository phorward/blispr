use crate::{
    error::{BlisprResult, Result},
    eval::lval_eval,
    lenv::Lenv,
    lval::{lval_add, lval_blispr, lval_num, lval_qexpr, lval_sexpr, lval_sym},
};
use pest::{iterators::Pair, Parser};

#[cfg(debug_assertions)]
const _GRAMMAR: &str = include_str!("blispr.pest");

#[derive(Parser)]
#[grammar = "blispr.pest"]
pub struct BlisprParser;

fn is_bracket_or_eoi(parsed: &Pair<Rule>) -> bool {
    if parsed.as_rule() == Rule::EOI {
        return true;
    }
    let c = parsed.as_str();
    c == "(" || c == ")" || c == "{" || c == "}"
}

fn lval_read(parsed: Pair<Rule>) -> BlisprResult {
    match parsed.as_rule() {
        Rule::blispr => {
            // a whole program is one or more expressions
            let mut ret = lval_blispr();
            for child in parsed.into_inner() {
                // here is where you skip stuff
                if is_bracket_or_eoi(&child) {
                    continue;
                }
                lval_add(&mut ret, &*lval_read(child)?)?;
            }
            Ok(ret)
        }
        Rule::expr => lval_read(parsed.into_inner().next().unwrap()),
        Rule::sexpr => {
            let mut ret = lval_sexpr();
            for child in parsed.into_inner() {
                // here is where you skip stuff
                if is_bracket_or_eoi(&child) {
                    continue;
                }
                lval_add(&mut ret, &*lval_read(child)?)?;
            }
            Ok(ret)
        }
        Rule::qexpr => {
            let mut ret = lval_qexpr();
            for child in parsed.into_inner() {
                if is_bracket_or_eoi(&child) {
                    continue;
                }
                lval_add(&mut ret, &*lval_read(child)?)?;
            }
            Ok(ret)
        }
        Rule::num => Ok(lval_num(parsed.as_str().parse::<i64>()?)),
        Rule::symbol => Ok(lval_sym(parsed.as_str())),
        _ => unreachable!(), // COMMENT/WHITESPACE etc
    }
}

pub fn eval_str(e: &mut Lenv, s: &str) -> Result<()> {
    let parsed = BlisprParser::parse(Rule::blispr, s)?.next().unwrap();
    debug!("{}", parsed);
    let mut lval_ptr = lval_read(parsed)?;
    debug!("Parsed: {:?}", *lval_ptr);
    println!("{}", lval_eval(e, &mut *lval_ptr)?);
    Ok(())
}
