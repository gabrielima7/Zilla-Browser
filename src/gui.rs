use crate::adblock::ADBLOCK_ENGINE;
use std::{borrow::Cow, sync::Arc};
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::{
    http::{Request, Response},
    WebViewBuilder, WebViewBuilderExtUnix, WebViewId,
};

// Tao's GTK extensions are needed on Linux.
#[cfg(target_os = "linux")]
use tao::platform::unix::WindowExtUnix;

pub fn create_and_run_gui() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Zilla Browser")
        .with_inner_size(tao::dpi::LogicalSize::new(1280, 720))
        .build(&event_loop)
        .unwrap();

    let handler = Arc::new(
        |_webview_id: WebViewId, request: Request<Vec<u8>>| {
            let url = request.uri().to_string();

            // This is a simplification. A real browser would have more context.
            let source_url = "https://adblock-tester.com/";
            let request_type = if url.ends_with(".css") {
                "css"
            } else if url.ends_with(".js") {
                "script"
            } else {
                "image"
            };

            let adblock_engine = ADBLOCK_ENGINE.lock().unwrap();

            if adblock_engine.check_should_block(&url, source_url, request_type) {
                println!("[ADBLOCK] Blocked: {}", url);
                // Return an empty response to block the request
                return Response::builder()
                    .status(200) // 200 OK with empty body is a common way to block
                    .body(Cow::from(vec![]))
                    .unwrap();
            }

            println!("[ADBLOCK] Allowed: {}", url);
            // Allowed, so fetch it with our client
            match adblock_engine.client.get(&url).send() {
                Ok(res) => {
                    let status = res.status().as_u16();
                    let headers = res.headers().clone();
                    match res.bytes() {
                        Ok(bytes) => {
                            let mut response_builder = Response::builder().status(status);
                            for (name, value) in headers.iter() {
                                response_builder = response_builder.header(name, value);
                            }
                            response_builder.body(Cow::from(bytes.to_vec())).unwrap()
                        }
                        Err(e) => {
                            eprintln!("[ERROR] Failed to read bytes from {}: {}", url, e);
                            Response::builder().status(500).body(Cow::from(vec![])).unwrap()
                        }
                    }
                }
                Err(e) => {
                    eprintln!("[ERROR] Failed to fetch {}: {}", url, e);
                    Response::builder().status(500).body(Cow::from(vec![])).unwrap()
                }
            }
        },
    );

    let _webview = {
        let https_handler = Arc::clone(&handler);
        let http_handler = Arc::clone(&handler);
        let builder = WebViewBuilder::new()
            .with_url("https://adblock-tester.com/")
            .with_custom_protocol("https".into(), move |id, req| https_handler(id, req))
            .with_custom_protocol("http".into(), move |id, req| http_handler(id, req));

        #[cfg(target_os = "linux")]
        {
            builder.build_gtk(window.gtk_window()).unwrap()
        }

        #[cfg(not(target_os = "linux"))]
        {
            builder.build(&window).unwrap()
        }
    };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
