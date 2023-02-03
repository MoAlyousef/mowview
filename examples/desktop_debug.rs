use livid_desktop::{App, Settings};

fn main() {
    let a = App::new(Settings {
        w: 600,
        h: 400,
        title: "My App",
        fixed: true,
        ..Default::default()
    });
    let wv = a.get_webview();
    let mwv = mowview::WebView::from(wv.clone());
    mwv.load_url("https://www.google.com");
    wv.run();
}