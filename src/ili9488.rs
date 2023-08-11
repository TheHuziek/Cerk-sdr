use core::iter::once;
use display_interface::DataFormat::{U16BEIter, U8Iter};
use display_interface::WriteOnlyDataCommand;
use embedded_graphics_core::{
    pixelcolor::{raw::RawU16, Rgb565},
    prelude::*,
    primitives::Rectangle,
};
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::v2::OutputPin;

enum Command {
    _NOP = 0x00,                    // No-op
    SWRESET = 0x01,                // Software reset
    _RDDID = 0x04,                  // Read display ID info
    _RDDST = 0x09,                  // Read display status
    _SLPIN = 0x10,                  // Enter sleep mode
    SLPOUT = 0x11,                 // Exit sleep mode
    _PTLON = 0x12,                  // Partial mode on
    _NORON = 0x13,                  // Normal display mode on
    _RDMODE = 0x0A,                 // Read display power mode
    _RDMADCTL = 0x0B,               // Read display MADCTL
    _RDPIXFMT = 0x0C,               // Read display pixel format
    _RDIMGFMT = 0x0D,               // Read display image format
    _RDSELFDIAG = 0x0F,             // Read display self-diagnostic
    _INVOFF = 0x20,                 // Display inversion off
    _INVON = 0x21,                  // Display inversion on
    _GAMMASET = 0x26,               // Gamma set
    _DisplayOff = 0x28,            // Display off
    DisplayOn = 0x29,              // Display on
    SetColumn = 0x2A,              // Column address set
    SetPage = 0x2B,                // Page address set
    WriteRam = 0x2C,               // Memory write
    _ReadRam = 0x2E,                // Memory read
    _PTLAR = 0x30,                  // Partial area
    _VSCRDEF = 0x33,                // Vertical scrolling definition
    MADCTL = 0x36,                 // Memory access control
    _VSCRSADD = 0x37,               // Vertical scrolling start address
    PIXFMT = 0x3A,                 // COLMOD: Pixel format set
    _WriteDisplayBrightness = 0x51, // Brightness hardware dependent!
    _ReadDisplayBrightness = 0x52,
    _WriteCtrlDisplay = 0x53,
    _ReadCtrlDisplay = 0x54,
    _WriteCabc = 0x55,        // Write Content Adaptive Brightness Control
    _ReadCabc = 0x56,         // Read Content Adaptive Brightness Control
    _WriteCabcMinimum = 0x5E, // Write CABC Minimum Brightness
    _ReadCabcMinimum = 0x5F,
    IMCTR=0xB0,  // Read CABC Minimum Brightness
    FRMCTR1 = 0xB1,          // Frame rate control (In normal mode/full colors)
    _FRMCTR2 = 0xB2,          // Frame rate control (In idle mode/8 colors)
    _FRMCTR3 = 0xB3,          // Frame rate control (In partial mode/full colors)
    INVCTR = 0xB4,           // Display inversion control
    DFUNCTR = 0xB6,          // Display function control
    PWCTR1 = 0xC0,           // Power control 1
    PWCTR2 = 0xC1,          // Power control 2
    _PWCTRA = 0xCB,           // Power control A
    _PWCTRB = 0xCF,           // Power control B
    VMCTR1 = 0xC5,           // VCOM control 1
    _VMCTR2 = 0xC7,           // VCOM control 2
    _RDID1 = 0xDA,            // Read ID 1
    _RDID2 = 0xDB,            // Read ID 2
    _RDID3 = 0xDC,            // Read ID 3
    _RDID4 = 0xDD,            // Read ID 4
    GMCTRP1 = 0xE0,          // Positive gamma correction
    GMCTRN1 = 0xE1,          // Negative gamma correction
    _DTCA = 0xE8,
    SIMFUNC=0xE9,             // Driver timing control A
    _DTCB = 0xEA,             // Driver timing control B
    _POSC = 0xED,             // Power on sequence control
    _ENABLE3G = 0xF2,         // Enable 3 gamma control
    PUMPRC = 0xF7,           // Pump ratio control
}

#[derive(Debug, Clone, Copy)]
pub struct Ili9488<IFACE, RESET> {
    interface: IFACE,
    reset: RESET,
    width: usize,
    height: usize,
}

