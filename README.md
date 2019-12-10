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

  # Task can be an object.
  echo:
    command: echo
    variables:
      sign: "!"

  # Task can be a string.
  # Task can call/alias/extend another task thanks to the "wk:" prefix.
  # Global variables and global environment variables will be used here.
  hello: wk:echo %Greeting% ${buddy}

  # Task can call/alias/extend another task thanks to the "wk:" prefix.
  # Task can depend to a list of tasks
  # Arguments can be passed to the child task
  welcome:
    command: wk:hello ${version}${sign}
    description: Say hello
    variables:
      buddy: WK
      version: 3
    environments:
      Greeting: Welcome to
    depends:
      - hello

  # Extend alternative
  # NOTE: Maybe prefer the first one, because it's cool.
  welcome_ext:
    extend: wk:hello
    args:
      - ${version}${sign}
    variables:
      buddy: WK
      version: 3
    environments:
      Greeting: Welcome to
    depends:
      - hello

  # Variable can be passed with --var.myvariable
  # args are pass before extended arguments
  how:
    command: wk:echo you${sign} --var.sign=?
    args:
      - How
      - are
    depends:
      - welcome
      - hello

  # Set shell
  how_wsl:
    command: wk:how
    shell: wsl.exe
```

## Todo

* Test units
* Pass arguments from user
* Conditional task (platform specific `macos` `win32` `unix`)
  ```yaml
  platforms:
    - win32
  commands:
    home1:
      command: echo %HOME%
    home2:
      command: echo $HOME
      platforms:
        - linux
        - osx
  ```
* Merge with global conditions
* Concurrent task
  ```yaml
  commands:
    welcome: echo Welcome
    hello: echo Hello
    echo0: wk:hello World!
    echo1: wk:hello John!
    echo2: wk:hello ${buddy}!
    echos:
      commands:
        - echo0
        - echo1
        - echo2 --var.buddy=Marc
      depends:
        - welcome
    echos_alt:
      - echo0
      - echo1
      - echo2
  ```