notepad\_logger
==============

A logger which puts its output to an open Notepad window. It specifically looks
for the window by its title, which must be either `Untitled - Notepad` or
`*Untitled - Notepad`. While there are basic assertions to make sure we aren't
operating on null pointers, there are no other safety checks, nor do we check
for the result.

This was written by someone who does not know the windows API or its invariants,
and probably shouldn't actually be used for anything. This is just a fun toy
based on
https://www.reddit.com/r/programming/comments/gnazif/ray\_tracing\_in\_notepadexe\_at\_30\_fps/fr8uy2l/.
This probably isn't thread safe.
