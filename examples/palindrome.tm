# palindrome.tm
alphabet a b
tapes input work
states q0 q1 q2 yes no
[q0 > _] [q0 @ @ -> -]
[q0 a _] [q0 @ a -> ->]
[q0 b _] [q0 @ b -> ->]
[q0 _ _] [q1 @ @ <- <-]
[q1 a _] [q1 @ @ <- -]
[q1 b _] [q1 @ @ <- -]
[q1 > _] [q2 @ @ -> -]
[q2 a b] [no @ @ - -]
[q2 b a] [no @ @ - -]
[q2 a a] [q2 @ @ -> <-]
[q2 b b] [q2 @ @ -> <-]
[q2 > _] [yes @ @ - -]
