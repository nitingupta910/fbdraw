fbdraw provides a single interface `put_pixel(x, y, color)` for drawing on the screen. This simple interface makes
it easy to play around with graphics algorithms like curve drawing, without the burden of setting up a window
(with an event loop), setting up GPU pipeline, or managing software buffers, etc.

The coordinate system has origin at the top-left, with X and Y axis running left-to-right and top-to-bottom, respectively.

It is a wrapper for the [minifb](https://docs.rs/minifb/latest/minifb/) library, and provides a simpler `Surface` based interface.
