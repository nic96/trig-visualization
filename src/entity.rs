#[cfg(target_arch = "wasm32")]
use crossbeam_channel::{unbounded, Receiver};
#[cfg(target_arch = "wasm32")]
use winit::dpi::LogicalSize;

pub struct CosLine;
pub struct SinLine;
pub struct TanLine;
pub struct CotLine;
pub struct RadiusLine;
pub struct Circle;

pub struct CosText;
pub struct SinText;
pub struct TanText;
pub struct CotText;
pub struct ThetaText;

pub struct Theta(pub f32);
pub struct Radius(pub f32);
pub struct Paused(pub bool);
pub struct PauseButtonHovered(pub bool);

#[cfg(target_arch = "wasm32")]
pub struct WinitWebResizing {
    pub rx: Receiver<LogicalSize<f32>>,
}

#[cfg(target_arch = "wasm32")]
impl WinitWebResizing {
    pub fn new() -> Self {
        use bevy::log;
        use wasm_bindgen::JsCast;
        let (tx, rx) = unbounded();

        let get_full_size = || {
            let win = web_sys::window().unwrap();
            // `inner_width` corresponds to the browser's `self.innerWidth` function, which are in
            // Logical, not Physical, pixels
            winit::dpi::LogicalSize::new(
                win.inner_width().unwrap().as_f64().unwrap() as f32,
                win.inner_height().unwrap().as_f64().unwrap() as f32,
            )
        };

        tx.send(get_full_size()).unwrap();

        let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |e: web_sys::Event| {
            log::debug!("handling resize event: {:?}", e);
            tx.send(get_full_size()).unwrap();
        }) as Box<dyn FnMut(_)>);
        let window = web_sys::window().unwrap();
        window
            .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();

        return Self { rx };
    }
}
