use crate::alloc::Allocator;
use crate::ptr::OwnedPtr;
use std::ptr::NonNull;
#[cfg(windows)]
use windows_sys::Win32::Foundation::HGLOBAL;

/// `RawPtr<T>` handles raw pointers to values outside of Rust's memory management. For example, `std::mem::forget` values or externally allocated memory.
///
/// Note that due to its nature, the pointers handled by `RawPtr<T>` must be freed, otherwise it will lead to memory leaks.
///
/// # Safety
///
/// If it is to be handled safely, it must be ensured that all of the following are true:
///
/// 1. The pointer is not NULL.
/// 2. The pointer is properly aligned for type `T`.
/// 3. The memory referenced by the pointer must not be accessed through any other pointer (not derived from the return value) for the duration of the `RawPtr<T>`. Both read and write accesses are forbidden.
/// 4. The lifetime `'a` returned by methods is not outlived by the `RawPtr<T>` which would lead to dangling references.
pub struct RawPtr<T: ?Sized> {
    /// A non-null raw pointer (`NonNull<T>`) that points to a memory block.
    /// This memory block could be a single `T` object or an array of `T` (slice `&[T]` or `&mut [T]`).
    /// The total size in bytes of the pointed memory block is represented by the `size` field.
    pub(in crate::ptr) ptr: NonNull<T>,

    /// The total size in bytes of the memory block that the pointer `ptr` refers to.
    /// It represents the product of the size of the pointed type `T` and the number of `T` elements,
    /// in case `T` is an array or slice type. For a single `T` object, `size` should be equivalent to `std::mem::size_of::<T>()`.
    pub(in crate::ptr) size: usize,
}

impl<T> RawPtr<T> {
    /// Returns reference to T
    ///
    /// # Safety
    ///
    /// This function is unsafe because it dereferences a raw pointer, which could potentially lead to undefined behavior.
    ///
    /// To ensure safe usage of this function, the following conditions must be met:
    ///
    /// 1. The pointer must be properly aligned. Misaligned pointers can lead to undefined behavior.
    ///
    /// 2. The pointer must be "dereferenceable" as defined in the Rust documentation. This means the pointer is not null, does not dangle, and is not unaligned.
    ///
    /// 3. The pointer must point to an initialized instance of T. Accessing uninitialized memory can lead to undefined behavior.
    ///
    /// 4. The aliasing rules of Rust must be enforced. The pointer must not be used to mutate data while this reference exists, unless the data lies inside an `UnsafeCell<U>`.
    ///
    /// Violating these conditions can lead to undefined behavior, so be very cautious when using this function.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use uka_util::ptr::RawPtr;
    /// #
    /// let value = &1;
    /// let ptr = RawPtr::from(value as *const i32);
    /// unsafe {
    ///     assert_eq!(ptr.as_ref(), value);
    /// }
    /// ```
    pub unsafe fn as_ref(&self) -> &T {
        self.ptr.as_ref()
    }

    /// Returns mutable reference to `T`
    ///
    /// # Safety
    ///
    /// This function is unsafe because it dereferences a raw pointer, which could potentially lead to undefined behavior.
    ///
    /// To ensure safe usage of this function, the following conditions must be met:
    ///
    /// 1. The pointer must be properly aligned. Misaligned pointers can lead to undefined behavior.
    ///
    /// 2. The pointer must be "dereferenceable" as defined in the Rust documentation. This means the pointer is not null, does not dangle, and is not unaligned.
    ///
    /// 3. The pointer must point to an initialized instance of T. Accessing uninitialized memory can lead to undefined behavior.
    ///
    /// 4. The aliasing rules of Rust must be enforced. The pointer must not be used to mutate data while this reference exists, unless the data lies inside an `UnsafeCell<U>`.
    ///
    /// Violating these conditions can lead to undefined behavior, so be very cautious when using this function.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use uka_util::ptr::RawPtr;
    /// #
    /// let value = Box::new(1);
    /// let ptr = Box::into_raw(value);
    /// let mut raw = RawPtr::from(ptr);
    /// unsafe {
    ///     *raw.as_mut() = 2;
    ///     assert_eq!(raw.as_ref(), &2);
    /// }
    /// ```
    pub unsafe fn as_mut(&mut self) -> &mut T {
        self.ptr.as_mut()
    }

    /// Returns the raw pointer of `T`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use uka_util::ptr::RawPtr;
    /// #
    /// let value = &1;
    /// let ptr = RawPtr::from(value as *const i32);
    /// assert_eq!(ptr.as_ptr(), value as *const i32);
    /// ```
    pub fn as_ptr(&self) -> *const T {
        self.ptr.as_ptr()
    }

