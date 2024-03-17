# rgbcal: RGB LED calibration tool

Skeleton by: Bart Massey 2024

## Final Assignment

Homework 3
CS 510 - Rust Embedded Programming
PDX Winter 2024
Bradley Thompson

Note: Report at the bottom of the readme

## Assignment

This tool is designed to find out a decent frame rate and
maximum RGB component values to produce a white-looking RGB
of reasonable brightness.

See below for UI.

**XXX This tool is _mostly_ finished! Please wire your
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

-   Red to P9 (GPIO1)
-   Green to P8 (GPIO2)
-   Blue to P16 (GPIO3)
-   Gnd to Gnd

Connect the potentiometer (knob) to the MB2 as follows:

-   Pin 1 to Gnd
-   Pin 2 to P2
-   Pin 3 to +3.3V

## UI

The knob controls the individual settings: frame rate and
color levels. Which parameter the knob controls should be
determined by which buttons are held. (Right now, the knob
jus always controls Blue. You should see the color change
from green to teal-blue as you turn the knob clockwise.)

-   No buttons held: Change the frame rate in steps of 10
    frames per second from 10..160.
-   A button held: Change the blue level from off to on over
    16 steps.
-   B button held: Change the green level from off to on over
    16 steps.
-   A+B buttons held: Change the red level from off to on over
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

### Calibration Results

```
red: 13  
green: 12
blue: 10       
frame rate: 60 
```

### Write-Up
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
through the assignment handout and, of course, the instructions there cover
this, but it didn't make much sense at the time. I had to be forced to think
about it a bit more before I understood what I was doing. I am actually
grateful that I got the wrong part in my kit, because If I hadn't I probably
would have just continued to copy wire positioning on the breadboard without
actually understanding it.

So with everything wired up and working I got started on the code...

First, I spent time running through all of the code and commenting everything
to help build an understanding of what was going on. Really the skeleton was
well built-out so it made it easy to grasp and iterate on.

Before worrying about updating state correctly, I got the frame rate in sync
with a const value. Didn't really need to, but figured it would help
to have those tied together before handling mutability. So, that's why I
made another constructor `UiState::with_frame_rate`; however, I ended up
removing this later when I realized that I hadn't configured RGB to take
in frame rate updates correctly.

Once I had roughly 50% of the code commented, I focused in on the state updates
in `ui.rs`. It was immediately clear that not a lot needed to change, as all
the components were already pulled into the UI so that I had everything to
correct the calibration logic. Just had to handle the switch for which
piece of state to update based on the various input sources (btnA, btnB, knob).
I originally made `Ui::update` a Unit-returning function, but realized that I
needed some way to avoid overwhelming log output and rgb updates every time the
UI's main loop hit the lines to show those values.

At the same time, I adjusted the framerate as-per the specification, by adding
10 to the scaled value we get from `Knob::measure`. When I got everything in
order and tested the code, I noticed that my logs were spamming. Through in
some `rprintln`s to realize that the issue was my frame rate update case:
The previously stored state for frame rate was always different because the
state was shifted + 10, but the active measurement wasn't shifted accordingly.
So I fixed that and _tada_ everything magically worked.

Around this time I also repositioned my knob: Originally, since I have the weird
ganged pot, it was all cattywampus and sidewise. Went ahead and moved it further
down the board, slightly, to make room, and flipped some of the pin configuration
so that I could have it face upright.

I proceeded to finish the write-up, start my calibration, and then realized that
I forgot to make RGB update with the frame rate calibration. Fixed that by 
emulating the RGB_LEVEL handling.

With all of the development complete, I went ahead and took my calibration
measurements (Shown at the top of the report). It was kinda hard to determine
how "white" white was supposed to be. Made more confusing because you can see the
green and blue coming from the individual pins / diodes... But I am pretty sure
that it all adds up to a relatively white-looking light! Frame rate didn't
seem to have too much effect, but the lower value did seem to cut out some
"blueness" from the hue, so long as I wasn't so low that I was getting blinking.


### Thank you!
Last, just wanted to say thank you very much for the fun assignment, great course
and all other awesome classes I've had with you. This was the last term of my
grad degree, and from Open Source Software, to the original Rust Programming course,
back to undergrad when I took your AI class, you've always made class extremely
engaging, and you've always been super thoughtful of your students time. The
passion shows in the teaching and makes it super easy to both follow these courses,
but also to want to follow them and continue to learn beyond the confines of the
term. I'll definitely keep playing around in embedded with my new-found free time!

So yeah, thanks overall for being a great teacher. Good luck moving forward with all
of the fun embedded projects. Cheers!