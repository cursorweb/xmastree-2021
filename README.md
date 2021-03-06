# Xmastree 2021
A fun christmas tree where you can add ornaments! Then, you can export the file into either:
- a rust file
- a txt file

## About
Taking a break from coding big projects

## Screenshots
![Tree](./static/tree.png)
Repl: https://replit.com/@Coder100/XMASTREE2021-Generate-cool-christmas-trees

Github: https://github.com/cursorweb/xmastree-2021

Youtube: https://youtu.be/EtKOtUh3Oz4

## Usage
```
xmastree.exe -t "tree.txt" -b 5 -o "tree.rs" -ot rs
```

* `-t`/`tree` The tree file to use
* `-b`/`base` How many lines *from the last line* is the base of the tree
    
    For example the [default tree](./src/tree.txt) has a base length of **5**.

    Defaults to 0

* `-o`/`output` The file to output to, e.g. `main.rs` `xmas.txt`

    Without this, it will be `tree.out`

* `-f`/`type` The output type, can be the following:
    * `rs`/`rust` - An rs file that literally prints the tree (does use the colorful module)
    * `txt`/`text` - A plaintext file of the resulting tree, all colors will be lost.
    * `o`/`none` - It will only be printed to the terminal! (optional)

    The tree will *always* print into the terminal!

### Movement
The `X` marker indicates your cursor. 

Use the `arrow keys` to move the cursor! It is colored based on the selected color.

Use `e` to go the previous color and use `r` to go to the next color.

Press `enter` to place the ornament down.

Press `esc` to remove an ornament the marker is on.

Press `` ` `` to reset the entire tree.

When you are done, press `q` to generate the tree!

## Credits
[@KumaPaul](https://replit.com/@KumuPaul) for getting this to work on replit!
