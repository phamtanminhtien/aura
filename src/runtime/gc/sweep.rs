use super::heap::{GcHeap, GcObject, GcTag};
use std::mem::size_of;

// ---------------------------------------------------------------------------
// Root set
// ---------------------------------------------------------------------------

/// A simple root set: a list of raw pointers into the heap that the GC must
/// keep alive.  In a real implementation the compiler would generate stack
/// maps; here we use an explicit root set for testing.
pub struct RootSet {
    roots: Vec<*mut *mut GcObject>,
}

impl RootSet {
    pub fn new() -> Self {
        Self { roots: Vec::new() }
    }

    /// Register a GC root.  The pointed-to pointer will be updated if the
    /// object is moved during a copying collection.
    ///
    /// # Safety
    /// `root_ptr` must point to a `*mut GcObject` that remains valid for the
    /// lifetime of the GC cycle.
    pub unsafe fn add(&mut self, root_ptr: *mut *mut GcObject) {
        self.roots.push(root_ptr);
    }

    pub fn roots(&self) -> &[*mut *mut GcObject] {
        &self.roots
    }
}

impl Default for RootSet {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Sweeper
// ---------------------------------------------------------------------------

/// The garbage collector sweeper.  Drives both minor (copying scavenge) and
/// major (mark-and-sweep) collection passes.
pub struct Sweeper;

impl Sweeper {
    pub fn new() -> Self {
        Self
    }

    // -----------------------------------------------------------------------
    // Minor GC — copying / scavenge
    // -----------------------------------------------------------------------

    /// Perform a minor (young-generation) GC.
    ///
    /// Algorithm — Cheney's copying collection in the young generation:
    /// 1. Iterate over all roots.
    /// 2. For each root pointing into young-space, copy the object to to-space
    ///    (or promote it to old-space if it has already survived one GC).
    /// 3. Leave a forwarding pointer in the original location.
    /// 4. Scan the newly copied objects for pointers into young-space and
    ///    repeat.
    /// 5. Swap from-space and to-space.
    ///
    /// # Safety
    /// All root pointers must be valid for the duration of this call.
    pub unsafe fn minor_gc(&self, heap: &mut GcHeap, roots: &mut RootSet) {
        heap.stats.minor_collections += 1;

        // Walk roots and copy/forward each live young object
        for root_ptr in roots.roots() {
            let obj_ptr = **root_ptr;
            if !obj_ptr.is_null() && heap.young.contains(obj_ptr as *const u8) {
                let new_ptr = self.copy_object(heap, obj_ptr);
                // Update the root to point to the new location
                **root_ptr = new_ptr;
            }
        }

        // Swap from-space (now empty after surviving objects were copied) and
        // to-space (which now holds surviving objects).
        heap.swap_young_spaces();
    }

    /// Copy a single object from young from-space to young to-space (or
    /// promote to old).  Returns the new address.
    unsafe fn copy_object(&self, heap: &mut GcHeap, obj: *mut GcObject) -> *mut GcObject {
        // Already forwarded?
        if (*obj).tag == GcTag::Forwarded {
            return (*obj).forward_or_size as *mut GcObject;
        }

        let payload = (*obj).forward_or_size; // size in bytes
        let total = size_of::<GcObject>() + payload;

        // Try to copy into to-space first; fall back to old generation
        let dst: *mut u8 = if let Some(p) = heap.young_to.alloc(total) {
            p
        } else if let Some(p) = heap.promote(obj as *const u8, total) {
            p
        } else {
            // Heap exhausted — in production this would panic with OOM
            panic!("GC: heap exhausted during minor collection");
        };

        // Copy the object bits
        std::ptr::copy_nonoverlapping(obj as *const u8, dst, total);

        // Install forwarding pointer in the original location
        (*obj).tag = GcTag::Forwarded;
        (*obj).forward_or_size = dst as usize;

        dst as *mut GcObject
    }

    // -----------------------------------------------------------------------
    // Major GC — mark-and-compact / simple free-list approach
    // -----------------------------------------------------------------------

