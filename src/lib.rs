use neon::prelude::*;

use scrap::{Capturer, Display};
use std::io::ErrorKind::WouldBlock;
use std::thread;
use std::time::Duration;

fn printscreen(mut cx: FunctionContext) -> JsResult<JsArray> {
    let one_second = Duration::new(1, 0);
    let one_frame = one_second/600 ;

    let display = Display::primary().expect("Monitor primario nao encontrado");
    let mut capturer = Capturer::new(display).expect("Captura nao iniciada");
    let (w, h) = (capturer.width(), capturer.height());
    let mut png = Vec::new();
    loop {
        let buffer = match capturer.frame() {
            Ok(buffer) => buffer,
            Err(error) => {
                if error.kind() == WouldBlock {
                    thread::sleep(one_frame);
                    continue;
                } else {
                    panic!("Error: {}", error);
                }
            }
        };
        let mut bitflipped = Vec::with_capacity(w * h * 4);
        let stride = buffer.len() / h;

        for y in 0..h {
            for x in 0..w {
                let i = stride * y + 4 * x;
                bitflipped.extend_from_slice(&[
                    buffer[i + 2],
                    buffer[i + 1],
                    buffer[i],
                    255,
                ]);
            }
        }
    
        repng::encode(
            &mut png,
            w as u32,
            h as u32,
            &bitflipped,
        ).unwrap();
        break;
    }
    let js_array = JsArray::new(&mut cx, png.len() as u32);
    
    for (i, obj) in png.iter().enumerate() {
        let js_string = cx.number(*obj as f64);
        js_array.set(&mut cx, i as u32, js_string).unwrap();
    }
    Ok(js_array)
}
fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("printscreen", printscreen)?;
    cx.export_function("hello", hello)?;
    Ok(())
}
