use crate::alloc::{AllocError, Allocator};
use crate::ptr::RawPtr;
use std::alloc::{Layout, System};
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

/// `OwnedPtr` gives ownership to the raw pointer.
/// Rust typically manages memory automatically, so when handling raw pointers in this way, care must be taken to avoid undefined behavior.
///
/// # Safety
///
/// The safe use of this struct requires that:
///
/// 1. The pointer must not be NULL.
/// 2. The memory block that the pointer refers to must not be modified by anything other than this `OwnedPtr<T, A>` while it owns the pointer.
/// 3. The memory block that the pointer refers to must not be deallocated by anything other than this `OwnedPtr<T, A>` while it owns the pointer.
/// 4. The same `Allocator` used to allocate the memory must be used to deallocate the memory.
pub struct OwnedPtr<T, A = System>
where
    T: Sized,
    A: Allocator,
{
    /// A non-null raw pointer (`NonNull<T>`) that points to a memory block.
    /// This memory block could be a single `T` object or an array of `T` (slice `&[T]` or `&mut [T]`).
    /// The total size in bytes of the pointed memory block is represented by the `size` field.
    pub(in crate::ptr) ptr: NonNull<T>,

    /// The total size in bytes of the memory block that the pointer `ptr` refers to.
    /// It represents the product of the size of the pointed type `T` and the number of `T` elements,
    /// in case `T` is an array or slice type. For a single `T` object, `size` should be equivalent to `std::mem::size_of::<T>()`.
    pub(in crate::ptr) size: usize,

    alloc: A,
}

