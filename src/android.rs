use jni::objects::JObject;
use jni::JNIEnv;
use jni::JavaVM;
use std::os::raw::c_void;

pub struct WebView {
    vm: JavaVM,
    wv: *mut c_void,
}
impl WebView {
    pub fn from(env: JNIEnv, view: *mut c_void) -> Self {
        let vm = env.get_java_vm().unwrap();
        WebView {
            vm,
            wv: view,
        }
    }

    pub fn load_url(&self, url: &str) {
        let env = self.vm.get_env().unwrap();
        let view = unsafe { JObject::from_raw(self.wv as _) };
        let url = env.new_string(url).unwrap();
        env.call_method(view, "loadUrl", "(Ljava/lang/String;)V", &[url.into()]).unwrap();
    }
}