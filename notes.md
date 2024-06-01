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