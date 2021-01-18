use wasm_bindgen_test::*;


#[cfg(test)]
pub mod test {
    use web_sys::{HtmlMediaElement, MediaStream, MediaStreamConstraints, window};
    use wasm_bindgen::JsValue;

    #[wasm_bindgen_test]
    fn test_get_display() {
        let mut constraints = MediaStreamConstraints::new();
        constraints.video(&JsValue::from(true));
        let media_stream_promise: Result<MediaStream, JsValue> = window()
            .navigator()
            .media_devices()
            .unwrap()
            .get_display_media(constraints);

        assert_eq!(media_stream_promise.is_ok(), true);
    }
}