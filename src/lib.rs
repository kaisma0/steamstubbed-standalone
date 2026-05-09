#![allow(internal_features)]
#![feature(link_llvm_intrinsics)]

use std::sync::{OnceLock, atomic::AtomicU32};

use minhook::MinHook;
use winapi::{shared::minwindef::{BOOL, DWORD, HMODULE, LPVOID, TRUE}, um::{libloaderapi::DisableThreadLibraryCalls, memoryapi::VirtualProtect, winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, PAGE_EXECUTE_READWRITE}}};

pub mod proxy;
mod exports;

const MAX_COUNT: u32 = 1;

static COUNT: AtomicU32 = AtomicU32::new(0);
static HOOK_ORIG: OnceLock<unsafe extern "system" fn() -> DWORD> = OnceLock::new();

unsafe extern "system" {
    #[link_name = "llvm.returnaddress"]
    fn return_address(level: i32) -> *const u8;
}

#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetTickCount() -> DWORD;
}

#[unsafe(no_mangle)]
extern "system" fn DllMain(_: HMODULE, reason: DWORD, _reserved: LPVOID) -> BOOL {    
    match reason {
        DLL_PROCESS_ATTACH => unsafe {
            DisableThreadLibraryCalls(std::ptr::null_mut());
            
            let orig = MinHook::create_hook(
                GetTickCount as *mut _, 
                hk_get_tick_count as *mut _
            );

            if let Ok(orig) = orig {
                HOOK_ORIG.set(std::mem::transmute(orig)).unwrap();
                let _ = MinHook::enable_hook(GetTickCount as *mut _);
            }
        },
        DLL_PROCESS_DETACH => unsafe {
            proxy::cleanup_proxied_dll();
        },
        _ => {}
    }

    TRUE
}

#[inline(never)]
extern "system" fn hk_get_tick_count() -> DWORD {
    unsafe {
        let start = return_address(0) as usize;
        let end = start + 128;

        let mut old_protect: DWORD = 0;
        VirtualProtect(
            start as LPVOID,
            end - start,
            PAGE_EXECUTE_READWRITE,
            &mut old_protect,
        );

        let sig = skidscan::signature!("44 0F B6 F8 3C 30 0F 84");
        let found = sig.scan_ptr(start as *mut u8, end as *mut u8);

        if let Some(addr) = found {
            *addr.add(6) = 0x90; // nop
            *addr.add(7) = 0xE9; // jmp

            let count = COUNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
            if count >= MAX_COUNT {
                let _ = MinHook::disable_hook(GetTickCount as *mut _);
            }
        }

        VirtualProtect(
            start as LPVOID,
            end - start,
            old_protect,
            &mut old_protect,
        );
    };

    unsafe {
        HOOK_ORIG.get().unwrap()()
    }
}
