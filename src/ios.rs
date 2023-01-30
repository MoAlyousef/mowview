extern crate objc;

use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};
use std::ffi::CString;
use std::os::raw::c_void;

pub struct WebView {
    wv: *mut c_void,
}
impl WebView {
    pub fn from(view: *mut c_void) -> Self {
        WebView {
            wv: view,
        }
    }

    pub fn load_url(&self, url: &str) {
        unsafe {
            let url = CString::new(url).unwrap();
            let url_str: *mut Object = msg_send![class!(NSString), stringWithUTF8String:url.as_ptr()];
            let ns_url: *mut Object = msg_send![class!(NSURL), URLWithString:url_str];
            let ns_url_req: *mut Object = msg_send![class!(NSURLRequest), requestWithURL:ns_url];
            let _: () = msg_send![self.wv as *mut Object, loadRequest:ns_url_req];
        }
    }
}