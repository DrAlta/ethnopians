pub mod forth_parser;
/*


 /--------------\
IF do_this ELSE other_wize_do_thing THEN
          \------------------------/
[then] else
[else] then
[]

[then] then


ELSE compiles a jmp to the else thread then starts the then thread and THEN starts the else thread

cnd_jmp(then) do_thing jmp(else)

  /---------------\
IF do_this        THEN
cnd_jmp(then)

when you read IF you compiles a condition jump with a dest TBD and put the location of that jump in an IF frame of top of the stack 

in 'IF' mode if you reach 'ELSE' you compile a jump to an unknown dest then pop the IF frame and set it's jump desc to the Working End then put the location of the jump you just compiled into  ELSE frame into the top of stack. 
in 'IF' mode if you reach an 'THEN' you pop the IF frame and set the dest of it's jump to the Working End


if in 'ELSE' mode when you reach 'THEN' you pop the ELSE frame and set the dest of it's jump to the Working End


Alterintively no mode

'ELSE' just compiles a jump with dest TBD and pops in IF frame and sets the dest it's jump to the Working End then with an pushes an ELSE frame with the location of te jump it compiled into the stack

'THEN' pops an ELSE or IF frame and set's the dest of the jump to the Working End

*/