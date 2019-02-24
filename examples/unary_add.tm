# unary_add
alphabet 1
tapes input_a input_b output_a+b
states add_a(initial) add_b done(final)
[add_a > > >] [add_a @ @ @ -> - ->]
[add_a 1 > _] [add_a @ @ 1 -> - ->]
[add_a _ > _] [add_b @ @ @ - -> -]
[add_b _ 1 _] [add_b @ @ 1 - -> ->]
[add_b _ _ _] [done @ @ @ - - -]
