# chess-validator (INCOMPLETE)

This is supposed to be used in conjunction with GUIs trying to communicate with the UCI protocol.

I heard that some UCIs pass the burden of checking for checkmate/stalemate to the GUI, which doesn't make sense.

I made this project to pass that burden to an executable file that any language can run.

## Usage
Have something that scans stdio, and parse that.
- Input
    - `position fen [fen-string] moves [moves, e.g: e2e3 e4e5 d5d3Q]`
    - Note: O-O is replaced by e1g1 or their respective king castle positions.
- Output
    - `res white checkmate`
    - `res black checkmate`
    - `res stalemate`
    - `res ongoing`
    - `board err`
    - `parse err`
