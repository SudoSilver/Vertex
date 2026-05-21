use crate::backend::{
    compiler::{
        comptime_variable_checker::comptime_value_for_check::ComptimeValueType,
        instructions::Instructions::{self, Halt},
    },
    linker::{obj_file::ObjFile, patch_objs::patch_objs_jumps, sort_objs::sort_objs_bfs},
};
use std::collections::HashMap;

pub enum SymbolType {
    Function,
    Variable,
}

pub struct GlobalSymbols {
    pub symbols: HashMap<String, Symbol>,
}

pub struct Symbol {
    pub symbol_value_type: Option<ComptimeValueType>,
    pub symbol_type: SymbolType,
    pub is_constant: bool,
    pub tag: String,
}
// NOTE: This linker is in a very early stage.
// Currently, it acts more as a concatenator of object files rather than a true symbol linker.
pub struct Linker;

impl Linker {
    // FIXME: The linker does not perform symbol resolution between different ObjFiles.
    // If multiple files import the same module, code duplication will likely occur in the final bytecode.
    pub fn link(objects: &mut Vec<ObjFile>) -> Vec<Instructions> {
        // Sort objs based on their imports (dependencies)
        let mut program: Vec<Instructions> = Vec::new();
        sort_objs_bfs(objects).unwrap();

        // Patch jump addresses to be relative to the entire program
        // NOTE: This only patches internal jumps within each ObjFile.
        // Cross-file function calls must be handled during compilation by "flattening" dependencies.
        patch_objs_jumps(&objects.to_vec(), &mut program);

        program.push(Halt); // Final Halt of a program
        program
    }
}