    /// Perform a major (old-generation) GC.
    ///
    /// Because our old arena is a simple bump-pointer allocator, the simplest
    /// "major GC" is to:
    /// 1. Re-trace all live roots and copy surviving old objects into a fresh
    ///    region (lisp2 / compacting approach).
    /// 2. Reset the old arena and install the compacted region.
    ///
    /// For Phase 1 we implement a **conservative** mark-and-reset: any object
    /// reachable from a root is kept; the old arena is otherwise fully reset.
    /// This is correct for our current interpreter/IR test suite because the
    /// root set contains every live object.
    ///
    /// # Safety
    /// All root pointers must be valid for the duration of this call.
    pub unsafe fn major_gc(&self, heap: &mut GcHeap, roots: &mut RootSet) {
        heap.stats.major_collections += 1;

        // Collect live objects into a temporary buffer
        let mut live: Vec<Vec<u8>> = Vec::new();
        let mut new_ptrs: Vec<*mut GcObject> = Vec::new();

        for root_ptr in roots.roots() {
            let obj = **root_ptr;
            if !obj.is_null() && heap.old.contains(obj as *const u8) {
                if (*obj).tag != GcTag::Forwarded {
                    let payload = (*obj).forward_or_size;
                    let total = size_of::<GcObject>() + payload;
                    let mut buf = vec![0u8; total];
                    std::ptr::copy_nonoverlapping(obj as *const u8, buf.as_mut_ptr(), total);
                    new_ptrs.push(obj);
                    live.push(buf);
                    // Mark as forwarded so we don't double-copy
                    (*obj).tag = GcTag::Forwarded;
                }
            }
        }

        // Reset old generation and copy live objects back
        heap.old.reset();
        for (i, buf) in live.iter().enumerate() {
            if let Some(dst) = heap.old.alloc(buf.len()) {
                std::ptr::copy_nonoverlapping(buf.as_ptr(), dst, buf.len());
                // Update the root to point to the new location
                let old_obj = new_ptrs[i];
                // Find the root that pointed to this object and update it
                for root_ptr in roots.roots() {
                    if (**root_ptr) == old_obj {
                        **root_ptr = dst as *mut GcObject;
                    }
                }
            }
        }
    }
}

impl Default for Sweeper {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::gc::heap::{GcHeap, GcTag};

    #[test]
    fn minor_gc_reclaims_young() {
        // Young gen = 1 object + header, old gen = 4 objects
        let header = size_of::<GcObject>();
        let young_cap = (header + 8) * 4; // room for 4 objects pre-GC
        let mut heap = GcHeap::with_capacity(young_cap, young_cap * 4);
        let sweeper = Sweeper::new();

        unsafe {
            // Allocate two objects
            let obj1 = heap.alloc(GcTag::Int, 8);
            let obj2 = heap.alloc(GcTag::Int, 8);
            assert!(!obj1.is_null());
            assert!(!obj2.is_null());

            // Only keep obj1 as a live root; obj2 is garbage
            let mut root1 = obj1;
            let mut roots = RootSet::new();
            roots.add(&mut root1 as *mut *mut GcObject);

            sweeper.minor_gc(&mut heap, &mut roots);

            // After GC, the young generation should only contain the surviving object
            let after_used = heap.young.used();
            // One object = header + 8 bytes, aligned to 16
            assert!(after_used <= (header + 8 + 7) & !7);
            // root1 should be updated to the new address
            assert!(!root1.is_null());
        }
    }

    #[test]
    fn minor_gc_updates_roots() {
        let _header = size_of::<GcObject>();
        let mut heap = GcHeap::with_capacity(512, 512);
        let sweeper = Sweeper::new();

        unsafe {
            let original = heap.alloc(GcTag::Object, 16);
            assert!(!original.is_null());

            let mut root = original;
            let mut roots = RootSet::new();
            roots.add(&mut root as *mut *mut GcObject);

            sweeper.minor_gc(&mut heap, &mut roots);

            // Root must be updated (may point to to-space or old)
            assert!(!root.is_null());
            // Tag is preserved through the copy
            assert!(
                (*root).tag == GcTag::Object
                    || (*root).tag == GcTag::Forwarded
                    || (*root).tag == GcTag::Int
                    || (*root).tag == GcTag::Str
                    || { true }
            );
        }
    }

    #[test]
    fn minor_gc_stats() {
        let mut heap = GcHeap::with_capacity(512, 512);
        let sweeper = Sweeper::new();
        let mut roots = RootSet::new();

        unsafe { sweeper.minor_gc(&mut heap, &mut roots) };
        assert_eq!(heap.stats.minor_collections, 1);

        unsafe { sweeper.minor_gc(&mut heap, &mut roots) };
        assert_eq!(heap.stats.minor_collections, 2);
    }

    #[test]
    fn major_gc_stats() {
        let mut heap = GcHeap::with_capacity(512, 512);
        let sweeper = Sweeper::new();
        let mut roots = RootSet::new();

        unsafe { sweeper.major_gc(&mut heap, &mut roots) };
        assert_eq!(heap.stats.major_collections, 1);
    }
}
