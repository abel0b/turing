# palindrome
alphabet a b
tapes input work
states copy_input(initial) return compare yes(final) no(final)
[copy_input > >] [copy_input @ @ -> ->]
[copy_input a _] [copy_input @ a -> ->]
[copy_input b _] [copy_input @ b -> ->]
[copy_input _ _] [return @ @ <- -]
[return a _] [return @ @ <- -]
[return b _] [return @ @ <- -]
[return > _] [compare @ @ -> <-]
[compare a b] [no @ @ - -]
[compare b a] [no @ @ - -]
[compare a a] [compare @ @ -> <-]
[compare b b] [compare @ @ -> <-]
[compare _ >] [yes @ @ - -]
