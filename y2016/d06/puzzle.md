\--- Day 6: Signals and Noise ---
----------

Something is jamming your communications with Santa. Fortunately, your signal is only partially jammed, and protocol in situations like this is to switch to a simple [repetition code](https://en.wikipedia.org/wiki/Repetition_code) to get the message through.

In this model, the same message is sent repeatedly. You've recorded the repeating message signal (your puzzle input), but the data seems quite corrupted - almost too badly to recover. *Almost*.

All you need to do is figure out which character is most frequent for each position. For example, suppose you had recorded the following messages:

```
eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar

```

The most common character in the first column is `e`; in the second, `a`; in the third, `s`, and so on. Combining these characters returns the error-corrected message, `easter`.

Given the recording in your puzzle input, *what is the error-corrected version* of the message being sent?

Your puzzle answer was `ygjzvzib`.

The first half of this puzzle is complete! It provides one gold star: \*

\--- Part Two ---
----------

Of course, that *would* be the message - if you hadn't agreed to use a *modified repetition code* instead.

In this modified code, the sender instead transmits what looks like random data, but for each character, the character they actually want to send is *slightly less likely* than the others. Even after signal-jamming noise, you can look at the letter distributions in each column and choose the *least common* letter to reconstruct the original message.

In the above example, the least common character in the first column is `a`; in the second, `d`, and so on. Repeating this process for the remaining characters produces the original message, `advent`.

Given the recording in your puzzle input and this new decoding methodology, *what is the original message* that Santa is trying to send?

Answer:

Although it hasn't changed, you can still [get your puzzle input](6/input).

You can also [Shareon [Bluesky](https://bsky.app/intent/compose?text=I%27ve+completed+Part+One+of+%22Signals+and+Noise%22+%2D+Day+6+%2D+Advent+of+Code+2016+%23AdventOfCode+https%3A%2F%2Fadventofcode%2Ecom%2F2016%2Fday%2F6) [Twitter](https://twitter.com/intent/tweet?text=I%27ve+completed+Part+One+of+%22Signals+and+Noise%22+%2D+Day+6+%2D+Advent+of+Code+2016&url=https%3A%2F%2Fadventofcode%2Ecom%2F2016%2Fday%2F6&related=ericwastl&hashtags=AdventOfCode) [Mastodon](javascript:void(0);)] this puzzle.