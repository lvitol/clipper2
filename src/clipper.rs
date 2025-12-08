use std::marker::PhantomData;

use clipper2c_sys::{
    clipper_clipper64, clipper_clipper64_add_clip, clipper_clipper64_add_open_subject,
    clipper_clipper64_add_subject, clipper_clipper64_execute,
    clipper_clipper64_execute_tree_with_open, clipper_clipper64_size,
    clipper_delete_clipper64, clipper_delete_path64, clipper_delete_paths64,
    clipper_delete_polytree64, clipper_paths64, clipper_paths64_size,
    clipper_polytree64, clipper_polytree64_area, clipper_polytree64_count,
    clipper_polytree64_get_child, clipper_polytree64_is_hole, clipper_polytree64_parent,
    clipper_polytree64_polygon, clipper_polytree64_size, clipper_polytree64_to_paths,
    ClipperClipper64, ClipperPolyTree64,
};

use crate::{malloc, Centi, ClipType, FillRule, Path, Paths, PointScaler};

/// The result of a boolean operation containing both closed and open paths.
#[derive(Debug, Clone)]
pub struct BooleanResult<P: PointScaler = Centi> {
    /// Closed paths from the boolean operation
    pub closed: Paths<P>,
    /// Open paths from the boolean operation
    pub open: Paths<P>,
}

impl<P: PointScaler> BooleanResult<P> {
    /// Create a new BooleanResult
    pub fn new(closed: Paths<P>, open: Paths<P>) -> Self {
        Self { closed, open }
    }
}

/// The result of a boolean operation containing a PolyTree with hierarchy and open paths.
#[derive(Debug)]
pub struct BooleanTreeResult<P: PointScaler = Centi> {
    /// PolyTree containing the closed paths with hierarchy information
    pub tree: PolyTree<P>,
    /// Open paths from the boolean operation
    pub open: Paths<P>,
}

impl<P: PointScaler> BooleanTreeResult<P> {
    /// Create a new BooleanTreeResult
    pub fn new(tree: PolyTree<P>, open: Paths<P>) -> Self {
        Self { tree, open }
    }
}

/// The state of the Clipper struct.
pub trait ClipperState {}

/// A state indicating no subjects and no clips.
#[derive(Debug)]
pub struct NoSubjects {}
impl ClipperState for NoSubjects {}

/// A state indicating one or more subjects and no clips.
#[derive(Debug)]
pub struct WithSubjects {}
impl ClipperState for WithSubjects {}

/// A state indicating one or more subjects and one or more clips.
#[derive(Debug)]
pub struct WithClips {}
impl ClipperState for WithClips {}

/// The Clipper struct used as a builder for applying boolean operations to paths.
#[derive(Debug)]
pub struct Clipper<S: ClipperState = NoSubjects, P: PointScaler = Centi> {
    ptr: *mut ClipperClipper64,
    keep_ptr_on_drop: bool,
    _marker: PhantomData<P>,
    _state: S,
}

impl<P: PointScaler> Clipper<NoSubjects, P> {
    /// Creates a new empty Clipper instance.
    pub fn new() -> Clipper<NoSubjects, P> {
        let ptr = unsafe {
            let mem = malloc(clipper_clipper64_size());
            clipper_clipper64(mem)
        };

        Clipper::<NoSubjects, P> {
            ptr,
            keep_ptr_on_drop: false,
            _marker: PhantomData,
            _state: NoSubjects {},
        }
    }
}

impl<P: PointScaler> Clipper<NoSubjects, P> {
    /// Adds a subject path to the Clipper instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use clipper2::*;
    ///
    /// let path: Paths = vec![(0.2, 0.2), (6.0, 0.2), (6.0, 6.0), (0.2, 6.0)].into();
    ///
    /// let clipper = Clipper::new().add_subject(path);
    /// ```
    pub fn add_subject(mut self, subject: impl Into<Paths<P>>) -> Clipper<WithSubjects, P> {
        self.keep_ptr_on_drop = true;

        let clipper = Clipper::<WithSubjects, P> {
            ptr: self.ptr,
            keep_ptr_on_drop: false,
            _marker: PhantomData,
            _state: WithSubjects {},
        };

        drop(self);

        clipper.add_subject(subject)
    }

