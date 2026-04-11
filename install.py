from os import path, makedirs, name, chdir
import shutil
import subprocess

REPO = "https://github.com/DomioKing653/Vertex.git"
DIR = "Vertex"


def run(cmd):
    print("Running:", " ".join(cmd))
    subprocess.check_call(cmd)

chdir(DIR)

run(["cargo", "build", "--bin", "vertexC", "--release"])
run(["cargo", "build", "--bin", "vertex", "--release"])

chdir("src/codegen")
run(["cargo","build","--lib","--release"])
