# shrtcut

### what even is this?
a simple clipboard shortcut manager.  set shortcuts and pull them into your global clipboard.

### how do i even get it?
simple. use cargo.

it goes without saying that you will need to have rust installed on your system to use cargo, but once you do simply run:
```sh
cargo install shrtcut
```

### how to even use it?
to set shortcuts, edit your `.shrtcut.toml` file and simply add them under the `[shortcuts]` section.  as an example, to create a shortcut to Google, you could update your `[shortcuts]` block to look like:
```console
foo@bar: ~$ cat $(shrtcut --configs)
[settings]
width=300
height=40

[shortcuts]
google="google.com"
```

to pull up a gui that lets you select a shortcut from a dropdown list, simply run:
```console
foo@bar: ~$ shortcut  # No args will pull up GUI

foo@bar: ~$ shortcut --select  # Will also pull up GUI
```

the point of this program was to make it easy to select from a list of saved urls.   the plan is to include a command to start a listener loop and check for a user-specified key combination to bring up the selection gui.  for now, you should use a hotkey program (via shortcuts on windows, spark/fastscripts/etc on mac, bind on linux) and set it to run `shrtcut --select`

there are some other goodies in here that let you copy shortcuts from the terminal, add new shortcuts from the terminal, and others.  to see the full list, you can run the help command like so:
```console
foo@bar: ~$ shrtcut --help
A simple clipboard shortcut manager.

Usage: shrtcut [options] [shortcut]
    [options]:
        --help, -h
        --version, -v
        --grab, -g
        --select, -s
        --add, -a
        --configs, -c
    [shortcut]: shortcut name

Examples:
    shrtcut --help
    shrtcut --version
    shrtcut --select
    shrtcut --configs
    shrtcut --grab google
    shrtcut --add current
```

### license?
mit.

### planned work:
- write more comprehensive unit tests.
- create listener command that will listen for user-specified keypresses and open the shortcut ui when pressed so users don't need to manually map the commands in their operating system (see develop branch for this development).
- considering switching from setting shortcuts in the `.shrtcut.toml` file to being stored in an embedded db, but this would make it harder to migrate my urls to be available in `shrtcut`.  possibly store shortcuts in an embedded db and create a command to load shortcuts in from a csv/toml file.
