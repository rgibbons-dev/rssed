# rssed notes

* got reasonably far into it without much strife
* stuck on an issue where a mutable variable scoped outside of a loop isn't updating when i re-assign it
    * i may need to use &mut but rust-analyzer is saying nothing
* i figured it out
    * enums i need to return a new value
    * for whatever reason, that wasn't happening in the if
    * so i switched it to a match
        * not sure if that's what actually did it but it made my code more concise
    * the main reason was i forgot to trim the command from the prefixed line address
        * once i did that i was able to print ranges
* remaining work:
    * [x] implement delete command
    * [x] implement open feed command
    * ~~[] implement exit feed command~~ **CANCELED**
    * [x] reduce scope of current_address
    * [x] impl numbered ranges and larger numbers
    * [x] impl fn to process items and pretty print
* i think i need to maintain a separate current line for the item-level
    * there's a fork in the road:
        1. having o be line-addressed means that it uses the line addressing for feed-level elements
        2. having o not be line addressed means i need to establish indices to use elsewhere
    * i think i'm going to go with route 2
        * so i'll have a new command `x` which accepts line addresses for items
            * these have to be stored outside of the loop
        * then i will need another command `b` which sets the item line number
        * i can't just copy everything directly
            * i only fill the store when o is called
        * a compromise:
            * my types aren't narrow enough
            * so i'm just going to do the single range
            * you can only read one entry at a time!
            * i also don't need x
* there's one more hurdle
    * the number parsing is broken
    * it is cutting off the zero