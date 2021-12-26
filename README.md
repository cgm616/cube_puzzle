`cube_puzzle`
==========

A brute-force solver for a specific cube puzzle, written in Rust.
Not well-commented, but could be useful for someone who wants to build something similar.

## The puzzle

We are given ten puzzle pieces made of five cubes each.
Each cube is either black or white, and the cubes are glued together face-to-face.
Every piece fits into a 3x2x2 rectangle.
For example, here is one piece:

```
bottom:   top:
 BW_       _B_
 _BW       ___
```

(We are looking down through one of the 2-deep dimensions.
`bottom` shows the slice on—you guessed it—the bottom, and likewise for `top`.)
The goal is to fit every piece together to completely fill a 5x5x2 space with no gaps, pieces left out, or cubes extending past the space.
The resulting block of wood must be follow a checkerboard pattern, so cubes of the same color may not have adjacent faces.

### Minutiae

There's a few more things.
In the physical puzzle, each piece is made of wood, and each "cube" is an individual piece of wood that has been glued to four others.
The creator of the puzzle arranged the cubes in each piece such that their grain is parallel.
While I didn't go into this _knowing_ the grains would match when put together, I thought it was a fair enough guess to cut down on the search space.
If my guess is correct, then pieces cannot be rotated 90 or 270 degrees about one of their axes.
Also, half of the pieces cannot be rotated 90 or 270 degrees about the other.
This hunch turned out to be right.

## The solver

I built the brute-force solver because neither my family nor I could figure out a faster algorithm (i.e. one possible for humans) to solve the puzzle.
The solver simply tries to place each puzzle piece in the first place possible, backtracking when it becomes no longer possible to place the remaining pieces.
It stops when the 5x5x2 board has no more gaps and the spaces are properly checkered.
(Currently, the code would panic and abort if it finds a non-checkered arrangement that fills all gaps (possible!).
Since I've already found the correct arrangement, I'm probably not going to fix it.)

[Rust](https://www.rust-lang.org/) was my language of choice for this project because 1) I enjoy programming in Rust, and 2) it is well suited to this sort of problem.
For starters, it's fast, and I wasn't sure how long it would take to try all of the arrangements.
More importantly, it has great support for well-structured data.
I liberally used Rust `enum`s and wrapper `struct`s to help me keep track of each puzzle piece, and Rust's `Result`-based error-handling pattern matched well with backtracking.
I also used Rust's amazing support for unit tests to debug my puzzle piece-rotating code, which was bug-ridden at first.

### Running it

There are no dependencies!
Make sure you've installed Rust and clone the repo.
Then a simple `cargo run --release` will run the solver for you.
