# chess-validator (INCOMPLETE)

This is supposed to be used in conjunction with GUIs trying to communicate with the UCI protocol.

I heard that some UCIs pass the burden of checking for checkmate/stalemate to the GUI, which doesn't make sense.

I made this project to pass that burden to an executable file that any language can run.

## Usage
Have something that scans stdio, and parse that.
- Input
    - `position fen [fen-string] moves [moves, e.g: e2e3 e4e5 d5d3Q]`
- Output
    - `result: stalemate`
    - `result: checkmate`
    - `result: ongoing`
    - `err: {}`
