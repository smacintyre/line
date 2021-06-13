# line -- a line editor

This is a learning project following @jonathandturner's YouTube video series
[Creating a line editor in Rust](https://www.youtube.com/playlist?list=PLP2yfE2-FXdQw0I6O4YdIX_mzBeF5TDdv).
It is currently up to date with the end of the first video. The series is very
good from what I've seen so far.

### Difference from the video

I try to implement things myself first and then see how @jonathandturner
does them in the video. This has resulted in a few differences

- I use the `queue!` macro instead of function approach
- I don't track the the the caret position
  - Rather, I get the current position at the start of the input loop.
    And use that to find the index to change in the buffer
  - I use `SavePosition` and `RestorePosition` when changing the line and
    always print the whole buffer
