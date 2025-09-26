use std::alloc::{GlobalAlloc, Layout, System};
#[cfg(windows)]
use std::ffi::c_void;
use std::ptr::NonNull;
#[cfg(windows)]
use windows_sys::Win32::Foundation::{GlobalFree, HGLOBAL};
#[cfg(windows)]
use windows_sys::Win32::System::Memory::{GlobalAlloc, GMEM_FIXED};

#[derive(thiserror::Error, Debug)]
#[error("failed to allocate memory")]
pub struct AllocError;

pub type Result<T> = std::result::Result<T, AllocError>;

/// The `Allocator` trait mirrors the `std::alloc::Allocator` from the standard library,
/// but it's implemented here as the latter is available only in the nightly version of Rust.
/// This trait provides the basic structure and functionality of an allocator.
pub trait Allocator {
    /// Attempts to allocate a block of memory following the given layout.
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>>;

    /// Deallocates the memory previously allocated by the `allocate` function.
    /// The `ptr` argument is a pointer to the memory to be deallocated.
    fn deallocate(&self, ptr: NonNull<u8>, layout: Layout);
}

impl Allocator for System {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>> {
        // TODO: Allow allocate with size zero
        assert!(layout.size() > 0);

        let raw_ptr = unsafe { self.alloc(layout) };
        if raw_ptr.is_null() {
            Err(AllocError)
        } else {
            let ptr = unsafe { std::slice::from_raw_parts_mut(raw_ptr, layout.size()) };
            Ok(NonNull::new(ptr).expect("unreachable"))
        }
    }

    fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        if layout.size() != 0 {
            unsafe { self.dealloc(ptr.as_ptr(), layout) }
        }
    }
}

/// `UkaAllocator` serves as an allocator for handling values specifically for the ukagaka DLL.
/// It follows a common specification found in ukagaka subsystems such as SHIORI, SAORI, MAKOTO, PLUGIN, and HEADLINE.
///
/// The Windows OS specification suggests that `GlobalAlloc(GMEM_FIXED)` be used for allocation
/// and `GlobalFree` be used for deallocation.
///
/// For non-Windows OS environments, the uka-rs specification suggests using libc's `malloc`/`free` for allocation and deallocation.
/// Please note that this is uka-rs's specific guideline and may not guarantee proper functionality with all ukagaka-related systems.
///
/// # Examples
///
/// ```rust
/// # use uka_util::alloc::{Allocator, UkaAllocator};
/// # use std::alloc::Layout;
/// # use std::mem::align_of;
/// # use std::ptr::copy_nonoverlapping;
/// #
/// let alloc = UkaAllocator::default();
/// let layout = Layout::from_size_align(4, align_of::<u8>()).unwrap();
///
/// let mut ptr = alloc.allocate(layout).unwrap();
/// unsafe {
///    copy_nonoverlapping([1u8, 2u8, 3u8, 4u8].as_ptr(), ptr.as_ptr() as *mut u8, layout.size());
///    assert_eq!([1u8, 2u8, 3u8, 4u8], ptr.as_ref());
/// }
///
/// alloc.deallocate(ptr.cast(), layout);
/// ```
#[derive(Default, Clone)]
pub struct UkaAllocator;

#[cfg(windows)]
impl Allocator for UkaAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>> {
        // TODO: Allow allocate with size zero
        assert!(layout.size() > 0);

        let hglobal = unsafe { GlobalAlloc(GMEM_FIXED, layout.size()) };
        if hglobal.is_null() {
            Err(AllocError)
        } else {
            let ptr = unsafe { std::slice::from_raw_parts_mut(hglobal as *mut u8, layout.size()) };
            Ok(NonNull::new(ptr).expect("unreachable"))
        }
    }

    fn deallocate(&self, ptr: NonNull<u8>, _layout: Layout) {
        let hglobal = HGLOBAL(ptr.as_ptr() as *mut c_void);
        // FIXME: Error returns if memory is successfully released.
        let _ = unsafe { GlobalFree(hglobal) };
    }
}

#[cfg(not(windows))]
impl Allocator for UkaAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>> {
        // TODO: Allow allocate with size zero
        assert!(layout.size() > 0);

        let raw_ptr = unsafe { libc::malloc(layout.size()) };
        if raw_ptr.is_null() {
            Err(AllocError)
        } else {
            let ptr = unsafe { std::slice::from_raw_parts_mut(raw_ptr as *mut u8, layout.size()) };
            Ok(NonNull::new(ptr).expect("unreachable"))
        }
    }

    fn deallocate(&self, ptr: NonNull<u8>, _layout: Layout) {
        unsafe { libc::free(ptr.as_ptr() as *mut libc::c_void) };
    }
}
