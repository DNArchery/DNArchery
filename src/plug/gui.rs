use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::{Icon, WindowBuilder},
    },
    webview::WebViewBuilder,
};

pub fn spin_gui_window() {
    let event_loop = EventLoop::new();

    let bytes: Vec<u8> = include_bytes!("../../assets/dnarchery-logo.png").to_vec();
    let imagebuffer = image::load_from_memory_with_format(&bytes, image::ImageFormat::Png)
        .unwrap()
        .into_rgba8();
    let (icon_width, icon_height) = imagebuffer.dimensions();
    let icon_rgba = imagebuffer.into_raw();

    let window = WindowBuilder::new()
        .with_title("DNArchery")
        .with_window_icon(Some(
            Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap(),
        ))
        .build(&event_loop)
        .unwrap();

    let _webview = WebViewBuilder::new(window)
        .unwrap()
        .with_url("http://127.0.0.1:1337/ui")
        .unwrap()
        .build()
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => info!("Starting integrated GUI"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
