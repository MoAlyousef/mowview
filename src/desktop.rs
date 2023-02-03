use livid_desktop::wv;

pub struct WebView {
    wv: wv::Webview,
}

impl WebView {
    pub fn from(view: wv::Webview) -> Self {
        WebView {
            wv: view,
        }
    }

    pub fn load_url(&self, url: &str) {
        self.wv.clone().navigate(url);
    }
}