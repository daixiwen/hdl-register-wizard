//! file upload/download helper functions

use seed::prelude::*;

use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

/// make the user download some data as a text file
pub fn download_text(filename: &str, data: &str) {

    let encoded_data: String = js_sys::encode_uri_component(&data).into();
    let mime = String::from("data:text/plain;charset=utf-8");

    let uri = &format!("{},{}", mime, encoded_data);

    let element = seed::document()
        .create_element("a")
        .expect("should be able to create element");

    let _ = element.set_attribute("href", uri);
    let _ = element.set_attribute("download", filename);

    let event = seed::document()
        .create_event("MouseEvents")
        .expect("should be able to call createEvent()")
        .dyn_into::<web_sys::MouseEvent>()
        .ok()
        .expect("should be a MouseEvent");
    event.init_mouse_event_with_can_bubble_arg_and_cancelable_arg("click", true, true);
    let _ = element.dispatch_event(&event);

    element.remove();
}

/// makes a file selector appear to let the user choose a file to upload
pub fn choose_upload(input_element_id : &str)
{
    let element = seed::document()
        .get_element_by_id(input_element_id)
        .unwrap();
    
    // simulate a click on it
    let event = seed::document()
        .create_event("MouseEvents")
        .expect("should be able to call createEvent()")
        .dyn_into::<web_sys::MouseEvent>()
        .ok()
        .expect("should be a MouseEvent");
    event.init_mouse_event_with_can_bubble_arg_and_cancelable_arg("click", true, true);
    let _ = element.dispatch_event(&event);   
}

/// starts a future to upload the text file and deliver it as a UploadText message 
pub fn upload_file(event : web_sys::Event, orders: &mut impl Orders<super::Msg>)
{
    let target = event.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().ok().unwrap();
    let file = target.files().unwrap().get(0).expect("should get a file");
    orders.perform_cmd(async move {
        let text = JsFuture::from(file.text())
            .await
            .expect("read file")
            .as_string()
            .expect("cast file text to String");
        super::Msg::UploadText(text)
    });
}
