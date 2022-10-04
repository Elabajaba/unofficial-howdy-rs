use std::time::Instant;

use v4l::io::mmap::Stream;
use v4l::Format;
use v4l::{buffer::Type, video::Capture, Device, FourCC};

#[allow(unused)]
pub struct CameraSettings {
    pub width: u32,
    pub height: u32,
    pub _pixel_fmt: usize,
}

impl CameraSettings {
    #[allow(unused)]
    fn new(width: u32, height: u32, pixel_fmt: usize) -> Self {
        CameraSettings {
            width,
            height,
            _pixel_fmt: pixel_fmt,
        }
    }
}

#[allow(unused)]
pub struct CamCapture<'a> {
    pub cam_settings: CameraSettings,
    pub device: Device,
    pub format: Format,
    pub stream: Stream<'a>,
}

impl CamCapture<'_> {
    #[allow(unused)]
    pub fn new() -> Self {
        let now = Instant::now();

        let mut device = Device::new(2).expect("Failed to open device");

        let cam_settings = CameraSettings::new(640, 360, 0);

        let mut format = device.format().expect("Failed to read format");
        format.width = cam_settings.width;
        format.height = cam_settings.height;
        format.fourcc = FourCC::new(b"GREY");
        device.set_format(&format).expect("Failed to write format");

        let stream = Stream::with_buffers(&mut device, Type::VideoCapture, 4)
            .expect("Failed to create buffer stream");

        let end = Instant::now() - now;
        println!("time to get camera: {:?}", end);

        CamCapture {
            cam_settings,
            device,
            format,
            stream,
        }
    }
}
