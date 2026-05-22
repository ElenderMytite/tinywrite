# Short documentation of execution process of this program
Output of each iteration is passed to the next one: _Text_ > _Tokens_ > _ASTree_ > _Language_ > _Executing_
# Tokens(lexer > grouper)
## lexer
- groups character into meaningful tokens; 
- Has 3 modes: _None_, _Number_ and _Name_;
- reads symblos one by one, grouping them into tokens;
- Starts in normal(_None_) mode.
- Writes characters to buffer, then when knows it needs to stop copies buffer to tokens vector and clears buffer.
### When in _None_ mode:
- If a symbol is a letter or _ character, enters _Name_ mode.
- If a symbol is a digit, enters _Number_ mode.
- If a symbol is an ascii punctuation character, pushes it to tokens.
- ignores symbol otherwise
### When in _Number_ mode:
- if symbol is a digit writes to buffer
- if symbol is _ character, ignores it
- otherwise pushes buffer to tokens, clears buffer and enters _None_ mode 
### When in _Name_ mode:
- if a symbol is a letter, digit or _ character, writes to buffer,
- otherwise pushes buffer to tokens, clears buffer and enters _None_ mode
## grouper
- reads tokens one by one
if encounters two punctuation tokens in a row that match the pattern "==", "!=", "=!", "=<", "=>", "<=", ">=", "<!", ">!", "!<", "!>", "!&", "!|", "!^", ",," or"..", groups them into one token.
# ASTree (parser)
- Creates an AST from tokens to easier translating it to instructions.
- Has 2 modes: _Code_ and _Expression_
- reads tokens one by one;
- if token is not special, converts it to __Part__ (token that makes sense in expression); pushes them to the buffer;
- when encounters $ sign, reads next token and assumes that it must be a _keyword_, then flushes buffer and pushes the keyword as __Part__;
- _keywords_ are __true__, __false__, __redo__, __end__, __if__, __else__;
- When meets ( character, recursively calls itself in _Expression_ mode;
- When meets { character, recursively calls itself in _Code_ mode;
- When meets ; character, converts the buffer to node and moves it to location depending on mode in: 
  - to buffer as 1 node if in _Expression_ mode 
  - to nodes if in _Code_ mode;
- When meets ) or } character, checks if it was in the right mode, flushes buffer and returns the expected result (expression or block)
#### ___Expression___
- May contain an operation, there are lots of ___Operations___.
Records all the parts that were to the left or to the right of an operation converted them into _values_.
- _values_ may be bool, name, number or a piece of code;
# Language
## __ir (intermediate representation)__
- Creates a list of easily executable commands from AST;
- Performs a depth first search (DFS) on abstract syntax tree (AST) that pushes commands to the end of code;
- When meets code block, sequencially calls itself on each of statements in block;
- If meets a 
- When meets expression:
  - if operation is binary, associative and commutative:
    - pushes neutral element of that operation to the stack
    - converts each individual term of an expression to code to compute them and appends it to commands, 
    - inserts operation execution call between each of the terms
  - if operation is anticommutative(comparison and assignment):
    - assumes the amount of terms by left to operator is the same as to the right
    - permforms operation individually on n-th term on the left and n-th term on the right for all natural n less then half of total terms
  - __Vector__ operations:
    - __Unpack__ push all values of a vector to the stack (insert neutral element and operation in between if call from another """good"operation)
    - __Pack__ create new vector and sequencially push all elements to it
# Execution
- operates on code using instuction pointer (ip), variables(env), and stack
- executes command ___ip___, commands manipulate stack, env and ip
- increases ip by 1 if it wasn't modified by command just executed
- after each statement there's a Cls command that prints the stack and clears it;
# That's it!
