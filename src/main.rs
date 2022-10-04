use howdy_rs::CamCapture;
use pixels::{Pixels, SurfaceTexture};
use v4l::io::traits::CaptureStream;
use winit::dpi::LogicalSize;
use winit::event::Event;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

mod lib;



fn main() {
    let mut cam_capture = CamCapture::new();
    let cam_settings = &cam_capture.cam_settings;

    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(cam_settings.width as f64, cam_settings.height as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(cam_settings.width, cam_settings.height, surface_texture).unwrap()
    };

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            let (buf, _meta) = cam_capture.stream.next().unwrap();
            let frame = pixels.get_frame();

            // Greyscale fill
            for (i, pix) in frame.chunks_exact_mut(4).enumerate() {
                let val = [buf[i]; 4];
                pix.copy_from_slice(&val);
            }
            if pixels
                .render()
                // .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        window.request_redraw();
    });
}
