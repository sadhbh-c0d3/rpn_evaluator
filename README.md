# Reverse Polish Notation Expression Evaluator

Parses braced expression from standard input, and processes it in RPN fashion.

## Examples

```bash
$> cargo run
1,1,+
2
2,3,4,+,*
14
2,3,+,1,2,+,*
15
2,(1,1,1,U),*
6
(2,3,+),(5,8,*),P
200
2,3,+,(5,8,*,3,6,*,7,3,*,U),*
395
2,3,+,(5,(3,3,2,U),*),*
200
```

In the above example we can see a mix of expressions and sub-expressions using $\text{binary}$ operators $*$ and $+$ as well as $\text{n-ary}$ operators $\sum$ and $\prod$ represented as `U` and `P` ascii characters.