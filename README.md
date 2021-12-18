# Xmastree 2021
A fun christmas tree where you can add ornaments! Then, you can export the file into either:
- a rust file
- a txt file

## About
Taking a break from coding big projects

## Screenshots

## Usage
```
xmastree.exe -t "tree.txt" -b 5 -o "tree.rs" -ot rs
```

* `-t`/`tree` The tree file to use
* `-b`/`base` How many lines *from the last line* is the base of the tree
    
    For example the [default tree](./src/tree.txt) has a base length of **5**.

    Defaults to 0

* `-o`/`output` The file to output to, e.g. `main.rs` `xmas.txt`
* `-f`/`type` The output type, can be the following:
    * `rs` - An rs file that literally prints the tree (does use the colorful module)
    * `text` - A plaintext file of the resulting tree, all colors will be lost.

    Without this argument, the tree will print into the terminal!

### Movement
The `X` marker indicates your cursor. 

Use the `arrow keys` to move the cursor! It is colored based on the selected color.

Use numbers `1-9` to change the color!

Press `enter` to place the ornament down.

Press `esc` to remove an ornament the marker is on.

Press `ctrl` to reset the entire tree.

When you are done, press `q` to generate the tree!