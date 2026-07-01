# Chapter 9: Tic-Tac-Toe Arena

## Story hook

Two players, one board, three in a row wins. You watch a full match unfold — move by move — like sitting ringside at a classic console game.

## What you will build

```text
--- move 1 (X at 0) ---
 X 
   
   

--- move 5 (O at 4) ---
 X 
 O 
 X 

...
Winner: X
```

---

### Step 1 — Empty board

**Algorithm:** create nine cells; print as 3×3 grid with spaces for empty cells.

**Types:** `board` is `Vec<char>` — nine characters in row order.

**Language:** nine boxes on paper, still blank.

```rust
// Playground
fn print_board(board: &[char]) {
    for row in 0..3 {
        for col in 0..3 {
            let cell = board[row * 3 + col];
            if cell == ' ' {
                print!("   ");
            } else {
                print!(" {} ", cell);
            }
            if col < 2 {
                print!("|");
            }
        }
        println!();
        if row < 2 {
            println!("---+---+---");
        }
    }
}

fn main() {
    let board = vec![' '; 9];
    print_board(&board);
}
```

**Expected output:**

```text
   |   |   
---+---+---
   |   |   
---+---+---
   |   |   
```

> **Trust & Use:** `vec![' '; 9]` makes nine copies of `' '`. See [Trust & Use](../appendix/TRUST_AND_USE.md).

---

### Step 2 — Place one mark

**Algorithm:** start empty; write `X` at index 0; print.

**Types:** index is `usize`; mark is `char`.

**Language:** crossing the first square with an X.

```rust
// Playground
fn print_board(board: &[char]) {
    for row in 0..3 {
        for col in 0..3 {
            let cell = board[row * 3 + col];
            if cell == ' ' {
                print!("   ");
            } else {
                print!(" {} ", cell);
            }
            if col < 2 {
                print!("|");
            }
        }
        println!();
        if row < 2 {
            println!("---+---+---");
        }
    }
}

fn main() {
    let mut board = vec![' '; 9];
    board[0] = 'X';
    print_board(&board);
}
```

**Expected output:**

```text
 X |   |   
---+---+---
   |   |   
---+---+---
   |   |   
```

---

### Step 3 — Detect a row win

**Algorithm:** after each placement, check three rows, three columns, two diagonals.

**Types:** returns `Option<char>` — either the winning mark or nothing.

**Language:** scanning lines on the page for three matching letters.

```rust
// Playground
fn winner(board: &[char]) -> Option<char> {
    let lines = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];
    for line in lines {
        let a = board[line[0]];
        let b = board[line[1]];
        let c = board[line[2]];
        if a != ' ' && a == b && b == c {
            return Some(a);
        }
    }
    None
}

fn main() {
    let board = vec!['X', 'X', 'X', 'O', 'O', ' ', ' ', ' ', ' '];
    match winner(&board) {
        Some(mark) => println!("Winner: {}", mark),
        None => println!("No winner yet."),
    }
}
```

**Expected output:**

```text
Winner: X
```

> **Trust & Use:** `Some(a)` wraps a value; `None` means no winner. See [Trust & Use](../appendix/TRUST_AND_USE.md).

---

### Step 4 — Full scripted match

**Algorithm:** replay moves as `(index, mark)` pairs; print board after each; stop when someone wins.

**Types:** moves are `(usize, char)`; board is `Vec<char>`.

**Language:** a recorded game you replay move by move.

```rust
// Playground
fn print_board(board: &[char]) {
    for row in 0..3 {
        for col in 0..3 {
            let cell = board[row * 3 + col];
            if cell == ' ' {
                print!("   ");
            } else {
                print!(" {} ", cell);
            }
            if col < 2 {
                print!("|");
            }
        }
        println!();
        if row < 2 {
            println!("---+---+---");
        }
    }
}

fn winner(board: &[char]) -> Option<char> {
    let lines = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];
    for line in lines {
        let a = board[line[0]];
        let b = board[line[1]];
        let c = board[line[2]];
        if a != ' ' && a == b && b == c {
            return Some(a);
        }
    }
    None
}

fn main() {
    let mut board = vec![' '; 9];
    let moves = [(0, 'X'), (4, 'O'), (1, 'X'), (3, 'O'), (2, 'X')];

    for (i, (idx, mark)) in moves.iter().enumerate() {
        board[*idx] = *mark;
        println!("--- move {} ({}) at {} ---", i + 1, mark, idx);
        print_board(&board);
        if let Some(w) = winner(&board) {
            println!("Winner: {}", w);
            break;
        }
    }
}
```

**Expected output:**

```text
--- move 1 (X) at 0 ---
 X |   |   
---+---+---
   |   |   
---+---+---
   |   |   
--- move 2 (O) at 4 ---
 X |   |   
---+---+---
   | O |   
---+---+---
   |   |   
--- move 3 (X) at 1 ---
 X | X |   
---+---+---
   | O |   
---+---+---
   |   |   
--- move 4 (O) at 3 ---
 X | X |   
---+---+---
 O | O |   
---+---+---
   |   |   
--- move 5 (X) at 2 ---
 X | X | X 
---+---+---
 O | O |   
---+---+---
   |   |   
Winner: X
```

X takes top row — classic finish.

---

## Full chapter solution

Same as Step 4 above.

---

## See also

Next: [Chapter 10 — Snake Trail](10_snake_trail.md) — a moving snake on a grid.
