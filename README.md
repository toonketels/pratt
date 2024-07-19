# Pratt parser

# Rules:

- no two tokens of same precedence can follow one another
  - if it does, parse error
  - ex: `+ +` or `100 12`
- except for terminating token `EOF`
  - because that one will never be consumed
  - we check we have terminated correctly
- alternatively, dont have an `EOF` token assert current = `None`
- start precedence is lowest, same as the `leafs` (like `number`) and `EOF`
- algo is
  - look into current:
    - should be a lhs expression (number)
    - parse it and advance
  - while current has higher precedence (parse has advanced it)
    - it means it is an binary operator
    - advance
    - rhs = parse with operator precedence
    - create binary exp (lhs, op, rhs)
    - update lhis to new binary expr
    - keep doing this if current has higher precedence
    - return lhs expr
  - done
- every parse advances current so current is ready to be inspected
- higher precedence operator will create a new node in the tree
  - with everything that came before as its lhs
- the last created node is top of the tree, evaluated first