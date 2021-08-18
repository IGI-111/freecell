# Freecell

Yet another implementation of the legendary total information solitaire.


## Build

This game uses ggez so on Linux you'll want to install `libasound2`, `libudev` and `pkg-config` with something like: 

```
apt install libasound2-dev libudev-dev pkg-config
```

and then you should be able to simply:

```
cargo run
```

## Play

Move cards with drag and drop, right click cards to send them directly to the foundations.