impl<T, A> OwnedPtr<T, A>
where
    T: Sized,
    A: Allocator,
{
    /// Returns reference to T
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use uka_util::ptr::OwnedPtr;
    /// #
    /// let ptr = OwnedPtr::new(1);
    /// assert_eq!(ptr.as_ref(), &1);
    /// ```
    pub fn as_ref(&self) -> &T {
        unsafe { self.ptr.as_ref() }
    }

    /// Returns mutable reference to T
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use uka_util::ptr::OwnedPtr;
    /// #
    /// let mut ptr = OwnedPtr::new(1);
    /// *ptr.as_mut() = 2;
    /// assert_eq!(ptr.as_ref(), &2);
    /// ```
    pub fn as_mut(&mut self) -> &mut T {
        unsafe { self.ptr.as_mut() }
    }

    /// Returns reference to `T` as slice.
    ///
    /// # Safety
    ///
    /// This function is marked `unsafe` because it involves working with raw pointers.
    ///
    /// This function is safe under the following conditions:
    ///
    /// 1. The `OwnedPtr<T, A>` instance must have been initialized with a pointer that truly points to a slice of `T` elements. If the `OwnedPtr<T, A>` was initialized with a pointer to a single `T` object or invalid memory, using this function can lead to undefined behavior.
    ///
    /// 2. The `size` field of `OwnedPtr<T, A>` must accurately represent the length of the slice. If the `size` field is not correctly set during the initialization of the `OwnedPtr<T, A>`, using this function can lead to undefined behavior.
    ///
    /// 3. The entire memory range of this slice must be contained within a single allocated object! Slices can never span across multiple allocated objects.
    ///
    /// 4. The memory must be initialized properly. This function does not initialize memory and assumes that it has been done beforehand.
    ///
    /// Ensuring these conditions prevents potential undefined behavior due to the dereferencing of a raw pointer and incorrect slice length assumptions.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use uka_util::ptr::OwnedPtr;
    /// #
    /// let mut ptr = OwnedPtr::from_slice(&[1, 2, 3]);
    /// unsafe {
    ///     assert_eq!(ptr.as_slice(), &[1, 2, 3]);
    /// }
    /// ```
    pub unsafe fn as_slice(&self) -> &[T] {
        assert_eq!(
            self.size % std::mem::size_of::<T>(),
            0,
            "Invalid size field"
        );

        let ptr = self.ptr.as_ptr() as *const T;
        let len = self.size / std::mem::size_of::<T>();
        std::slice::from_raw_parts(ptr, len)
    }

    /// Returns mutable reference to `T` as slice.
    ///
    /// # Safety
    ///
    /// This function is marked `unsafe` because it involves working with raw pointers.
    ///
    /// This function is safe under the following conditions:
    ///
    /// 1. The `OwnedPtr<T, A>` instance must have been initialized with a pointer that truly points to a slice of `T` elements. If the `OwnedPtr<T, A>` was initialized with a pointer to a single `T` object or invalid memory, using this function can lead to undefined behavior.
    ///
    /// 2. The `size` field of `OwnedPtr<T, A>` must accurately represent the length of the slice. If the `size` field is not correctly set during the initialization of the `OwnedPtr<T, A>`, using this function can lead to undefined behavior.
    ///
    /// 3. The entire memory range of this slice must be contained within a single allocated object! Slices can never span across multiple allocated objects.
    ///
    /// 4. The memory must be initialized properly. This function does not initialize memory and assumes that it has been done beforehand.
    ///
    /// Ensuring these conditions prevents potential undefined behavior due to the dereferencing of a raw pointer and incorrect slice length assumptions.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use uka_util::ptr::OwnedPtr;
    /// #
    /// let mut ptr = OwnedPtr::from_vec(vec![1, 2, 3]);
    /// unsafe {
    ///     ptr.as_slice_mut()[0] = 4;
    ///     assert_eq!(ptr.as_slice(), &[4, 2, 3]);
    /// };
    /// ```
    pub unsafe fn as_slice_mut(&mut self) -> &mut [T] {
        assert_eq!(
            self.size % std::mem::size_of::<T>(),
            0,
            "Invalid size field"
        );

        let ptr = self.ptr.as_ptr();
        let len = self.size / std::mem::size_of::<T>();
        std::slice::from_raw_parts_mut(ptr, len)
    }

    /// Relinquishes ownership from `OwnedPtr<T, A>` and returns a `RawPtr<T>`.
    ///
    /// This function abandons the management of the memory by `OwnedPtr` and returns a `RawPtr` pointing to the same memory. It's then up to the caller to properly manage and eventually deallocate this memory.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::alloc::{Layout, System};
    /// # use std::mem::{align_of, size_of};
    /// # use uka_util::alloc::Allocator;
    /// # use uka_util::ptr::OwnedPtr;
    /// #
    /// let ptr = OwnedPtr::new(1);
    /// let raw = ptr.into_raw();
    /// unsafe {
    ///     assert_eq!(raw.as_ref(), &1);
    /// }
    ///
    /// let alloc = System::default();
    /// let layout = Layout::from_size_align(size_of::<i32>(), align_of::<i32>()).unwrap();
    /// alloc.deallocate(raw.into(), layout);
    /// ```
    pub fn into_raw(self) -> RawPtr<T> {
        let ptr = self.ptr;
        let size = self.size;
        std::mem::forget(self);

        RawPtr { ptr, size }
    }

    /// Relinquishes ownership from `OwnedPtr<T, A>` and returns a `RawPtr<[T]>`.
    ///
    /// This function abandons the management of the memory by `OwnedPtr` and returns a `RawPtr` pointing to the same memory. It's then up to the caller to properly manage and eventually deallocate this memory.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::alloc::{Layout, System};
    /// # use std::mem::{align_of, size_of};
    /// # use uka_util::alloc::Allocator;
    /// # use uka_util::ptr::OwnedPtr;
    /// #
    /// let ptr = OwnedPtr::from_slice(&[1, 2, 3]);
    /// let raw = ptr.into_raw_slice();
    /// unsafe {
    ///     assert_eq!(raw.as_ref(), &[1, 2, 3]);
    /// }
    ///
    /// let alloc = System::default();
    /// let layout = Layout::from_size_align(size_of::<i32>() * 3, align_of::<i32>()).unwrap();
    /// alloc.deallocate(raw.into(), layout);
    /// ```
    pub fn into_raw_slice(self) -> RawPtr<[T]> {
        let ptr = self.ptr.as_ptr();
        let size = self.size;
        std::mem::forget(self);

        unsafe { RawPtr::from_raw_parts(ptr, size) }
    }
}

