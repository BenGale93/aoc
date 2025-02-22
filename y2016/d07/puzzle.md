\--- Day 7: Internet Protocol Version 7 ---
----------

While snooping around the local network of EBHQ, you compile a list of [IP addresses](https://en.wikipedia.org/wiki/IP_address) (they're IPv7, of course; [IPv6](https://en.wikipedia.org/wiki/IPv6) is much too limited). You'd like to figure out which IPs support *TLS* (transport-layer snooping).

An IP supports TLS if it has an Autonomous Bridge Bypass Annotation, or *ABBA*. An ABBA is any four-character sequence which consists of a pair of two different characters followed by the reverse of that pair, such as `xyyx` or `abba`. However, the IP also must not have an ABBA within any hypernet sequences, which are contained by *square brackets*.

For example:

* `abba[mnop]qrst` supports TLS (`abba` outside square brackets).
* `abcd[bddb]xyyx` does *not* support TLS (`bddb` is within square brackets, even though `xyyx` is outside square brackets).
* `aaaa[qwer]tyui` does *not* support TLS (`aaaa` is invalid; the interior characters must be different).
* `ioxxoj[asdfgh]zxcvbn` supports TLS (`oxxo` is outside square brackets, even though it's within a larger string).

*How many IPs* in your puzzle input support TLS?

Your puzzle answer was `115`.

\--- Part Two ---
----------

You would also like to know which IPs support *SSL* (super-secret listening).

An IP supports SSL if it has an Area-Broadcast Accessor, or *ABA*, anywhere in the supernet sequences (outside any square bracketed sections), and a corresponding Byte Allocation Block, or *BAB*, anywhere in the hypernet sequences. An ABA is any three-character sequence which consists of the same character twice with a different character between them, such as `xyx` or `aba`. A corresponding BAB is the same characters but in reversed positions: `yxy` and `bab`, respectively.

For example:

* `aba[bab]xyz` supports SSL (`aba` outside square brackets with corresponding `bab` within square brackets).
* `xyx[xyx]xyx` does *not* support SSL (`xyx`, but no corresponding `yxy`).
* `aaa[kek]eke` supports SSL (`eke` in supernet with corresponding `kek` in hypernet; the `aaa` sequence is not related, because the interior character must be different).
* `zazbz[bzb]cdb` supports SSL (`zaz` has no corresponding `aza`, but `zbz` has a corresponding `bzb`, even though `zaz` and `zbz` overlap).

*How many IPs* in your puzzle input support SSL?

Your puzzle answer was `231`.

Both parts of this puzzle are complete! They provide two gold stars: \*\*

At this point, you should [return to your Advent calendar](/2016) and try another puzzle.

If you still want to see it, you can [get your puzzle input](7/input).

You can also [Shareon [Bluesky](https://bsky.app/intent/compose?text=I%27ve+completed+%22Internet+Protocol+Version+7%22+%2D+Day+7+%2D+Advent+of+Code+2016+%23AdventOfCode+https%3A%2F%2Fadventofcode%2Ecom%2F2016%2Fday%2F7) [Twitter](https://twitter.com/intent/tweet?text=I%27ve+completed+%22Internet+Protocol+Version+7%22+%2D+Day+7+%2D+Advent+of+Code+2016&url=https%3A%2F%2Fadventofcode%2Ecom%2F2016%2Fday%2F7&related=ericwastl&hashtags=AdventOfCode) [Mastodon](javascript:void(0);)] this puzzle.