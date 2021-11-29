# lemmy Meme Theif

A lemmy community backup utility.

All images uploaded via lemmy is downloaded. A single html file `index.html` is generated to view the content in the directory this utility is ran in. 

Currently supports; Title; body; url from every post.

Comments are not currently supported but are planned.

Usage:

```bash

cargo run <lemmyserver> <community>
```

Example command to backup meme from `lemmy.ml`

```
cargo run https://lemmy.ml memes
```

Example command to backup fren world from wolfballs

```
cargo run https://wolfballs.com frenworld
```