impl<T> OwnedPtr<T, System>
where
    T: Sized,
{
    pub fn new(x: T) -> Self {
        let boxed = Box::new(x);
        let ptr = Box::into_raw(boxed);
        Self {
            ptr: NonNull::new(ptr).expect("can never be NULL as it is a pointer to a valid T"),
            size: std::mem::size_of::<T>(),
            alloc: System::default(),
        }
    }

    /// `OwnedPtr<T, System>` from `Vec<T>`.
    pub fn from_vec(mut x: Vec<T>) -> Self
    where
        T: Clone,
    {
        let ptr = x.as_mut_ptr();
        let size = x.len() * std::mem::size_of::<T>();
        std::mem::forget(x);

        Self {
            ptr: NonNull::new(ptr).expect("can never be NULL as it is a pointer to a valid Vec<T>"),
            size,
            alloc: System::default(),
        }
    }

    /// `OwnedPtr<T, System>` from `&[T]`.
    pub fn from_slice(x: &[T]) -> Self
    where
        T: Clone,
    {
        let x = x.to_vec();
        Self::from_vec(x)
    }
}

impl OwnedPtr<u8, System> {
    /// Reallocates the `OwnedPtr` with a new allocator.
    ///
    /// This method attempts to reallocate the `OwnedPtr` using a new allocator `R`.
    /// The allocator `R` should be an instance of the `Allocator` trait and should
    /// be default constructible.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::alloc::{Layout, System};
    /// # use uka_util::alloc::{Allocator, UkaAllocator};
    /// # use uka_util::ptr::OwnedPtr;
    /// #
    /// let ptr = OwnedPtr::<u8>::from_slice(&[1, 2, 3]);
    /// let ptr = ptr.reallocate::<UkaAllocator>();
    /// let ptr = ptr.unwrap();
    /// unsafe {
    ///     assert_eq!(ptr.as_slice(), &[1, 2, 3]);
    /// }
    /// ```
    pub fn reallocate<R: Allocator + Default>(self) -> Result<OwnedPtr<u8, R>, AllocError> {
        let alloc = R::default();
        let layout = Layout::from_size_align(self.size, std::mem::align_of::<u8>())
            .expect("`Layout` can always be created with align_of created align");
        let ptr = alloc.allocate(layout)?;
        let src = self.ptr.as_ptr() as *const u8;
        let dest = ptr.as_ptr() as *mut u8;
        unsafe {
            std::ptr::copy_nonoverlapping(src, dest, self.size);
        };

        Ok(OwnedPtr {
            ptr: ptr.cast(),
            size: self.size,
            alloc,
        })
    }
}

impl<T, A> Deref for OwnedPtr<T, A>
where
    T: Sized,
    A: Allocator,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T, A> DerefMut for OwnedPtr<T, A>
where
    T: Sized,
    A: Allocator,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl<T, A> Drop for OwnedPtr<T, A>
where
    T: Sized,
    A: Allocator,
{
    fn drop(&mut self) {
        let layout = Layout::from_size_align(self.size, std::mem::align_of::<T>()).unwrap();
        self.alloc.deallocate(self.ptr.cast(), layout);
    }
}

impl<T, A> From<RawPtr<T>> for OwnedPtr<T, A>
where
    A: Allocator + Default,
{
    fn from(value: RawPtr<T>) -> Self {
        Self {
            ptr: value.ptr,
            size: value.size,
            alloc: A::default(),
        }
    }
}

impl<T, A> From<RawPtr<[T]>> for OwnedPtr<T, A>
where
    T: Sized,
    A: Allocator + Default,
{
    fn from(value: RawPtr<[T]>) -> Self {
        Self {
            ptr: value.ptr.cast(),
            size: value.size,
            alloc: A::default(),
        }
    }
}

impl<T, A> From<OwnedPtr<T, A>> for RawPtr<T>
where
    A: Allocator + Default,
{
    fn from(value: OwnedPtr<T, A>) -> Self {
        value.into_raw()
    }
}

impl<T, A> From<OwnedPtr<T, A>> for RawPtr<[T]>
where
    A: Allocator + Default,
{
    fn from(value: OwnedPtr<T, A>) -> Self {
        value.into_raw_slice()
    }
}
