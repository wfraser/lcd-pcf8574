use std::env::args;
use std::process::exit;
use lcd_pcf8574::Pcf8574;

fn usage() -> ! {
    eprintln!("usage: {} <bus> [<i2c-addr>]", args().next().unwrap());
    eprintln!("  where <bus> is the number of /dev/i2c-<bus> to open");
    exit(1);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = args().skip(1);
    let bus = args.next()
        .map(|s| u8::from_str_radix(&s, 10))
        .unwrap_or_else(|| usage())
        .unwrap_or_else(|e| {
            eprintln!("invalid bus number: {}", e);
            usage();
        });
    let addr = args.next()
        .map_or_else(|| Ok(0x27), |s| {
            if let Some(hex) = s.strip_prefix("0x") {
                u16::from_str_radix(hex, 16)
            } else {
                u16::from_str_radix(&s, 10)
            }
        })
        .unwrap_or_else(|e| {
            eprintln!("invalid i2c address: {}", e);
            usage();
        });

    let mut display = lcd::Display::new(Pcf8574::new(bus, addr)?);
    display.init(lcd::FunctionLine::Line2, lcd::FunctionDots::Dots5x8);
    display.display(
        lcd::DisplayMode::DisplayOn,
        lcd::DisplayCursor::CursorOff,
        lcd::DisplayBlink::BlinkOff);

    display.clear();
    display.home();
    display.print("Hello, World!");
    display.position(2, 1);
    display.print("This is line two");
    display.position(2, 2);
    display.print("... and line three");
    display.position(0, 3);
    display.print("Good bye!");

    Ok(())
}
