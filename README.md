# Selfmate finder

Running:
```console
$ cargo run --release path/to/pgn/files/ > selfmates.pgn
```

This will find all selfmates in all PGN files in the given directory and 
save the PGN of the matching games to `selfmates.pgn`.

A sample of 19 selfmates from games played by players 2400+ is included in
[selfmates.pgn](./selfmates.pgn).

# License

The source code is distributed under the terms of the WTFPLv2 license, the 
full text of which is included below:

```
           DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
                   Version 2, December 2004
 
Copyright (C) 2004 Sam Hocevar <sam@hocevar.net>

Everyone is permitted to copy and distribute verbatim or modified
copies of this license document, and changing it is allowed as long
as the name is changed.
 
           DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
  TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION

 0. You just DO WHAT THE FUCK YOU WANT TO.
```

The example games in `selfmates.pgn` come from the 
[Lichess Elite Database](https://database.nikonoel.fr/), which itself is a 
subset of the [lichess.org open database](https://database.lichess.org/), 
made available under the terms of the 
[Creative Commons CC0 license](https://tldrlegal.com/license/creative-commons-cc0-1.0-universal).