    /// Returns mutable pointer of `T`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use uka_util::ptr::RawPtr;
    /// #
    /// let value = Box::new(1);
    /// let ptr = Box::into_raw(value);
    /// let raw = RawPtr::from(ptr);
    /// unsafe {
    ///     *raw.as_mut_ptr() = 2;
    /// }
    /// assert_eq!(raw.as_mut_ptr(), ptr);
    /// ```
    pub fn as_mut_ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }

    /// Convert from raw pointers to pointer types that emulate ownership.
    /// Use the same `Allocator` that allocated the memory area the pointer points to.
    /// It is unsafe to specify a different `Allocator`.
    pub fn to_owned<A: Allocator + Default>(self) -> OwnedPtr<T, A> {
        self.into()
    }
}

impl<T> RawPtr<[T]> {
    /// `RawPtr<[T]>` from `*mut T` with length.
    ///
    /// # Safety
    ///
    /// This function is safe to use if the following conditions are met:
    ///
    /// 1. `ptr` must be a valid pointer for the type `T`, meaning it must not be null and it must be properly aligned.
    /// 2. `ptr` must point to a memory region of at least `len * size_of::<T>()` bytes and this region must be initialized properly.
    /// 3. The memory `ptr` points to must be unique for the duration of the `RawPtr<T>`, i.e., no other code may hold a reference to the same memory region during the lifetime of the created `RawPtr<T>`.
    /// 4. The memory region `ptr` points to must not cross over any allocated object boundaries.
    /// 5. `len` must accurately represent the number of contiguous elements of type `T` that `ptr` points to.
    /// 6. The total size `len * size_of::<T>()` must not be larger than `isize::MAX`. In other words, adding that size to `ptr` must not result in a wrap around the address space.
    ///
    /// When these conditions are upheld, using `RawPtr::from_raw_parts` will not cause undefined behavior.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use uka_util::ptr::RawPtr;
    /// #
    /// let mut value = [1, 2, 3];
    /// unsafe {
    ///     let ptr = RawPtr::from_raw_parts(value.as_mut_ptr(), value.len());
    ///     assert_eq!(ptr.as_slice(), &value[..]);
    /// }
    pub unsafe fn from_raw_parts(ptr: *mut T, len: usize) -> Self {
        let ptr = std::slice::from_raw_parts_mut(ptr, len);
        Self {
            ptr: NonNull::new(ptr)
                .expect("failed to convert raw pointer to RawPtr<[T]>: null pointer received"),
            size: len * std::mem::size_of::<T>(),
        }
    }

    /// `RawPtr<[T]>` from `*mut [T]` with length.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use uka_util::ptr::RawPtr;
    /// #
    /// let mut value = [1, 2, 3];
    /// let ptr = RawPtr::from_raw_slice_parts(value.as_mut() as *mut [i32], value.len());
    ///unsafe {
    ///   assert_eq!(ptr.as_slice(), &value[..]);
    /// }
    /// ```
    pub fn from_raw_slice_parts(ptr: *mut [T], len: usize) -> Self {
        Self {
            ptr: NonNull::new(ptr)
                .expect("failed to convert *mut [T] to RawPtr<T>: null pointer received"),
            size: len * std::mem::size_of::<T>(),
        }
    }

    /// `RawPtr<[T]>` from `*const T` with length.
    ///
    /// # Safety
    ///
    /// This function is safe to use if the following conditions are met:
    ///
    /// 1. `ptr` must be a valid pointer for the type `T`, meaning it must not be null and it must be properly aligned.
    /// 2. `ptr` must point to a memory region of at least `len * size_of::<T>()` bytes and this region must be initialized properly.
    /// 3. The memory `ptr` points to must be unique for the duration of the `RawPtr<T>`, i.e., no other code may hold a reference to the same memory region during the lifetime of the created `RawPtr<T>`.
    /// 4. The memory region `ptr` points to must not cross over any allocated object boundaries.
    /// 5. `len` must accurately represent the number of contiguous elements of type `T` that `ptr` points to.
    /// 6. The total size `len * size_of::<T>()` must not be larger than `isize::MAX`. In other words, adding that size to `ptr` must not result in a wrap around the address space.
    ///
    /// When these conditions are upheld, using `RawPtr::from_raw_parts` will not cause undefined behavior.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use uka_util::ptr::RawPtr;
    /// #
    /// let value = [1, 2, 3];
    /// unsafe {
    ///    let ptr = RawPtr::from_raw_parts_const(value.as_ptr(), value.len());
    ///    assert_eq!(ptr.as_slice(), &value[..]);
    /// }
    pub unsafe fn from_raw_parts_const(ptr: *const T, len: usize) -> Self {
        Self::from_raw_parts(ptr as *mut T, len)
    }

