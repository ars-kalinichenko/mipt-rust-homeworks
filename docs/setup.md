# Workspace Setup

This document is to help you to set up your course workspace.

## Operating System

Course homeworks are guaranteed to compile and work in Linux and macOS. Please note that any other operating system **is not supported** through it may work natively, using Docker, WLS, or other virtualization tools.

## Setup process

### Step 1 - Installing Rust and C linker

Install `rustup` either [using the official guide](https://www.rust-lang.org/tools/install) or using your package manager such as `apt`, `pacman` or `brew`.

On Linux, you probably have the C language linker, but make sure you already have it by installing `build-essential` using your package manager.

On MacOS, users have to install XCode tools to get a C language linker.

```shell
xcode-select --install
```

Run this command to get the stable Rust compiler:

```shell
rustup update stable
```

### Step 2 - VS Code and plugins

The only supported editor in the course is [Visual Studio Code](https://code.visualstudio.com). Install it. Then, install the following plugins:

- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) - language server, your best friend.
- [Better TOML](https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml) - syntax highlight for `.toml` files.
- [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) - optional, needed for debugging.

_IDE such as CLion or editors with plugins such ad Vim will work perfectly because of Rust's nature, but the lecturer doesn't use them and won't support them officially._

### Step 3 - Cloning repository

It's not necessary, but may be convenient to create a workspace folder first:

```shell
mkdir workspace
cd workspace
```

Clone the repository:

```shell
git clone https://gitlab.com/alex.stanovoy/mipt-rust.git
cd mipt-rust
```

### Step 4 - Rover tool

We have a course assistant named `rover`. It will automatize the part of your solving routine. Go to build and install it with the command:

```shell
cargo install --path tools/rover
```

Probably, you'll need `libssl-dev` installed to build `rover`. Install it if needed.

From this moment, you can call it from any place you want! Just try to type:

```shell
rover --help
```

To uninstall it later, run this line from anyplace:

```shell
cargo uninstall rover
```

[Here](../tools/rover/README.md) you can read more information (mostly useless for a student) about `rover`.

### Step 5 - First solution

Read the document about [solving and submitting problems](solving.md). Solve the [add](problems/tutorial/add) problem.

### Step 6 - Student CI

1. Register in [testing system](https://mipt-rust.manytask.org). You can find the secret code in the course channel.
2. Clone your repository **near the course repository** in the folder named solutions:

    ```shell
    git clone YOUR_SOLUTIONS_REPOSITORY_IN_MANYTASK solutions
    ```

3. The directory structure should be:

    ```shell
    $ tree -L 1
    .
    ├── mipt-rust
    ├── solutions
    ```

4. Copy the workflow file from course repository and push it to the remote:

    ```shell
    cp mipt-rust/.grader-ci.yml solutions/.gitlab-ci.yml
    cd solutions
    git add .
    git commit -m "Add .gitlab-ci.yml"
    git push
    ```

5. Try to submit your solution to CI by running `rover submit` in the folder of the problem `add`. Feel free to ask any questions if something is wrong.
