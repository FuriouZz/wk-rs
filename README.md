# wk-rs

A task runner written to learn Rust with a minimum of dependencies. Inspired from my [Typescript version](https://github.com/wk-js/command).

`Commands.yaml`

```yaml
# Array of relative file to extend
extends:
  - ./simple2.yml

# Set environment variables shared between tasks
environments:
  Greeting: Hello

# Set variables shared between tasks
variables:
  buddy: World
  version: 0

# List commands
commands:

  # Task can be an object
  echo:
    command: echo
    variables:
      sign: "!"

  # Task can be written in one line and call another task with the "wk:" prefix.
  # Global variables and global environment variables will be used here.
  hello: wk:echo %Greeting% ${buddy}

  # Task can extend another task with "wk:" prefix.
  # Arguments can be passed to the child task
  welcome:
    command: wk:hello ${version}${sign}
    variables:
      buddy: WK
      version: 3
    environments:
      Greeting: Welcome to
    depends:
      - hello

  # Variable can be passed with --var.myvariable
  how:
    command: wk:echo you${sign} --var.sign=?
    args:
      - How
      - are
    depends:
      - welcome
      - hello
```

## Todo

* Test units
* Pass arguments from user
* Conditional task (platform specific `macos` `win32` `unix`)
* Merge with global conditions
* Concurrent task