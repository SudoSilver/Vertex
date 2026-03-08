# Flauncher

Flauncher is build system for flare that does all linkig etc. 

## New project

Create new project using:
```bash
flauncher create my-project
```

It creates project with this structure:
```
out/
src/
 | main.flare
prj.toml
```

---

## Building project

For building you can use the depracated ```flarec build foo bar``` or the new
```bash
flauncher build
```

that build project starting from main.flare in to out/ based on ```name``` in prj.toml
