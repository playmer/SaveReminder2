// build.rs

extern crate winres;

fn main() {
  if cfg!(target_os = "windows") {
    let mut res = winres::WindowsResource::new();
    res.set_icon("icon.ico");
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