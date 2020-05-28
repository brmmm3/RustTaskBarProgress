mod TaskBarLib;

use fltk::{button::*, window::*};

fn main() {
    let mut wnd = Window::default()
        .with_size(600, 228)
        .center_screen()
        .with_label("Example");
    wnd.end();
    wnd.show();

    unsafe {
        let hr = winapi::um::objbase::CoInitialize(std::ptr::null_mut());
        assert!(winapi::shared::winerror::SUCCEEDED(hr));

        let mut taskbar: *mut winapi::ctypes::c_void = std::ptr::null_mut();
        let hr = winapi::um::combaseapi::CoCreateInstance(
            &<TaskBarLib::TaskbarList as winapi::Class>::uuidof(),
            std::ptr::null_mut(),
            winapi::um::combaseapi::CLSCTX_ALL,
            &<TaskBarLib::ITaskbarList3 as winapi::Interface>::uuidof(),
            &mut taskbar,
        );
        let taskbar = &*(taskbar as *mut TaskBarLib::ITaskbarList3);
        taskbar.HrInit();
        let hWnd: i32 = wnd.raw_handle() as i32;
        taskbar.SetProgressState(hWnd, TaskBarLib::TBPF_NORMAL);
        for i in 1..100 {
            taskbar.SetProgressValue(hWnd, i, 100);
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        assert!(winapi::shared::winerror::SUCCEEDED(hr));
    }
}
