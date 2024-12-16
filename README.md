# Boolean Expression Interpreter

This repository contains a simple boolean expression interpreter written in Rust.

It is a solution to this [LeetCode problem](https://leetcode.com/problems/parsing-a-boolean-expression/).

But really, it is just an excuse for me to learn about lexers, parsers, ASTs, and how interpreters work.

## BNF for Boolean Expressions

```bnf
<boolean_constant> ::= "t" | "f"
<not_expression>   ::= "!" "(" <expression> ")"
<and_expression>   ::= "&" "(" <expression> {"," <expression> }* ")"
<or_expression>    ::= "|" "(" <expression> {"," <expression> }* ")"
<expression>       ::= <boolean_constant> | <not_expression> | <and_expression> | <or_expression>
```
