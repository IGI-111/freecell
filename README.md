# Freecell

Yet another implementation of the legendary total information solitaire.
Play patience like it's 1991, complete with sights and sounds.

![game](https://user-images.githubusercontent.com/7190144/130370493-4763e2f1-5e6e-4dd0-9e61-888e4203fb5d.gif)

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

The goal of the game is to move the four aces, as they appear, to the foundations (the four slots in the top left), and build each up in suit from ace to king (A-2-3-4-5-6-7-8-9-10-J-Q-K), with the help of the four free cells (the four slots in the top right).

Only the top card or tableau of each cascade is available for play. It may be moved to a foundation pile, a free cell, or to another tableau pile. Within the tableau, cards are built down in sequence and alternating in color. Any card may be moved into an empty space. Blocks of cards may not be moved, unless the requisite number of free cells and/or tableau spaces are availabe to allow each individual card to be moved. If you fill all four foundation piles, you win.

Move cards or tableaux with drag and drop, right click cards to send them directly to the foundations.
