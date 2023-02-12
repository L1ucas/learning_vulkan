use winit::event_loop::EventLoop;
mod window;

fn main() {
    let event_loop = EventLoop::new();
    let _window = window::VulkanApp::init_window(&event_loop);
    window::VulkanApp::main_loop(event_loop);
}