    /// `RawPtr<[T]>` from `*const [T]` with length.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use uka_util::ptr::RawPtr;
    /// #
    /// let value = [1, 2, 3].as_slice();
    /// let ptr = RawPtr::from_raw_slice_parts_const(value as *const [i32], value.len());
    /// unsafe {
    ///     assert_eq!(ptr.as_slice(), &value[..]);
    /// }
    /// ```
    pub fn from_raw_slice_parts_const(ptr: *const [T], len: usize) -> Self {
        Self::from_raw_slice_parts(ptr as *mut [T], len)
    }

    /// `RawPtr<[T]>` from `isize` with length.
    ///
    /// # Safety
    ///
    /// This function is safe to use if the following conditions are met:
    ///
    /// 1. `ptr` must be a valid pointer for the type `T`, meaning it must not be null and it must be properly aligned.
    /// 2. `ptr` must point to a memory region of at least `len * size_of::<T>()` bytes and this region must be initialized properly.
    /// 3. The memory `ptr` points to must be unique for the duration of the `RawPtr<T>`, i.e., no other code may hold a reference to the same memory region during the lifetime of the created `RawPtr<T>`.
    /// 4. The memory region `ptr` points to must not cross over any allocated object boundaries.
    /// 5. `len` must accurately represent the number of contiguous elements of type `T` that `ptr` points to.
    /// 6. The total size `len * size_of::<T>()` must not be larger than `isize::MAX`. In other words, adding that size to `ptr` must not result in a wrap around the address space.
    ///
    /// When these conditions are upheld, using `RawPtr::from_raw_parts` will not cause undefined behavior.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use uka_util::ptr::RawPtr;
    /// #
    /// let mut value = [1, 2, 3];
    /// unsafe {
    ///     let ptr = RawPtr::<[i32]>::from_raw_address_parts(value.as_ptr() as isize, value.len());
    ///     assert_eq!(ptr.as_slice(), &value[..]);
    /// }
    pub unsafe fn from_raw_address_parts(addr: isize, len: usize) -> Self {
        Self::from_raw_parts(addr as *mut T, len)
    }

    /// `RawPtr<[T]>` from `HGLOBAL` with length.
    ///
    /// # Safety
    ///
    /// This function is safe to use if the following conditions are met:
    ///
    /// 1. `ptr` must be a valid pointer for the type `T`, meaning it must not be null and it must be properly aligned.
    /// 2. `ptr` must point to a memory region of at least `len * size_of::<T>()` bytes and this region must be initialized properly.
    /// 3. The memory `ptr` points to must be unique for the duration of the `RawPtr<T>`, i.e., no other code may hold a reference to the same memory region during the lifetime of the created `RawPtr<T>`.
    /// 4. The memory region `ptr` points to must not cross over any allocated object boundaries.
    /// 5. `len` must accurately represent the number of contiguous elements of type `T` that `ptr` points to.
    /// 6. The total size `len * size_of::<T>()` must not be larger than `isize::MAX`. In other words, adding that size to `ptr` must not result in a wrap around the address space.
    ///
    /// When these conditions are upheld, using `RawPtr::from_raw_parts` will not cause undefined behavior.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::ffi::c_void;
    /// # use windows_sys::Win32::Foundation::HGLOBAL;
    /// # use uka_util::ptr::RawPtr;
    /// #
    /// let mut value = [1, 2, 3];
    /// let hglobal = value.as_ptr() as *mut c_void;
    /// unsafe {
    ///    let ptr = RawPtr::<[i32]>::from_hglobal_parts(hglobal, value.len());
    ///   assert_eq!(ptr.as_slice(), &value[..]);
    /// }
    #[cfg(windows)]
    pub unsafe fn from_hglobal_parts(hglobal: HGLOBAL, len: usize) -> Self {
        Self::from_raw_parts(hglobal as *mut T, len)
    }
    /// Returns the raw pointer of `T`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use uka_util::ptr::RawPtr;
    /// #
    /// let value = [1, 2, 3].as_slice();
    /// let ptr = RawPtr::from_raw_slice_parts_const(value as *const [i32], value.len());
    /// assert_eq!(ptr.as_ptr(), value.as_ptr());
    /// ```
    pub fn as_ptr(&self) -> *const T {
        self.ptr.as_ptr() as *const T
    }