impl<IFACE, RESET> Ili9488<IFACE, RESET>
where
    IFACE: WriteOnlyDataCommand,
    RESET: OutputPin,
{
    pub fn new<DELAY>(interface: IFACE, reset: RESET, delay: &mut DELAY) -> Self
    where
        DELAY: DelayMs<u16>,
    {
        let mut ili9488 = Ili9488 {
            interface,
            reset,
            width: 320,
            height: 480,
        };

        let _ = ili9488.reset.set_high();
        let _ = delay.delay_ms(1);
        let _ = ili9488.reset.set_low();
        let _ = delay.delay_ms(5);
        let _ = ili9488.reset.set_high();
        let _ = delay.delay_ms(120);
        ili9488.command(Command::SWRESET, &[]);
        let _ = delay.delay_ms(120);
        ili9488.command(
            Command::GMCTRP1,
            &[
                0x00, 0x03, 0x09, 0x08, 0x16, 0x0A, 0x3F, 0x78, 0x4C, 0x09, 0x0A, 0x08, 0x16, 0x1A,
                0x0F,
            ],
        );
        ili9488.command(Command::GMCTRN1, &[0x00, 0x0E, 0x14, 0x03, 0x11, 0x07, 0x31,
            0xC1, 0x48, 0x08, 0x0F, 0x0C, 0x31, 0x36, 0x0F]);
        ili9488.command(Command::PWCTR1,&[0x17,0x15]);
        ili9488.command(Command::PWCTR2, &[0x41]);
        ili9488.command(Command::VMCTR1,&[0x00,0x12,0x80]);
        ili9488.command(Command::MADCTL, &[0x48]);
        ili9488.command(Command::PIXFMT, &[0x66]);
        ili9488.command(Command::IMCTR, &[0x80]);
        ili9488.command(Command::FRMCTR1, &[0xA0]);
        ili9488.command(Command::INVCTR,&[0x02]);
        ili9488.command(Command::DFUNCTR, &[0x02,0x02]);
        ili9488.command(Command::SIMFUNC,&[0x00]);
        ili9488.command(Command::PUMPRC,&[0xA9,0x51,0x2C,0x82]);
        ili9488.command(Command::SLPOUT, &[]);
        delay.delay_ms(120);
        ili9488.command(Command::DisplayOn,&[]);
        ili9488
    }
    fn command(&mut self, cmd: Command, args: &[u8]) {
        let _ = self.interface.send_commands(U8Iter(&mut once(cmd as u8)));
        let _ = self.interface.send_data(U8Iter(&mut args.iter().cloned()));
    }
    pub fn block(&mut self, x0: u16, y0: u16, x1: u16, y1: u16) {
        self.command(
            Command::SetColumn,
            &[
                (x0 >> 8) as u8,
                (x0 & 0xff) as u8,
                (x1 >> 8) as u8,
                (x1 & 0xff) as u8,
            ],
        );
        self.command(
            Command::SetPage,
            &[
                (y0 >> 8) as u8,
                (y0 & 0xff) as u8,
                (y1 >> 8) as u8,
                (y1 & 0xff) as u8,
            ],
        );
    }
    fn write_iter<I: IntoIterator<Item = u16>>(&mut self, data: I) {
        self.command(Command::WriteRam, &[]);
        let _ = self.interface.send_data(U16BEIter(&mut data.into_iter()));
    }
    pub fn draw_raw_iter<I: IntoIterator<Item = u16>>(
        &mut self,
        x0: u16,
        y0: u16,
        x1: u16,
        y1: u16,
        data: I,
    ) {
        self.block(x0, y0, x1, y1);
        self.write_iter(data)
    }

    pub fn clear_screen(&mut self, color: u16) {
        let color = core::iter::repeat(color).take(self.width * self.height);
        self.draw_raw_iter(0, 0, self.width as u16, self.height as u16, color)
    }
}

impl<IFACE, RESET> OriginDimensions for Ili9488<IFACE, RESET> {
    fn size(&self) -> Size {
        Size::new(480 as u32, 320 as u32)
    }
}

impl<IFACE, RESET> DrawTarget for Ili9488<IFACE, RESET>
where
    IFACE: display_interface::WriteOnlyDataCommand,
    RESET: OutputPin,
{
    type Error = display_interface::DisplayError;

    type Color = Rgb565;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(point, color) in pixels {
            if self.bounding_box().contains(point) {
                let x = point.x as u16;
                let y = point.y as u16;

                self.draw_raw_iter(
                    x,
                    y,
                    x,
                    y,
                    core::iter::once(RawU16::from(color).into_inner()),
                );
            }
        }
        Ok(())
    }

    fn fill_contiguous<I>(&mut self, area: &Rectangle, colors: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Self::Color>,
    {
        let drawable_area = area.intersection(&self.bounding_box());

        if let Some(drawable_bottom_right) = drawable_area.bottom_right() {
            let x0 = drawable_area.top_left.x as u16;
            let y0 = drawable_area.top_left.y as u16;
            let x1 = drawable_bottom_right.x as u16;
            let y1 = drawable_bottom_right.y as u16;

            if area == &drawable_area {
                // All pixels are on screen
                self.draw_raw_iter(
                    x0,
                    y0,
                    x1,
                    y1,
                    area.points()
                        .zip(colors)
                        .map(|(_, color)| RawU16::from(color).into_inner()),
                );
                Ok(())
            } else {
                // Some pixels are on screen
                self.draw_raw_iter(
                    x0,
                    y0,
                    x1,
                    y1,
                    area.points()
                        .zip(colors)
                        .filter(|(point, _)| drawable_area.contains(*point))
                        .map(|(_, color)| RawU16::from(color).into_inner()),
                );
                Ok(())
            }
        } else {
            // No pixels are on screen
            Ok(())
        }
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        self.clear_screen(RawU16::from(color).into_inner());
        Ok(())
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
        self.fill_contiguous(area, core::iter::repeat(color))
    }
}
