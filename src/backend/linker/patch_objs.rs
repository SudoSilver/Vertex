use crate::backend::compiler::instructions::Instructions;
use crate::backend::linker::obj_file::ObjFile;

// WARN: This function assumes all Jump instructions are absolute and internal to the ObjFile.
// It simply offsets them by the current position in the global program.
// FIXME: If cross-file jumps are ever introduced without being resolved at compile-time,
// this logic will break them.
pub fn patch_objs_jumps(sorted_objects: &Vec<ObjFile>, program: &mut Vec<Instructions>) {
    let mut offset: usize = 0;
    for obj in sorted_objects {
        let mut patched = Vec::new();

        for instr in &obj.instructions {
            let new_instr = match instr {
                Instructions::Jump(addr) => Instructions::Jump(addr + offset),

                Instructions::JumpIfTrue(addr) => Instructions::JumpIfTrue(addr + offset),

                Instructions::JumpIfFalse(addr) => Instructions::JumpIfFalse(addr + offset),

                other => other.clone(),
            };

            patched.push(new_instr);
        }

        // NOTE: Inefficiently cloning and extending.
        // Could be optimized by using a pre-allocated vector and modifying instructions in-place.
        offset += patched.len();
        program.extend(patched);
    }
}
