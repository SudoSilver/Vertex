use std::collections::VecDeque;

use crate::backend::linker::obj_file::ObjFile;
use crate::backend::errors::linker::linker_errors::{LinkerError};
pub fn sort_objs(objs:Vec<ObjFile>)->Result<Vec<ObjFile>,LinkerError>{
    let mut finale_objs:VecDeque<ObjFile> = VecDeque::new();
    for obj in objs.clone(){
        if obj.imports.len() == 0 {
            finale_objs.push_back(obj.clone());
        }
    }

    Ok(finale_objs.into())
}