    /// Adds an open subject path to the Clipper instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use clipper2::*;
    ///
    /// let path: Paths = vec![(0.2, 0.2), (6.0, 0.2), (6.0, 6.0), (0.2, 6.0)].into();
    ///
    /// let clipper = Clipper::new().add_open_subject(path);
    /// ```
    pub fn add_open_subject(mut self, subject: impl Into<Paths<P>>) -> Clipper<WithSubjects, P> {
        self.keep_ptr_on_drop = true;

        let clipper = Clipper::<WithSubjects, P> {
            ptr: self.ptr,
            keep_ptr_on_drop: false,
            _marker: PhantomData,
            _state: WithSubjects {},
        };

        drop(self);

        clipper.add_open_subject(subject)
    }
}

impl<P: PointScaler> Clipper<WithSubjects, P> {
    /// Adds another subject path to the Clipper instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use clipper2::*;
    ///
    /// let path: Paths = vec![(0.2, 0.2), (6.0, 0.2), (6.0, 6.0), (0.2, 6.0)].into();
    /// let path2: Paths = vec![(1.2, 1.2), (4.0, 1.2), (1.2, 4.0)].into();
    ///
    /// let clipper = Clipper::new().add_subject(path).add_subject(path2);
    /// ```
    pub fn add_subject(self, subject: impl Into<Paths<P>>) -> Self {
        unsafe {
            let subject_ptr = subject.into().to_clipperpaths64();
            clipper_clipper64_add_subject(self.ptr, subject_ptr);
            clipper_delete_paths64(subject_ptr);
        }

        self
    }

    /// Adds another open subject path to the Clipper instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use clipper2::*;
    ///
    /// let path: Paths = vec![(0.2, 0.2), (6.0, 0.2), (6.0, 6.0), (0.2, 6.0)].into();
    /// let path2: Paths = vec![(1.2, 1.2), (4.0, 1.2), (1.2, 4.0)].into();
    ///
    /// let clipper = Clipper::new().add_subject(path).add_open_subject(path2);
    /// ```
    pub fn add_open_subject(self, subject: impl Into<Paths<P>>) -> Self {
        unsafe {
            let subject_ptr = subject.into().to_clipperpaths64();
            clipper_clipper64_add_open_subject(self.ptr, subject_ptr);
            clipper_delete_paths64(subject_ptr);
        }

        self
    }

    /// Adds a clip path to the Clipper instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use clipper2::*;
    ///
    /// let path: Paths = vec![(0.2, 0.2), (6.0, 0.2), (6.0, 6.0), (0.2, 6.0)].into();
    /// let path2: Paths = vec![(1.2, 1.2), (4.0, 1.2), (1.2, 4.0)].into();
    ///
    /// let clipper = Clipper::new().add_subject(path).add_clip(path2);
    /// ```
    pub fn add_clip(mut self, clip: impl Into<Paths<P>>) -> Clipper<WithClips, P> {
        self.keep_ptr_on_drop = true;

        let clipper = Clipper::<WithClips, P> {
            ptr: self.ptr,
            keep_ptr_on_drop: false,
            _marker: PhantomData,
            _state: WithClips {},
        };

        drop(self);

        clipper.add_clip(clip)
    }
}

