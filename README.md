# rgbcal: RGB LED calibration tool
Bart Massey 2024

This tool is designed to find out a decent frame rate and
maximum RGB component values to produce a white-looking RGB
of reasonable brightness.

See below for UI.

**XXX This tool is *mostly* finished! Please wire your
hardware up (see below), finish it, comment it, and use it
to find good values. Then document those values in this
README.**

## Build and Run

Run with `cargo embed --release`. You'll need `cargo embed`, as
`cargo run` / `probe-rs run` does not reliably maintain a
connection for printing. See
https://github.com/probe-rs/probe-rs/issues/1235 for the
details.

## Wiring

Connect the RGB LED to the MB2 as follows:

* Red to P9 (GPIO1)
* Green to P8 (GPIO2)
* Blue to P16 (GPIO3)
* Gnd to Gnd

Connect the potentiometer (knob) to the MB2 as follows:

* Pin 1 to Gnd
* Pin 2 to P2
* Pin 3 to +3.3V

## UI

The knob controls the individual settings: frame rate and
color levels. Which parameter the knob controls should be
determined by which buttons are held. (Right now, the knob
jus always controls Blue. You should see the color change
from green to teal-blue as you turn the knob clockwise.)

* No buttons held: Change the frame rate in steps of 10
  frames per second from 10..160.
* A button held: Change the blue level from off to on over
  16 steps.
* B button held: Change the green level from off to on over
  16 steps.
* A+B buttons held: Change the red level from off to on over
  16 steps.

The "frame rate" (also known as the "refresh rate") is the
time to scan out all three colors. (See the scanout code.)
At 30 frames per second, every 1/30th of a second the LED
should scan out all three colors. If the frame rate is too
low, the LED will appear to "blink". If it is too high, it
will eat CPU for no reason.

I think the frame rate is probably set higher than it needs
to be right now: it can be tuned lower.

## Report
To start, as I was getting together the parts needed for the assignment,
I realized that I was given the wrong pentiometer. After some discussion
in Zulip and a good deal of Google searching, I was able to find the
[part that I had online](https://www.aliexpress.us/item/3256805445652896.html?gatewayAdapt=glo2usa4itemAdapt).
It wasn't clear to me that 'duplex' indicated the knob controlled two sets
of pins, but Prof. Massey explained in the Zulip chat that 'The "duplex" 
indicates a ganged pot, which is what [he] suspected. Likely the three leads 
on the left are connected to the first pot, the three on the right to the 
second pot, and both pots are controlled by that single knob.' I just bent
the 3 on the left up so that I could still plug them in. Didn't work.

To be honest, up until this point, I had been following along wiring hardware
by copying the wiring by example. So I was just following along wiring the
board without really getting what I was doing. This assignment was the
first time that it "clicked" for me that the breadboard is essentially
just room for us to wire some circuits together. Further, the way that
the dragontail splays out the pins over the first 11 rows on the board
finally made sense. Actually understanding what I was doing made wiring
things up much easier; however, the knob wasn't working, so I realized 
that the mapping for the pins on the knob was probably weird. I recalled
seeing a pinout diagram of sorts on the above aliexpress product page, and
used that to find that 3 of the inner pins on the knob actually formed one
of the sets for a pot. Around this time, I had my "aha" moment with the
breadboard / dragontail. Plugged those 3 inner pins in, and finally got a 
knob that was shifting the light from blue to... red.

Figured I was running into a similar issue and checked out the part online
(my RGB components were the correct ones from our kit, which made finding them
online much easier!). A quick peak at the red, green, blue and ground pins
on the light made it clear that I just had to flip the sucker around and:
Tada! Green to light-blue / tealish-white light! Of course, I re-read 
through the assignment handout and of course the instructions there cover
this, but it didn't make much sense at the time. I had to be forced to think
about it a bit more before I understood what I was doing. I am actually
grateful that I got the wrong part in my kit, because If I hadn't I probably
would have just continued to copy wire positioning on the breadboard without
actually understanding it.

So with everything wired up and working...


