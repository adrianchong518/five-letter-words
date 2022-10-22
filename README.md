# Five-Letter Words

This repo is a solution in Rust to the problem of "25 unique letters in 5 five-letter words" raised by Matt Parker (ie @standupmaths).
The problem is explained in his [video](https://youtu.be/_-AfhLQfb6w) in detail.

Matt Parker's solution to this problem was "greatly inefficient" to say the least, which motivated me to attempt this problem.

## Original Ideas

Notably, I *did not* look at his [follow-up video](https://youtu.be/c33AZBnRHks) for ideas before the commit `f332fca8`.

To optimize the code, I basically done the following:

- Use a bit / bit mask representation for all the words and word sets.
  - This reduces repetition check overhead by using bitwise and.
  - This also removes all anagrams in the process.
- Build a map of all possible combinations of words that produced a set of unique letters.

To my surprise, some of these techniques were also used in the then optimal solution shown in his follow-up video.

My code resulted in a total runtime of about 300 seconds on a M1 Macbook Air, including writing the results and "bit representations" file.
