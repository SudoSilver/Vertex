# Installing Vertex
*NOTE:In document you will find how to build Vertex from source. If you don't wanna do it just use pre-built binaries at the github repo althrought they are just for Linux*

---

## Dependencies
- Rust programming language
- git
- Zig toolchain
## Installation process
First clone the repo and cd in to it
```bash
git clone github.com/DomioKing653/Vertex
cd Vertex
```
### Compilation
```bash
cargo build --release
cd src/codegen
cargo build --lib --release
```
### Final setup
Now move `vertex` and `vertexC` from `./target/` and `libvm_runtime.a` from `./src/codegen/target/` and put it to your enviroment variables. 
Than setup VERTEX_RUNTIME_PATH enviroment variable as path libvm_runtime.a

--- 
## Congrats now you've built Vertex from source :]