impl<P: PointScaler> Clipper<WithClips, P> {
    /// Adds another clip path to the Clipper instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use clipper2::*;
    ///
    /// let path: Paths = vec![(0.2, 0.2), (6.0, 0.2), (6.0, 6.0), (0.2, 6.0)].into();
    /// let path2: Paths = vec![(1.2, 1.2), (4.0, 1.2), (1.2, 4.0)].into();
    /// let path3: Paths = vec![(2.2, 2.2), (5.0, 2.2), (2.2, 5.0)].into();
    ///
    /// let clipper = Clipper::new().add_subject(path).add_clip(path2).add_clip(path3);
    /// ```
    pub fn add_clip(self, clip: impl Into<Paths<P>>) -> Self {
        unsafe {
            let clip_ptr = clip.into().to_clipperpaths64();
            clipper_clipper64_add_clip(self.ptr, clip_ptr);
            clipper_delete_paths64(clip_ptr);
        }

        self
    }

    /// Applies a union boolean operation to the Clipper instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use clipper2::*;
    ///
    /// let path: Paths = vec![(0.2, 0.2), (6.0, 0.2), (6.0, 6.0), (0.2, 6.0)].into();
    /// let path2: Paths = vec![(1.2, 1.2), (4.0, 1.2), (1.2, 4.0)].into();
    ///
    /// let result = Clipper::new().add_subject(path).add_clip(path2).union(FillRule::NonZero).unwrap();
    /// let closed = result.closed;
    /// let open = result.open;
    /// ```
    ///
    /// For more details see the original [union](https://www.angusj.com/clipper2/Docs/Units/Clipper/Functions/Union.htm) docs.
    pub fn union(self, fill_rule: FillRule) -> Result<BooleanResult<P>, ClipperError> {
        self.boolean_operation(ClipType::Union, fill_rule)
    }

    /// Applies a difference boolean operation to the Clipper instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use clipper2::*;
    ///
    /// let path: Paths = vec![(0.2, 0.2), (6.0, 0.2), (6.0, 6.0), (0.2, 6.0)].into();
    /// let path2: Paths = vec![(1.2, 1.2), (4.0, 1.2), (1.2, 4.0)].into();
    ///
    /// let result = Clipper::new().add_subject(path).add_clip(path2).difference(FillRule::NonZero).unwrap();
    /// let closed = result.closed;
    /// let open = result.open;
    /// ```
    ///
    /// For more details see the original [difference](https://www.angusj.com/clipper2/Docs/Units/Clipper/Functions/Difference.htm) docs.
    pub fn difference(self, fill_rule: FillRule) -> Result<BooleanResult<P>, ClipperError> {
        self.boolean_operation(ClipType::Difference, fill_rule)
    }

    /// Applies an intersection boolean operation to the Clipper instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use clipper2::*;
    ///
    /// let path: Paths = vec![(0.2, 0.2), (6.0, 0.2), (6.0, 6.0), (0.2, 6.0)].into();
    /// let path2: Paths = vec![(1.2, 1.2), (4.0, 1.2), (1.2, 4.0)].into();
    ///
    /// let result = Clipper::new().add_subject(path).add_clip(path2).intersect(FillRule::NonZero).unwrap();
    /// let closed = result.closed;
    /// let open = result.open;
    /// ```
    ///
    /// For more details see the original [intersect](https://www.angusj.com/clipper2/Docs/Units/Clipper/Functions/Intersect.htm) docs.
    pub fn intersect(self, fill_rule: FillRule) -> Result<BooleanResult<P>, ClipperError> {
        self.boolean_operation(ClipType::Intersection, fill_rule)
    }

    /// Applies an xor boolean operation to the Clipper instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use clipper2::*;
    ///
    /// let path: Paths = vec![(0.2, 0.2), (6.0, 0.2), (6.0, 6.0), (0.2, 6.0)].into();
    /// let path2: Paths = vec![(1.2, 1.2), (4.0, 1.2), (1.2, 4.0)].into();
    ///
    /// let result = Clipper::new().add_subject(path).add_clip(path2).xor(FillRule::NonZero).unwrap();
    /// let closed = result.closed;
    /// let open = result.open;
    /// ```
    ///
    /// For more details see the original [xor](https://www.angusj.com/clipper2/Docs/Units/Clipper/Functions/XOR.htm) docs.
    pub fn xor(self, fill_rule: FillRule) -> Result<BooleanResult<P>, ClipperError> {
        self.boolean_operation(ClipType::Xor, fill_rule)
    }

