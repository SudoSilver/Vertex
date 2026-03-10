#[derive(Debug)]
pub enum LinkerError{
    CyclicImport{imported:String,from:String}
}