    /// Returns mutable pointer of `T`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use uka_util::ptr::RawPtr;
    /// #
    /// let value = [1, 2, 3].as_slice();
    /// let ptr = RawPtr::from_raw_slice_parts_const(value as *const [i32], value.len());
    /// assert_eq!(ptr.as_mut_ptr(), value.as_ptr() as *mut i32);
    /// ```
    pub fn as_mut_ptr(&self) -> *mut T {
        self.ptr.as_ptr() as *mut T
    }

    /// Returns reference to `T` as slice.
    ///
    /// # Safety
    ///
    /// This function is unsafe due to the following conditions:
    ///
    /// 1. The `RawPtr<T>` instance must have been initialized with a pointer that truly points to a slice of `T` elements. If the `RawPtr<T>` was initialized with a pointer to a single `T` object, uninitialized memory, or invalid memory, using this function can lead to undefined behavior.
    /// 2. The `size` field of `RawPtr<T>` must accurately represent the length of the slice. If the `size` field is not correctly set during the initialization of the `RawPtr<T>`, using this function can lead to undefined behavior.
    /// 3. The entire memory range of this slice must be contained within a single allocated object. Slices can never span across multiple allocated objects.
    /// 4. The total size `len * std::mem::size_of::<T>()` of the slice must be no larger than `isize::MAX`. It is also important to ensure that adding that size to the data pointer must not wrap around the address space.
    /// 5. The memory referenced by the returned slice must not be accessed through any other pointer (not derived from the return value) for the duration of lifetime 'a. Both read and write accesses are forbidden.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use uka_util::ptr::RawPtr;
    /// #
    /// let value = [1, 2, 3].as_slice();
    /// let ptr = RawPtr::from_raw_slice_parts_const(value as *const [i32], value.len());
    /// unsafe {
    ///     assert_eq!(ptr.as_slice(), &[1, 2, 3]);
    /// }
    /// ```
    pub unsafe fn as_slice(&self) -> &[T] {
        self.ptr.as_ref()
    }

    /// Returns mutable reference to `T` as slice.
    ///
    /// # Safety
    ///
    /// This function is unsafe due to the following conditions:
    ///
    /// 1. The `RawPtr<T>` instance must have been initialized with a pointer that truly points to a slice of `T` elements. If the `RawPtr<T>` was initialized with a pointer to a single `T` object, uninitialized memory, or invalid memory, using this function can lead to undefined behavior.
    /// 2. The `size` field of `RawPtr<T>` must accurately represent the length of the slice. If the `size` field is not correctly set during the initialization of the `RawPtr<T>`, using this function can lead to undefined behavior.
    /// 3. The entire memory range of this slice must be contained within a single allocated object. Slices can never span across multiple allocated objects.
    /// 4. The total size `len * std::mem::size_of::<T>()` of the slice must be no larger than `isize::MAX`. It is also important to ensure that adding that size to the data pointer must not wrap around the address space.
    /// 5. The memory referenced by the returned slice must not be accessed through any other pointer (not derived from the return value) for the duration of lifetime 'a. Both read and write accesses are forbidden.
    /// 6. The memory referenced by the returned slice must be mutable.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use uka_util::ptr::RawPtr;
    /// #
    /// let value = [1, 2, 3];
    /// let boxed = Box::new(value);
    /// let ptr = Box::into_raw(boxed) as *mut [i32];
    /// let mut raw = RawPtr::from_raw_slice_parts_const(ptr, value.len());
    /// unsafe {
    ///     raw.as_slice_mut()[0] = 4;
    ///     assert_eq!(raw.as_slice(), &[4, 2, 3]);
    /// }
    /// ```
    pub unsafe fn as_slice_mut(&mut self) -> &mut [T] {
        self.ptr.as_mut()
    }

    /// Returns raw pointer to `T` as slice.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use uka_util::ptr::RawPtr;
    /// #
    /// let value = [1, 2, 3].as_slice();
    /// let ptr = RawPtr::from_raw_slice_parts_const(value as *const [i32], value.len());
    /// assert_eq!(ptr.as_slice_ptr(), value as *const [i32]);
    /// ```
    pub fn as_slice_ptr(&self) -> *const [T] {
        self.ptr.as_ptr() as *const [T]
    }