    fn boolean_operation(
        self,
        clip_type: ClipType,
        fill_rule: FillRule,
    ) -> Result<BooleanResult<P>, ClipperError> {
        let closed_path = unsafe { Paths::<P>::new(Vec::new()).to_clipperpaths64() };
        let open_path = unsafe { Paths::<P>::new(Vec::new()).to_clipperpaths64() };

        let result = unsafe {
            let success = clipper_clipper64_execute(
                self.ptr,
                clip_type.into(),
                fill_rule.into(),
                closed_path,
                open_path,
            );

            if success != 1 {
                clipper_delete_paths64(closed_path);
                clipper_delete_paths64(open_path);
                return Err(ClipperError::FailedBooleanOperation);
            }

            let closed_result = Paths::from_clipperpaths64(closed_path);
            let open_result = Paths::from_clipperpaths64(open_path);
            clipper_delete_paths64(closed_path);
            clipper_delete_paths64(open_path);

            Ok(BooleanResult::new(closed_result, open_result))
        };

        drop(self);

        result
    }

    /// Applies a boolean operation and returns a PolyTree with hierarchy information.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use clipper2::*;
    ///
    /// let path: Paths = vec![(0.2, 0.2), (6.0, 0.2), (6.0, 6.0), (0.2, 6.0)].into();
    /// let path2: Paths = vec![(1.2, 1.2), (4.0, 1.2), (1.2, 4.0)].into();
    ///
    /// let result = Clipper::new()
    ///     .add_subject(path)
    ///     .add_clip(path2)
    ///     .union_tree(FillRule::NonZero)
    ///     .unwrap();
    /// ```
    pub fn union_tree(self, fill_rule: FillRule) -> Result<BooleanTreeResult<P>, ClipperError> {
        self.boolean_operation_tree(ClipType::Union, fill_rule)
    }

    /// Applies a difference boolean operation and returns a PolyTree with hierarchy information.
    pub fn difference_tree(self, fill_rule: FillRule) -> Result<BooleanTreeResult<P>, ClipperError> {
        self.boolean_operation_tree(ClipType::Difference, fill_rule)
    }

    /// Applies an intersection boolean operation and returns a PolyTree with hierarchy information.
    pub fn intersect_tree(self, fill_rule: FillRule) -> Result<BooleanTreeResult<P>, ClipperError> {
        self.boolean_operation_tree(ClipType::Intersection, fill_rule)
    }

    /// Applies an xor boolean operation and returns a PolyTree with hierarchy information.
    pub fn xor_tree(self, fill_rule: FillRule) -> Result<BooleanTreeResult<P>, ClipperError> {
        self.boolean_operation_tree(ClipType::Xor, fill_rule)
    }

    fn boolean_operation_tree(
        self,
        clip_type: ClipType,
        fill_rule: FillRule,
    ) -> Result<BooleanTreeResult<P>, ClipperError> {
        unsafe {
            // Allocate memory for PolyTree
            let tree_mem = malloc(clipper_polytree64_size());
            let tree_ptr = clipper_polytree64(tree_mem, std::ptr::null_mut());
            
            // Allocate memory for open paths
            let open_path_mem = malloc(clipper_paths64_size());
            let open_path_ptr = clipper_paths64(open_path_mem);

            let success = clipper_clipper64_execute_tree_with_open(
                self.ptr,
                clip_type.into(),
                fill_rule.into(),
                tree_ptr,
                open_path_ptr,
            );

            if success != 1 {
                clipper_delete_polytree64(tree_ptr);
                clipper_delete_paths64(open_path_ptr);
                return Err(ClipperError::FailedBooleanOperation);
            }

            let poly_tree = PolyTree::from_ptr(tree_ptr);
            let open_paths = Paths::from_clipperpaths64(open_path_ptr);
            
            // Note: poly_tree will manage its own memory via Drop trait
            // We don't need to delete open_path_ptr here because Paths::from_clipperpaths64 creates a copy
            // and we need to clean up the original
            clipper_delete_paths64(open_path_ptr);
            
            Ok(BooleanTreeResult::new(poly_tree, open_paths))
        }
    }
}

