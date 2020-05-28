#![allow(non_camel_case_types, non_snake_case, unused)]

use winapi::ctypes::c_void;
use winapi::shared::minwindef::UINT;
use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::{BSTR, VARIANT_BOOL};
use winapi::um::oaidl::{IDispatch, IDispatchVtbl, LPDISPATCH, VARIANT};
use winapi::um::unknwnbase::LPUNKNOWN;
use winapi::um::winnt::LPCWSTR;
use winapi::{ENUM, RIDL, STRUCT};

macro_rules! UNION {
    ($(#[$attrs:meta])* union $name:ident {
        [$stype:ty; $ssize:expr],
        $($variant:ident $variant_mut:ident: $ftype:ty,)+
    }) => (
        #[repr(C)] $(#[$attrs])*
        pub struct $name([$stype; $ssize]);
        impl Copy for $name {}
        impl Clone for $name {
            #[inline]
            fn clone(&self) -> $name { *self }
        }
        #[cfg(feature = "impl-default")]
        impl Default for $name {
            #[inline]
            fn default() -> $name { unsafe { $crate::_core::mem::zeroed() } }
        }
        impl $name {$(
            #[inline]
            pub unsafe fn $variant(&self) -> &$ftype {
                &*(self as *const _ as *const $ftype)
            }
            #[inline]
            pub unsafe fn $variant_mut(&mut self) -> &mut $ftype {
                &mut *(self as *mut _ as *mut $ftype)
            }
        )+}
    );
    ($(#[$attrs:meta])* union $name:ident {
        [$stype32:ty; $ssize32:expr] [$stype64:ty; $ssize64:expr],
        $($variant:ident $variant_mut:ident: $ftype:ty,)+
    }) => (
        #[repr(C)] $(#[$attrs])* #[cfg(target_pointer_width = "32")]
        pub struct $name([$stype32; $ssize32]);
        #[repr(C)] $(#[$attrs])* #[cfg(target_pointer_width = "64")]
        pub struct $name([$stype64; $ssize64]);
        impl Copy for $name {}
        impl Clone for $name {
            #[inline]
            fn clone(&self) -> $name { *self }
        }
        #[cfg(feature = "impl-default")]
        impl Default for $name {
            #[inline]
            fn default() -> $name { unsafe { $crate::_core::mem::zeroed() } }
        }
        impl $name {$(
            #[inline]
            pub unsafe fn $variant(&self) -> &$ftype {
                &*(self as *const _ as *const $ftype)
            }
            #[inline]
            pub unsafe fn $variant_mut(&mut self) -> &mut $ftype {
                &mut *(self as *mut _ as *mut $ftype)
            }
        )+}
    );
}

RIDL! {#[uuid(0x56fdf342, 0xfd6d, 0x11d0, 0x95, 0x8a, 0x00, 0x60, 0x97, 0xc9, 0xa0, 0x90)]
interface ITaskbarList(ITaskbarListVtbl): IUnknown(IUnknownVtbl) {
    fn HrInit(
    ) -> HRESULT,
    fn AddTab(
        hwnd: i32,
    ) -> HRESULT,
    fn DeleteTab(
        hwnd: i32,
    ) -> HRESULT,
    fn ActivateTab(
        hwnd: i32,
    ) -> HRESULT,
    fn SetActivateAlt(
        hwnd: i32,
    ) -> HRESULT,
}}

RIDL! {#[uuid(0x00000000, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IUnknown(IUnknownVtbl) {
    fn QueryInterface(
        riid: *const GUID,
        ppvObject: *mut *mut c_void,
    ) -> HRESULT,
    fn AddRef(
    ) -> u32,
    fn Release(
    ) -> u32,
}}

pub type GUID = __MIDL___MIDL_itf_TaskbarLib_0006_0001_0001;

STRUCT! {struct __MIDL___MIDL_itf_TaskbarLib_0006_0001_0001 {
    Data1: u32,
    Data2: u16,
    Data3: u16,
    Data4: [u8; 8],
}}

