# yru-tremolo-rs-stereo

## Overview
Tremolo with depth and rate plus a phase parameter to create a stereo effect.
The _tremolo stereo block diagram_ give an overview of how it works.

![tremolo stereo block diagram](diagram/tremolo-diagram.png)

## Parameters
 - **Depth:** amplitude of the modulation.
 - **Rate:** modulation speed in hertz.
 - **Phase:** in degree. It control phase between left and right channel LFOs.
   This allow to create a kind of auto pan effect.

## Technical notes

At this time, i order to keep code simple, i don't use smoothing methods on
input controls. But this may change... or not.
