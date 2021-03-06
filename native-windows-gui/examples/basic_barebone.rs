/**
    A very simple application that show your name in a message box.

    This demo shows how to use NWG without the NativeUi trait boilerplate.
    Note that this way of doing things is alot less extensible and cannot make use of native windows derive.

    See `basic` for the NativeUi version and `basic_d` for the derive version
*/
extern crate native_windows_gui as nwg;
use std::rc::Rc;


fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let mut window = Default::default();
    let mut name_edit = Default::default();
    let mut hello_button = Default::default();

    nwg::Window::builder()
        .flags(nwg::WindowFlags::WINDOW | nwg::WindowFlags::VISIBLE)
        .size((300, 115))
        .position((300, 300))
        .title("Basic example")
        .build(&mut window)
        .unwrap();

    nwg::TextInput::builder()
        .size((280, 25))
        .position((10, 10))
        .text("Heisenberg")
        .focus(true)
        .parent(&window)
        .build(&mut name_edit)
        .unwrap();

    nwg::Button::builder()
        .size((280, 60))
        .position((10, 40))
        .text("Say my name")
        .parent(&window)
        .build(&mut hello_button)
        .unwrap();

    let window = Rc::new(window);
    let events_window = window.clone();

    let handler = nwg::full_bind_event_handler(&window.handle, move |evt, _evt_data, handle| {
        use nwg::Event as E;

        match evt {
            E::OnWindowClose => 
                if &handle == &events_window as &nwg::Window {
                    nwg::simple_message("Goodbye", &format!("Goodbye {}", name_edit.text()));
                    nwg::stop_thread_dispatch();
                },
            E::OnButtonClick => 
                if &handle == &hello_button {
                    nwg::simple_message("Hello", &format!("Hello {}", name_edit.text()));
                },
            _ => {}
        }
    });

    nwg::dispatch_thread_events();
    nwg::unbind_event_handler(&handler);
}

