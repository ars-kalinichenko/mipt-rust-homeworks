# Persistent stack

In this problem, you'll write a simple persistent stack.

## Task

A persistent stack is a stack where `push` and `pop` do not change the original stack but return a "new" one, with the new element present.

The `push` and `pop` operations, of course, shouldn't copy all the data that was on the original stack. Instead, use reference counting: each stack node is a separate object, which refers to the previous stack node. Each node lives as long as there's at least one reference to it.
