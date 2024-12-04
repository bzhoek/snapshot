use headless_chrome::{Browser, LaunchOptionsBuilder};
use headless_chrome::protocol::cdp::Page;
use url::Url;

pub fn make_snapshot(url: &str, headless: bool) -> anyhow::Result<()> {
  let width = 592;
  let height = 1080;

  let url = Url::parse(url).unwrap();
  let options = LaunchOptionsBuilder::default()
    .headless(headless)
    .window_size(Some((width, height)))
    .args(vec!["--force-device-scale-factor=1".as_ref()])
    .build()?;
  let browser = Browser::new(options)?;
  let tab = browser.new_tab()?;

  let x = tab.navigate_to(url.as_str())?;
  tab.wait_until_navigated()?;

  let png_data = tab.find_element("body")?.capture_screenshot(Page::CaptureScreenshotFormatOption::Png)?;
  // let png_data = tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Png, None, None, true)?;
  std::fs::write("target/screenshot.png", &png_data)?;

  Ok(())
}