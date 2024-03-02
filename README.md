# chess-validator (INCOMPLETE)

This is supposed to be used in conjunction with GUIs trying to communicate with the UCI protocol.

I heard that some UCIs pass the burden of checking for checkmate/stalemate to the GUI, which doesn't make sense.

I made this project to pass that burden to an executable file that any language can run.

## Usage
Have something that scans stdio, and parse that.
- Input
    - `position fen [fen-string] moves [moves, e.g: e2e3 e4e5 d5d3Q] verifymove [move]`
    - Notes: 
        - O-O and O-O-O (castling) is replaced by e1g1 or their respective king castle positions.
        - `moves` and `verifymove` are optional.
        - `moves` isn't checked if it's legal, but `verifymove` is. `verifymove` is slower than `moves`.
        - `verifymove` is evaluated first before `moves`.
- Output
    - `res white checkmate`
    - `res black checkmate`
    - `res stalemate`
    - `res ongoing`
    - `move legal`
    - `move illegal`
    - `move unknown`
    - `board err`
    - `parse err`
