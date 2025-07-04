\--- Day 20: Firewall Rules ---
----------

You'd like to set up a small hidden computer here so you can use it to get back into the network later. However, the corporate firewall only allows communication with certain external [IP addresses](https://en.wikipedia.org/wiki/IPv4#Addressing).

You've retrieved the list of blocked IPs from the firewall, but the list seems to be messy and poorly maintained, and it's not clear which IPs are allowed. Also, rather than being written in [dot-decimal](https://en.wikipedia.org/wiki/Dot-decimal_notation) notation, they are written as plain [32-bit integers](https://en.wikipedia.org/wiki/32-bit), which can have any value from `0` through `4294967295`, inclusive.

For example, suppose only the values `0` through `9` were valid, and that you retrieved the following blacklist:

```
5-8
0-2
4-7

```

The blacklist specifies ranges of IPs (inclusive of both the start and end value) that are *not* allowed. Then, the only IPs that this firewall allows are `3` and `9`, since those are the only numbers not in any range.

Given the list of blocked IPs you retrieved from the firewall (your puzzle input), *what is the lowest-valued IP* that is not blocked?

Your puzzle answer was `31053880`.

The first half of this puzzle is complete! It provides one gold star: \*

\--- Part Two ---
----------

*How many IPs* are allowed by the blacklist?

Answer:

Although it hasn't changed, you can still [get your puzzle input](20/input).

You can also [Shareon [Bluesky](https://bsky.app/intent/compose?text=I%27ve+completed+Part+One+of+%22Firewall+Rules%22+%2D+Day+20+%2D+Advent+of+Code+2016+%23AdventOfCode+https%3A%2F%2Fadventofcode%2Ecom%2F2016%2Fday%2F20) [Twitter](https://twitter.com/intent/tweet?text=I%27ve+completed+Part+One+of+%22Firewall+Rules%22+%2D+Day+20+%2D+Advent+of+Code+2016&url=https%3A%2F%2Fadventofcode%2Ecom%2F2016%2Fday%2F20&related=ericwastl&hashtags=AdventOfCode) [Mastodon](javascript:void(0);)] this puzzle.