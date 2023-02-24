# midicheap

super simple midi monitor.

run with no arguments to get a list of ports. for example,
```
0 Midi Through:Midi Through Port-0 14:0
1 Launchkey Mini MK3:Launchkey Mini MK3 Launchkey Mi 28:0
2 Launchkey Mini MK3:Launchkey Mini MK3 Launchkey Mi 28:1
```
the first number is the port index.

run with one argument (a port index) to monitor messages on that port. system common and realtime messages are filtered out so you don't get spammed with clocks.

the columns are `timestamp command channel data0 data1`, except for pitchbend, where they're ``timestamp command channel bend`.

midicheap is missing most of the functionality that you'd want, but it's useful for quick testing or debugging of midi-producing devices or programs.

### todo:

- channel mode messages
- system common/real time
- dynamic filtering
