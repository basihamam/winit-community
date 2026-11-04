use std::cell::OnceCell;

use js_sys::{Object, Promise};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Document, Element, Navigator};

pub(crate) fn is_cursor_lock_raw(navigator: &Navigator, document: &Document) -> bool {
    thread_local! {
        static IS_CURSOR_LOCK_RAW: OnceCell<bool> = const { OnceCell::new() };
    }

    IS_CURSOR_LOCK_RAW.with(|cell| {
        *cell.get_or_init(|| {
            // TODO: Remove when Chrome can better advertise that they don't support unaccelerated
            // movement on Linux.
            // See <https://issues.chromium.org/issues/40833850>.
            if super::web_sys::chrome_linux(navigator) {
                return false;
            }

            let element: ElementExt = document.create_element("div").unwrap().unchecked_into();
            let promise = element.request_pointer_lock();

            if promise.is_undefined() {
                false
            } else {
                thread_local! {
                    static REJECT_HANDLER: Closure<dyn FnMut(JsValue)> = Closure::new(|_| ());
                }

                let promise: Promise = promise.unchecked_into();
                let _ = REJECT_HANDLER.with(|handler| promise.catch(handler));
                true
            }
        })
    })
}

pub(crate) fn request_pointer_lock(navigator: &Navigator, document: &Document, element: &Element) {
    let element_ref: &ElementExt = element.unchecked_ref();
    if is_cursor_lock_raw(navigator, document) {
        let element_fallback = element.clone();
        let fallback_handler: Closure<dyn FnMut(JsValue)> = Closure::once(move |_error: JsValue| {
            let el: &ElementExt = element_fallback.unchecked_ref();
            el.request_pointer_lock();
        });

        let options: PointerLockOptions = Object::new().unchecked_into();
        options.set_unadjusted_movement(true);
        let _ = element_ref
            .request_pointer_lock_with_options(&options)
            .catch(&fallback_handler);
    } else {
        element_ref.request_pointer_lock();
    }
}

#[wasm_bindgen]
extern "C" {
    type ElementExt;

    #[wasm_bindgen(method, js_name = requestPointerLock)]
    fn request_pointer_lock(this: &ElementExt) -> JsValue;

    #[wasm_bindgen(method, js_name = requestPointerLock)]
    fn request_pointer_lock_with_options(
        this: &ElementExt,
        options: &PointerLockOptions,
    ) -> Promise;

    type PointerLockOptions;

    #[wasm_bindgen(method, setter, js_name = unadjustedMovement)]
    fn set_unadjusted_movement(this: &PointerLockOptions, value: bool);
}
