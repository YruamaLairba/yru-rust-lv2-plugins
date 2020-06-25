# yru-echo-rs

Basic echo with delay feedback and mix parameters. The following diagram
give an overview of how it works :

![echo block diagram](diagram/echo-diagram.png)

At this time, i order to keep code simple:
- no smoothing when tweaking parameters
- no taped delay efect when change delay value
- no subsample delay precision, the delay is truncated to the corresponding
  number of samples.

But this may change... or not.
