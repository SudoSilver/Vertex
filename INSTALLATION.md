# Installing Vertex
*NOTE:In document you will find how to build Vertex from source. If you don't wanna do it just use pre-built binaries at the github repo althrought they are just for Linux*

---

You will use install.py skript in the root of this project
## Dependencies
- Python 3 or greater
- Rust programming language
- git
## Installation process
First clone the repo and cd in to it
```bash
git clone github.com/DomioKing653/Vertex
cd Vertex
```

now run the install.py skript:
```bash
python install.py
```
--- 
the final build should be in ./target/release/ and the runtime should be in ```src/codegen/libvm_runtime.a```

# Congrats you have just build Vertex from source