RIDL! {#[uuid(0x602d4995, 0xb13a, 0x429b, 0xa6, 0x6e, 0x19, 0x35, 0xe4, 0x4f, 0x43, 0x17)]
interface ITaskbarList2(ITaskbarList2Vtbl): ITaskbarList(ITaskbarListVtbl) {
    fn MarkFullscreenWindow(
        hwnd: i32,
        fFullscreen: i32,
    ) -> HRESULT,
}}

STRUCT! {struct tagTHUMBBUTTON {
    dwMask: u32,
    iId: UINT,
    iBitmap: UINT,
    hIcon: LPUNKNOWN,
    szTip: [u16; 260],
    dwFlags: u32,
}}

RIDL! {#[uuid(0xea1afb91, 0x9e28, 0x4b86, 0x90, 0xe9, 0x9e, 0x9f, 0x8a, 0x5e, 0xef, 0xaf)]
interface ITaskbarList3(ITaskbarList3Vtbl): ITaskbarList2(ITaskbarList2Vtbl) {
    fn SetProgressValue(
        hwnd: i32,
        ullCompleted: u64,
        ullTotal: u64,
    ) -> HRESULT,
    fn SetProgressState(
        hwnd: i32,
        tbpFlags: TBPFLAG,
    ) -> HRESULT,
    fn RegisterTab(
        hwndTab: i32,
        hwndMDI: wireHWND,
    ) -> HRESULT,
    fn UnregisterTab(
        hwndTab: i32,
    ) -> HRESULT,
    fn SetTabOrder(
        hwndTab: i32,
        hwndInsertBefore: i32,
    ) -> HRESULT,
    fn SetTabActive(
        hwndTab: i32,
        hwndMDI: i32,
        tbatFlags: TBATFLAG,
    ) -> HRESULT,
    fn ThumbBarAddButtons(
        hwnd: i32,
        cButtons: UINT,
        pButton: *const tagTHUMBBUTTON,
    ) -> HRESULT,
    fn ThumbBarUpdateButtons(
        hwnd: i32,
        cButtons: UINT,
        pButton: *const tagTHUMBBUTTON,
    ) -> HRESULT,
    fn ThumbBarSetImageList(
        hwnd: i32,
        himl: LPUNKNOWN,
    ) -> HRESULT,
    fn SetOverlayIcon(
        hwnd: i32,
        hIcon: LPUNKNOWN,
        pszDescription: LPCWSTR,
    ) -> HRESULT,
    fn SetThumbnailTooltip(
        hwnd: i32,
        pszTip: LPCWSTR,
    ) -> HRESULT,
    fn SetThumbnailClip(
        hwnd: i32,
        prcClip: *const tagRECT,
    ) -> HRESULT,
}}

ENUM! {enum TBPFLAG {
    TBPF_NOPROGRESS = 0,
    TBPF_INDETERMINATE = 1,
    TBPF_NORMAL = 2,
    TBPF_ERROR = 4,
    TBPF_PAUSED = 8,
}}

pub type wireHWND = *mut _RemotableHandle;

STRUCT! {struct _RemotableHandle {
    fContext: i32,
    u: __MIDL_IWinTypes_0009,
}}

UNION! {union __MIDL_IWinTypes_0009 {
    [u32; 1],
    hInproc hInproc_mut: i32,
    hRemote hRemote_mut: i32,
}}

ENUM! {enum TBATFLAG {
    TBATF_USEMDITHUMBNAIL = 1,
    TBATF_USEMDILIVEPREVIEW = 2,
}}

STRUCT! {struct tagRECT {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}}

// Implements ITaskbarList3
RIDL! {#[uuid(0x56fdf344, 0xfd6d, 0x11d0, 0x95, 0x8a, 0x00, 0x60, 0x97, 0xc9, 0xa0, 0x90)]
class TaskbarList; }
