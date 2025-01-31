\--- Day 20: Infinite Elves and Infinite Houses ---
----------

To keep the Elves busy, Santa has them deliver some presents by hand, door-to-door. He sends them down a street with infinite houses numbered sequentially: `1`, `2`, `3`, `4`, `5`, and so on.

Each Elf is assigned a number, too, and delivers presents to houses based on that number:

* The first Elf (number `1`) delivers presents to every house: `1`, `2`, `3`, `4`, `5`, ....
* The second Elf (number `2`) delivers presents to every second house: `2`, `4`, `6`, `8`, `10`, ....
* Elf number `3` delivers presents to every third house: `3`, `6`, `9`, `12`, `15`, ....

There are infinitely many Elves, numbered starting with `1`. Each Elf delivers presents equal to *ten times* his or her number at each house.

So, the first nine houses on the street end up like this:

```
House 1 got 10 presents.
House 2 got 30 presents.
House 3 got 40 presents.
House 4 got 70 presents.
House 5 got 60 presents.
House 6 got 120 presents.
House 7 got 80 presents.
House 8 got 150 presents.
House 9 got 130 presents.

```

The first house gets `10` presents: it is visited only by Elf `1`, which delivers `1 * 10 = 10` presents. The fourth house gets `70` presents, because it is visited by Elves `1`, `2`, and `4`, for a total of `10 + 20 + 40 = 70` presents.

What is the *lowest house number* of the house to get at least as many presents as the number in your puzzle input?

Your puzzle answer was `831600`.

The first half of this puzzle is complete! It provides one gold star: \*

\--- Part Two ---
----------

The Elves decide they don't want to visit an infinite number of houses. Instead, each Elf will stop after delivering presents to `50` houses. To make up for it, they decide to deliver presents equal to *eleven times* their number at each house.

With these changes, what is the new *lowest house number* of the house to get at least as many presents as the number in your puzzle input?

Answer:

Your puzzle input is still `36000000`.

You can also [Shareon [Bluesky](https://bsky.app/intent/compose?text=I%27ve+completed+Part+One+of+%22Infinite+Elves+and+Infinite+Houses%22+%2D+Day+20+%2D+Advent+of+Code+2015+%23AdventOfCode+https%3A%2F%2Fadventofcode%2Ecom%2F2015%2Fday%2F20) [Twitter](https://twitter.com/intent/tweet?text=I%27ve+completed+Part+One+of+%22Infinite+Elves+and+Infinite+Houses%22+%2D+Day+20+%2D+Advent+of+Code+2015&url=https%3A%2F%2Fadventofcode%2Ecom%2F2015%2Fday%2F20&related=ericwastl&hashtags=AdventOfCode) [Mastodon](javascript:void(0);)] this puzzle.