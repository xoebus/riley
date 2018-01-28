# riley

## about

This program will solve the [Countdown number round][game] problems. This is
written poorly but will still brute-force all possible solutions in well under
a second on modern hardware. The solution provided might not be the only valid
solution.

[game]: http://datagenetics.com/blog/august32014/index.html

It can be installed by cloning this repository and running `cargo install`.

## usage

```
$ riley [NUMBERS...] SOLUTION
```

# example

```
$ riley 25 50 75 100 3 6 952
(((((6 + 100) * 3) * 75) - 50) / 25) = 952
```

