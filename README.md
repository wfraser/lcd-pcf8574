This is an adapter class for the [`lcd`][lcd] crate which makes it work with the popular PCF8574
I2C expander boards that are used with small LCM1602 or HD44780 LCD screens.

They look like this:
![example photo](http://yourduino.com/docs/LCD-20x4-New3-800.jpg)

This requires a slight modification to the `lcd` crate [which I have made][lcd-wfraser].

[lcd]: https://github.com/idubrov/lcd
[lcd-wfraser]: https://github.com/wfraser/lcd
