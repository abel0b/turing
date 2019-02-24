# unary_add
alphabet 1
tapes input_a input_b output_a-b
states sub(initial) error:negative_value(final) done(final)
[sub > > >] [sub @ @ @ -> -> ->]
[sub 1 1 _] [sub @ @ @ -> -> -]
[sub _ 1 _] [error:negative_value @ @ @ -> -> -]
[sub 1 _ _] [sub @ @ 1 -> - ->]
[sub _ _ _] [done @ @ @ - - -]
