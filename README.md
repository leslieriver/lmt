# lemmy Meme Theif

A lemmy community backup utility.

All images that are uploaded as the url will be downloaded. generated html file and media will be stored in a new folder in the path in which this utility is ran on. 

Currently supports; Title; body; url from every post.

Comments are not currently supported but are planned.

Usage:

```bash

cargo run <lemmyserver> <community> <limit>
```

Example command to backup memes from `lemmy.ml`

```
cargo run https://lemmy.ml memes 50
```




