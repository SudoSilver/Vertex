use std::collections::HashMap;
use crate::backend::compiler::instructions::Instructions;

pub fn push_pop_opt(code: Vec<Instructions>) -> (Vec<Instructions>,HashMap<usize,usize>) {
    let mut out:Vec<Instructions> = Vec::new();
    let mut old_to_new: HashMap<usize, usize> = HashMap::new();
    let mut i = 0;

    while i < code.len() {
        match (code.get (i),code.get(i+1)) {
            _ =>{
                old_to_new.insert(i, out.len());
                out.push(code[i].clone());
                i += 1;
            }
        }
    }
    (out,old_to_new)
}