// build.rs

extern crate winres;

fn main() {
  if cfg!(target_os = "windows") {
    let mut res = winres::WindowsResource::new();
    res.set_icon_with_id("icon.ico", "icon_1");
    res.compile().unwrap();
  }

  windows::build!{
    Windows::Win32::UI::WindowsAndMessaging::*,
    Windows::Win32::Foundation::*,
    Windows::Win32::System::Diagnostics::*,
    Windows::Win32::System::LibraryLoader::*,
    Windows::Win32::UI::Controls::*
  };
}