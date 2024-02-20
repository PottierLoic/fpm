# Fast Project Manager

FPM is a fast project manager that allow you to set up your whole working environment with a single command.

# Usage

First, you need to create a project shortcut. To do that, you can use the fpm `new` command.
```bash
fpm new project1
```

This will create a new project and set it as the current project.  
You can now configure it with the `config` command. It can take several arguments :

### Mandatory arguments
- `--path` : Set the path to the project.

### Optional arguments
- `--editor` : Set the editor to use for this project. If not set, VSCode will be used
- `--terminal` : Set the terminal to use for this project. If not set, the basic terminal will be used
```bash
fpm config --path /path/to/project --editor nvim --terminal kitty
```

You can now simply use fpm to open your whole working environment with a single command.
```bash
fpm open
```

If you want to select another project, you can simply type
```bash
fpm project2
```