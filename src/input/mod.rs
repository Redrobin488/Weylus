pub mod device;
pub mod mouse_device;

#[cfg(target_os = "linux")]
pub mod uinput_device;
