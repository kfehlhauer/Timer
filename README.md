# timer

A simple CLI timer that resets and repeats.

```
Usage: timer [OPTIONS]

Options:
  -m <minutes>      Minutes for the countdown [default: 0]
  -s <seconds>      Seconds for the countdown [default: 0]
  -r <reset>        Reset time in seconds
  -n <repeat>       Number of times to repeat. If not specified the timer repeats forever if the reset flag is set
  -h, --help        Print help
  -V, --version     Print version
```

Example: Run timer for 10 seconds and wait for 5 seconds before restarting. Repeat 10 times.
```
timer -s 10 -r 5 -n 5
```

Output
```
5 reps left
0 seconds remaining
Timer reset in 5s
4 reps left
0 seconds remaining
Timer reset in 5s
3 reps left
0 seconds remaining
Timer reset in 5s
2 reps left
0 seconds remaining
Timer reset in 5s
1 reps left
Last rep!
0 seconds remaining
Time's up!
```
