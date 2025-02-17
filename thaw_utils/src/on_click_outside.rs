use crate::BoxCallback;
use leptos::{html::Div, prelude::*};
use tachys::reactive_graph::node_ref::NodeRef;

pub fn call_on_click_outside(element: NodeRef<Div>, on_click: BoxCallback) {
    #[cfg(any(feature = "csr", feature = "hydrate"))]
    {
        use leptos::ev;
        let handle = window_event_listener(ev::click, move |ev| {
            use leptos::wasm_bindgen::__rt::IntoJsResult;
            let el = ev.target();
            let mut el: Option<web_sys::Element> =
                el.into_js_result().map_or(None, |el| Some(el.into()));
            let body = document().body().unwrap();
            while let Some(current_el) = el {
                if current_el == *body {
                    break;
                };
                let Some(displayed_el) = element.get_untracked() else {
                    break;
                };
                if current_el == **displayed_el {
                    return;
                }
                el = current_el.parent_element();
            }
            on_click();
        });
        on_cleanup(move || handle.remove());
    }
    #[cfg(not(any(feature = "csr", feature = "hydrate")))]
    {
        let _ = element;
        let _ = on_click;
    }
}
