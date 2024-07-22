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
- what is significant
  - knowing you are in the lhs expression
    - should only match tokens that can be in that position
    - advance from there
- add infix by lhs = this.parse_expression
  - even works with same tokens
  - depends on token position
  - lhs or rhs
    - rhs: will do an advance and then rhs=this.parse_expression
- same level of precedence =>
  - ((a + b) + c)
    1. makes node left op right
    2. proceeds
- higher level of precedence
  - (a + (b * c))
    1. start making node left op 
    2. instead of finishing with node right it creates a new node with higher precedence: left op right
    3. finishing original node with right = new node
- grouping:
  - both `(` and `)` have 0 precedence
  - `(` is evalutated in rhs position, advances
  - forces evaluation with `0` precedence which means "just start evaluating from scratch within here"
  - after done (because closing is `0` precedence, not evaluated too), expexts `)` and advances
  - key idea: grouping forces evaluation from scratch in the innter group + matches next token is `)`