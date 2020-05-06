use std::cell::Cell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

type Canvas = web_sys::HtmlCanvasElement;
type Context = web_sys::CanvasRenderingContext2d;
type MouseClosure = Closure<dyn FnMut(web_sys::MouseEvent)>;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
	let canvas = create_canvas()?;
	let context = Rc::new(create_context(&canvas)?);
	let pressed = Rc::new(Cell::new(false));

	let mut mouse_events = HashMap::new();
	mouse_events.insert("mousedown", mouse_down(context.clone(), pressed.clone()));
	mouse_events.insert("mousemove", mouse_move(context.clone(), pressed.clone()));
	mouse_events.insert("mouseup", mouse_up(context.clone(), pressed.clone()));

	for (name, closure) in mouse_events.into_iter() {
		canvas.add_event_listener_with_callback(name, closure.as_ref().unchecked_ref())?;
		closure.forget();
	}

	Ok(())
}

fn create_canvas() -> Result<Canvas, JsValue> {
	let document = web_sys::window().unwrap().document().unwrap();
	let canvas = document.create_element("canvas")?.dyn_into::<Canvas>()?;

	document.body().unwrap().append_child(&canvas)?;
	canvas.set_width(640);
	canvas.set_height(480);
	canvas.style().set_property("border", "solid")?;

	Ok(canvas)
}

fn create_context(canvas: &Canvas) -> Result<Context, JsValue> {
	let context = canvas.get_context("2d")?.unwrap().dyn_into::<Context>()?;
	Ok(context)
}

fn mouse_down(context: Rc<Context>, pressed: Rc<Cell<bool>>) -> MouseClosure {
	Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
		context.begin_path();
		context.move_to(event.offset_x() as f64, event.offset_y() as f64);
		pressed.set(true);
	}) as Box<dyn FnMut(_)>)
}

fn mouse_move(context: Rc<Context>, pressed: Rc<Cell<bool>>) -> MouseClosure {
	Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
		if pressed.get() {
			context.line_to(event.offset_x() as f64, event.offset_y() as f64);
			context.stroke();
			context.begin_path();
			context.move_to(event.offset_x() as f64, event.offset_y() as f64);
		}
	}) as Box<dyn FnMut(_)>)
}

fn mouse_up(context: Rc<Context>, pressed: Rc<Cell<bool>>) -> MouseClosure {
	Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
		pressed.set(false);
		context.line_to(event.offset_x() as f64, event.offset_y() as f64);
		context.stroke();
	}) as Box<dyn FnMut(_)>)
}
