use winit::event_loop::EventLoop;

#[cfg(target_os = "linux")]
pub fn new_event_loop() -> EventLoop<()> {
    winit::platform::unix::EventLoopExtUnix::new_any_thread()
}

#[cfg(target_os = "windows")]
pub fn new_event_loop() -> EventLoop<()> {
    winit::platform::windows::EventLoopExtWindows::new_any_thread()
}

#[cfg(target_arch = "wasm32")]
pub fn new_event_loop() -> EventLoop<()> {
    unimplemented!("multi-threaded event loop in wasm32 is not supported yet")
}