impl Default for Clipper<NoSubjects, Centi> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: ClipperState, P: PointScaler> Drop for Clipper<S, P> {
    fn drop(&mut self) {
        if !self.keep_ptr_on_drop {
            unsafe { clipper_delete_clipper64(self.ptr) }
        }
    }
}

/// A PolyTree structure representing the result of a boolean operation with hierarchy.
pub struct PolyTree<P: PointScaler = Centi> {
    ptr: *mut ClipperPolyTree64,
    _marker: PhantomData<P>,
}

impl<P: PointScaler> PolyTree<P> {
    /// Create a PolyTree from a raw pointer. This is unsafe because the caller must ensure
    /// the pointer is valid and will be properly managed.
    pub unsafe fn from_ptr(ptr: *mut ClipperPolyTree64) -> Self {
        Self {
            ptr,
            _marker: PhantomData,
        }
    }

    /// Get the number of direct children of this PolyTree node.
    pub fn child_count(&self) -> usize {
        unsafe { clipper_polytree64_count(self.ptr) }
    }

    /// Get a child PolyTree at the given index.
    pub fn get_child(&self, index: usize) -> Option<PolyTree<P>> {
        if index >= self.child_count() {
            return None;
        }
        let child_ptr = unsafe { clipper_polytree64_get_child(self.ptr, index) };
        // The C function returns a const pointer, but we need a mutable pointer for our API.
        // Since the underlying C API likely expects mutable pointers in other functions,
        // we'll cast away const. This is safe as long as we don't mutate while other references exist.
        Some(PolyTree {
            ptr: child_ptr as *mut ClipperPolyTree64,
            _marker: PhantomData,
        })
    }

    /// Check if this PolyTree node represents a hole.
    pub fn is_hole(&self) -> bool {
        unsafe { clipper_polytree64_is_hole(self.ptr) != 0 }
    }

    /// Get the polygon path associated with this PolyTree node.
    pub fn polygon(&self) -> Path<P> {
        unsafe {
            let mem = malloc(clipper_polytree64_size());
            let path_ptr = clipper_polytree64_polygon(mem, self.ptr);
            let path = Path::from_clipperpath64(path_ptr);
            clipper_delete_path64(path_ptr);
            path
        }
    }

    /// Get the area of this PolyTree node's polygon.
    pub fn area(&self) -> f64 {
        unsafe { clipper_polytree64_area(self.ptr) / (P::MULTIPLIER * P::MULTIPLIER) }
    }

    /// Convert this PolyTree (and all its children) to Paths.
    pub fn to_paths(&self) -> Paths<P> {
        unsafe {
            let mem = malloc(clipper_polytree64_size());
            let paths_ptr = clipper_polytree64_to_paths(mem, self.ptr);
            let paths = Paths::from_clipperpaths64(paths_ptr);
            clipper_delete_paths64(paths_ptr);
            paths
        }
    }

    /// Get the parent of this PolyTree node, if any.
    pub fn parent(&self) -> Option<PolyTree<P>> {
        let parent_ptr = unsafe { clipper_polytree64_parent(self.ptr) };
        if parent_ptr.is_null() {
            None
        } else {
            Some(PolyTree {
                ptr: parent_ptr as *mut ClipperPolyTree64,
                _marker: PhantomData,
            })
        }
    }
}

impl<P: PointScaler> Drop for PolyTree<P> {
    fn drop(&mut self) {
        unsafe {
            clipper_delete_polytree64(self.ptr);
        }
    }
}


/// Errors that can occur during clipper operations.
#[derive(Debug, thiserror::Error)]
pub enum ClipperError {
    /// Failed execute boolean operation.
    #[error("Failed boolean operation")]
    FailedBooleanOperation,
}
