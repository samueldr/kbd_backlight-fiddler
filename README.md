kbd_backlight-fiddler
=====================

A simple tool to manipulate sysfs entries for keyboard backlights.

This should support any backlight respecting the `/sys/class/leds/*::kbd_backlight/`
glob.

Quick Usage
-----------

Checkout the repository somewhere.

Import `module.nix` from this repo in your system configuration.

`kbd_backlight-fiddler` should work.

More details
------------

It should be built, and called with the appropriate rights to fiddle with the
files in `/sys`.

Usage
-----

```
 $ kbd_backlight-fiddler <number>
```

The number may be dependent on the driver, the maximum might be 100, or greated.

Giving a number of `-1` should print the range.

A better CLI with options and diagnostics is planned.