    /// Returns mutable raw pointer to `T` as slice.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use uka_util::ptr::RawPtr;
    /// #
    /// let value = [1, 2, 3];
    /// let boxed = Box::new(value);
    /// let ptr = Box::into_raw(boxed) as *mut [i32];
    /// let mut raw = RawPtr::from_raw_slice_parts_const(ptr, value.len());
    /// unsafe {
    ///     let slice_ptr = raw.as_slice_mut_ptr();
    ///     (*slice_ptr)[0] = 4;
    ///     assert_eq!(raw.as_slice(), &[4, 2, 3]);
    /// }
    /// ```
    pub fn as_slice_mut_ptr(&mut self) -> *mut [T] {
        self.ptr.as_ptr()
    }

    /// Convert from raw pointers to pointer types that emulate ownership.
    /// Use the same `Allocator` that allocated the memory area the pointer points to.
    /// It is unsafe to specify a different `Allocator`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use uka_util::ptr::RawPtr;
    /// # use uka_util::ptr::OwnedPtr;
    /// # use uka_util::alloc::Allocator;
    /// # use std::alloc::System;
    /// #
    /// let value = [1, 2, 3];
    /// let boxed = Box::new(value);
    /// let ptr = Box::<[i32]>::into_raw(boxed);
    /// let raw = RawPtr::from_raw_slice_parts_const(ptr, value.len());
    /// let owned = raw.to_owned::<System>();
    /// unsafe {
    ///     assert_eq!(owned.as_slice(), value.as_slice());
    /// }
    /// ```
    pub fn to_owned<A: Allocator + Default>(self) -> OwnedPtr<T, A> {
        self.into()
    }
}

impl<T> From<*mut T> for RawPtr<T> {
    fn from(value: *mut T) -> Self {
        Self {
            ptr: NonNull::new(value)
                .expect("failed to convert *mut T to RawPtr<T>: null pointer received"),
            size: std::mem::size_of::<T>(),
        }
    }
}

impl<T> From<*const T> for RawPtr<T> {
    fn from(value: *const T) -> Self {
        Self::from(value as *mut T)
    }
}

impl<T> From<isize> for RawPtr<T> {
    fn from(value: isize) -> Self {
        Self::from(value as *mut T)
    }
}

impl<T> From<RawPtr<T>> for *mut T {
    fn from(value: RawPtr<T>) -> Self {
        value.as_mut_ptr()
    }
}

impl<T> From<RawPtr<[T]>> for *mut T {
    fn from(value: RawPtr<[T]>) -> Self {
        value.ptr.as_ptr().cast()
    }
}

impl<T> From<RawPtr<[T]>> for *mut [T] {
    fn from(mut value: RawPtr<[T]>) -> Self {
        value.as_slice_mut_ptr()
    }
}

impl<T> From<RawPtr<[T]>> for *mut &[T] {
    fn from(mut value: RawPtr<[T]>) -> Self {
        value.as_slice_mut_ptr().cast()
    }
}

impl<T> From<RawPtr<T>> for *const T {
    fn from(value: RawPtr<T>) -> Self {
        value.as_ptr()
    }
}

impl<T> From<RawPtr<[T]>> for *const T {
    fn from(value: RawPtr<[T]>) -> Self {
        value.as_ptr()
    }
}

impl<T> From<RawPtr<[T]>> for *const [T] {
    fn from(value: RawPtr<[T]>) -> Self {
        value.as_slice_ptr()
    }
}

impl<T> From<RawPtr<[T]>> for *const &[T] {
    fn from(value: RawPtr<[T]>) -> Self {
        value.as_slice_ptr().cast()
    }
}

impl<T> From<RawPtr<T>> for isize {
    fn from(value: RawPtr<T>) -> Self {
        value.as_ptr() as isize
    }
}

impl<T> From<RawPtr<[T]>> for isize {
    fn from(value: RawPtr<[T]>) -> Self {
        value.as_ptr() as isize
    }
}

impl<T> From<RawPtr<T>> for NonNull<T> {
    fn from(value: RawPtr<T>) -> Self {
        value.ptr
    }
}

impl<T> From<RawPtr<[T]>> for NonNull<T> {
    fn from(value: RawPtr<[T]>) -> Self {
        value.ptr.cast()
    }
}

impl<T> From<RawPtr<[T]>> for NonNull<[T]> {
    fn from(value: RawPtr<[T]>) -> Self {
        value.ptr
    }
}

impl<T> From<RawPtr<[T]>> for NonNull<&[T]> {
    fn from(value: RawPtr<[T]>) -> Self {
        value.ptr.cast()
    }
}
