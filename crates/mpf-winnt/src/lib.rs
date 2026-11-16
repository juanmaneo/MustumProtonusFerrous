//! Win32 ABI structures — the `include/winnt.h` / `winternl.h` equivalents.
//!
//! x64 layouts, matching reference Wine's `include/winnt.h` and
//! `dlls/ntdll/ntapi.c` (see `repos/wine/include/winnt.h`). Offsets of the
//! fields we rely on are pinned by unit tests with `core::mem::offset_of!`.

use core::ffi::c_void;

/// UNICODE_STRING as laid out in the ABI.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UnicodeString {
    pub length: u16,
    pub maximum_length: u16,
    pub buffer: *mut u16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ListEntry {
    pub flink: *mut ListEntry,
    pub blink: *mut ListEntry,
}

impl ListEntry {
    pub const fn zeroed() -> Self {
        ListEntry {
            flink: core::ptr::null_mut(),
            blink: core::ptr::null_mut(),
        }
    }

    /// Initializes an empty circular list head (points at itself).
    pub fn init_head(&mut self) {
        self.flink = self;
        self.blink = self;
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct RtlCriticalSection {
    pub debug_info: *mut c_void,     // 0x00
    pub lock_count: i32,             // 0x08
    pub recursion_count: i32,        // 0x0C
    pub owning_thread: *mut c_void,  // 0x10
    pub lock_semaphore: *mut c_void, // 0x18
    pub spin_count: usize,           // 0x20
}

impl RtlCriticalSection {
    pub const fn zeroed() -> Self {
        RtlCriticalSection {
            debug_info: core::ptr::null_mut(),
            lock_count: -1, // unlocked, as in RtlInitializeCriticalSection
            recursion_count: 0,
            owning_thread: core::ptr::null_mut(),
            lock_semaphore: core::ptr::null_mut(),
            spin_count: 0,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ExceptionRecord {
    pub exception_code: u32,
    pub exception_flags: u32,
    pub exception_record: *mut ExceptionRecord,
    pub exception_address: *mut c_void,
    pub number_of_parameters: u32,
    pub exception_information: [usize; 15],
}

pub const EXCEPTION_MAXIMUM_PARAMETERS: usize = 15;

#[repr(C)]
#[derive(Debug)]
pub struct PebLdrData {
    pub length: u32,                                    // 0x00
    pub initialized: u8,                                // 0x04
    pub ss_handle: *mut c_void,                         // 0x08
    pub in_load_order_module_list: ListEntry,           // 0x10
    pub in_memory_order_module_list: ListEntry,         // 0x20
    pub in_initialization_order_module_list: ListEntry, // 0x30
}

impl PebLdrData {
    pub fn new() -> Self {
        let mut ldr = PebLdrData {
            length: core::mem::size_of::<PebLdrData>() as u32,
            initialized: 1,
            ss_handle: core::ptr::null_mut(),
            in_load_order_module_list: ListEntry::zeroed(),
            in_memory_order_module_list: ListEntry::zeroed(),
            in_initialization_order_module_list: ListEntry::zeroed(),
        };
        ldr.in_load_order_module_list.init_head();
        ldr.in_memory_order_module_list.init_head();
        ldr.in_initialization_order_module_list.init_head();
        ldr
    }
}

impl Default for PebLdrData {
    fn default() -> Self {
        Self::new()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct LdrDataTableEntry {
    pub in_load_order_links: ListEntry,           // 0x00
    pub in_memory_order_links: ListEntry,         // 0x10
    pub in_initialization_order_links: ListEntry, // 0x20
    pub dll_base: *mut c_void,                    // 0x30
    pub entry_point: *mut c_void,                 // 0x38
    pub size_of_image: u32,                       // 0x40
    pub full_dll_name: UnicodeString,             // 0x48
    pub base_dll_name: UnicodeString,             // 0x58
}

#[repr(C)]
#[derive(Debug)]
pub struct RtlUserProcessParameters {
    pub maximum_length: u32,         // 0x00
    pub length: u32,                 // 0x04
    pub flags: u32,                  // 0x08
    pub debug_flags: u32,            // 0x0C
    pub console_handle: *mut c_void, // 0x10
    pub console_flags: u32,          // 0x18
    pub _pad0: [u8; 4],              // 0x1C
                                     // ... remaining fields omitted; we only need the prefix
}

/// Process Environment Block, x64. Fields up to `loader_lock` are real;
/// everything after is zeroed padding kept for layout fidelity.
#[repr(C)]
#[derive(Debug)]
pub struct Peb {
    pub inherited_address_space: u8,                       // 0x00
    pub read_image_file_exec_options: u8,                  // 0x01
    pub being_debugged: u8,                                // 0x02
    pub bit_field: u8,                                     // 0x03
    pub _pad0: [u8; 4],                                    // 0x04
    pub mutant: *mut c_void,                               // 0x08
    pub image_base_address: *mut c_void,                   // 0x10
    pub ldr: *mut PebLdrData,                              // 0x18
    pub process_parameters: *mut RtlUserProcessParameters, // 0x20
    pub sub_system_data: *mut c_void,                      // 0x28
    pub process_heap: *mut c_void,                         // 0x30
    pub fast_peb_lock: *mut RtlCriticalSection,            // 0x38
    pub _pad1: [u8; 0x18],                                 // 0x40..0x58
    pub kernel_callback_table: *mut c_void,                // 0x58
    pub _pad2: [u8; 0x20],                                 // 0x60..0x80
    pub ansi_code_page_data: *mut c_void,                  // 0x80
    pub oem_code_page_data: *mut c_void,                   // 0x88
    pub unicode_case_table_data: *mut c_void,              // 0x90
    pub number_of_processors: u32,                         // 0x98
    pub nt_global_flag: u32,                               // 0x9C
    pub _pad3: [u8; 0x50],                                 // 0xA0..0xF0
    pub loader_lock: *mut RtlCriticalSection,              // 0xF0
    pub os_major_version: u32,                             // 0xF8
    pub os_minor_version: u32,                             // 0xFC
    pub os_build_number: u16,                              // 0x100
    pub os_csd_version: u16,                               // 0x102
    pub os_platform_id: u32,                               // 0x104
    pub image_subsystem: u32,                              // 0x108
    pub image_subsystem_major_version: u32,                // 0x10C
    pub image_subsystem_minor_version: u32,                // 0x110
    pub _pad4: [u8; 4],                                    // 0x114
    pub image_process_affinity_mask: usize,                // 0x118
    pub gdi_handle_buffer: [u32; 60],                      // 0x120..0x210
    pub _pad5: [u8; 0x100],                                // reserved tail (post-init fields)
}

impl Peb {
    pub fn new(
        image_base: *mut c_void,
        ldr: *mut PebLdrData,
        process_parameters: *mut RtlUserProcessParameters,
        process_heap: *mut c_void,
    ) -> Box<Self> {
        Box::new(Peb {
            inherited_address_space: 0,
            read_image_file_exec_options: 0,
            being_debugged: 0,
            bit_field: 0,
            _pad0: [0; 4],
            mutant: core::ptr::null_mut(),
            image_base_address: image_base,
            ldr,
            process_parameters,
            sub_system_data: core::ptr::null_mut(),
            process_heap,
            fast_peb_lock: core::ptr::null_mut(),
            _pad1: [0; 0x18],
            kernel_callback_table: core::ptr::null_mut(),
            _pad2: [0; 0x20],
            ansi_code_page_data: core::ptr::null_mut(),
            oem_code_page_data: core::ptr::null_mut(),
            unicode_case_table_data: core::ptr::null_mut(),
            number_of_processors: 1,
            nt_global_flag: 0,
            _pad3: [0; 0x50],
            loader_lock: core::ptr::null_mut(),
            os_major_version: 10,
            os_minor_version: 0,
            os_build_number: 19045,
            os_csd_version: 0,
            os_platform_id: 2,  // VER_PLATFORM_WIN32_NT
            image_subsystem: 3, // IMAGE_SUBSYSTEM_WINDOWS_CUI
            image_subsystem_major_version: 10,
            image_subsystem_minor_version: 0,
            _pad4: [0; 4],
            image_process_affinity_mask: 0,
            gdi_handle_buffer: [0; 60],
            _pad5: [0; 0x100],
        })
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NtTib {
    pub exception_list: *mut c_void,         // 0x00
    pub stack_base: *mut c_void,             // 0x08
    pub stack_limit: *mut c_void,            // 0x10
    pub sub_system_tib: *mut c_void,         // 0x18
    pub fiber_data: *mut c_void,             // 0x20
    pub arbitrary_user_pointer: *mut c_void, // 0x28
    pub self_: *mut NtTib,                   // 0x30
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ClientId {
    pub unique_process: *mut c_void, // 0x40
    pub unique_thread: *mut c_void,  // 0x48
}

/// Thread Environment Block, x64 prefix (fields we set).
#[repr(C)]
#[derive(Debug)]
pub struct Teb {
    pub nt_tib: NtTib,                             // 0x00..0x38
    pub environment_pointer: *mut c_void,          // 0x38
    pub client_id: ClientId,                       // 0x40..0x50
    pub active_rpc_handle: *mut c_void,            // 0x50
    pub thread_local_storage_pointer: *mut c_void, // 0x58
    pub process_environment_block: *mut Peb,       // 0x60
    pub last_error_value: u32,                     // 0x68
    pub count_of_owned_critical_sections: u32,     // 0x6C
    pub _pad: [u8; 0x100],                         // reserve tail space
}

impl Teb {
    pub fn new(peb: *mut Peb) -> Box<Self> {
        let mut teb = Box::new(Teb {
            nt_tib: NtTib {
                exception_list: core::ptr::null_mut(),
                stack_base: core::ptr::null_mut(),
                stack_limit: core::ptr::null_mut(),
                sub_system_tib: core::ptr::null_mut(),
                fiber_data: core::ptr::null_mut(),
                arbitrary_user_pointer: core::ptr::null_mut(),
                self_: core::ptr::null_mut(),
            },
            environment_pointer: core::ptr::null_mut(),
            client_id: ClientId {
                unique_process: core::ptr::null_mut(),
                unique_thread: core::ptr::null_mut(),
            },
            active_rpc_handle: core::ptr::null_mut(),
            thread_local_storage_pointer: core::ptr::null_mut(),
            process_environment_block: peb,
            last_error_value: 0,
            count_of_owned_critical_sections: 0,
            _pad: [0; 0x100],
        });
        // NtTib.Self points at the TEB itself.
        let self_ptr = &mut *teb as *mut Teb as *mut NtTib;
        teb.nt_tib.self_ = self_ptr;
        teb
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::offset_of;

    #[test]
    fn peb_offsets_match_winnt_h_x64() {
        assert_eq!(offset_of!(Peb, image_base_address), 0x10);
        assert_eq!(offset_of!(Peb, ldr), 0x18);
        assert_eq!(offset_of!(Peb, process_parameters), 0x20);
        assert_eq!(offset_of!(Peb, process_heap), 0x30);
        assert_eq!(offset_of!(Peb, fast_peb_lock), 0x38);
        assert_eq!(offset_of!(Peb, kernel_callback_table), 0x58);
        assert_eq!(offset_of!(Peb, number_of_processors), 0x98);
        assert_eq!(offset_of!(Peb, loader_lock), 0xF0);
        assert_eq!(offset_of!(Peb, os_major_version), 0xF8);
        assert_eq!(offset_of!(Peb, image_subsystem), 0x108);
        assert_eq!(offset_of!(Peb, gdi_handle_buffer), 0x120);
    }

    #[test]
    fn teb_offsets_match_x64() {
        assert_eq!(offset_of!(NtTib, stack_base), 0x08);
        assert_eq!(offset_of!(NtTib, self_), 0x30);
        assert_eq!(offset_of!(Teb, client_id), 0x40);
        assert_eq!(offset_of!(Teb, process_environment_block), 0x60);
        assert_eq!(offset_of!(Teb, last_error_value), 0x68);
    }

    #[test]
    fn ldr_offsets_match_x64() {
        assert_eq!(offset_of!(PebLdrData, in_load_order_module_list), 0x10);
        assert_eq!(offset_of!(PebLdrData, in_memory_order_module_list), 0x20);
        assert_eq!(
            offset_of!(PebLdrData, in_initialization_order_module_list),
            0x30
        );
        assert_eq!(offset_of!(LdrDataTableEntry, dll_base), 0x30);
        assert_eq!(offset_of!(LdrDataTableEntry, entry_point), 0x38);
        assert_eq!(offset_of!(LdrDataTableEntry, size_of_image), 0x40);
        assert_eq!(offset_of!(LdrDataTableEntry, full_dll_name), 0x48);
    }

    #[test]
    fn critical_section_and_exception_layouts() {
        assert_eq!(offset_of!(RtlCriticalSection, lock_count), 0x08);
        assert_eq!(offset_of!(RtlCriticalSection, recursion_count), 0x0C);
        assert_eq!(offset_of!(RtlCriticalSection, owning_thread), 0x10);
        assert_eq!(offset_of!(RtlCriticalSection, spin_count), 0x20);
        assert_eq!(offset_of!(ExceptionRecord, exception_record), 0x08);
        assert_eq!(offset_of!(ExceptionRecord, exception_address), 0x10);
        assert_eq!(offset_of!(ExceptionRecord, exception_information), 0x20);
        assert_eq!(EXCEPTION_MAXIMUM_PARAMETERS, 15);
        let cs = RtlCriticalSection::zeroed();
        assert_eq!(cs.lock_count, -1);
    }

    #[test]
    fn peb_and_teb_constructors_wire_pointers() {
        let ldr = Box::new(PebLdrData::new());
        let ldr_ptr = Box::into_raw(ldr);
        let peb = Peb::new(
            0x1400_0000 as *mut c_void,
            ldr_ptr,
            core::ptr::null_mut(),
            0x100_0000 as *mut c_void,
        );
        assert_eq!(peb.image_base_address as usize, 0x1400_0000);
        assert_eq!(peb.ldr, ldr_ptr);
        assert_eq!(peb.os_major_version, 10);
        let peb_ptr = Box::into_raw(peb);
        let ldr_ptr = unsafe { (*peb_ptr).ldr };
        let teb = Teb::new(peb_ptr);
        assert_eq!(teb.process_environment_block, peb_ptr);
        assert_eq!(
            teb.nt_tib.self_ as usize,
            &*teb as *const Teb as *const NtTib as usize
        );
        unsafe {
            drop(Box::from_raw(ldr_ptr));
            drop(Box::from_raw(peb_ptr));
        }
    }

    #[test]
    fn list_head_points_at_itself() {
        let mut head = ListEntry::zeroed();
        head.init_head();
        assert_eq!(head.flink, &head as *const ListEntry as *mut ListEntry);
        assert_eq!(head.blink, &head as *const ListEntry as *mut ListEntry);
    }
}
