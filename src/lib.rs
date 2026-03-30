#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(unsafe_op_in_unsafe_fn)]

use autocxx::prelude::*;
use cgmath;
use cgmath::One;
use std::cell::RefCell;
use std::ffi::CString;
use thiserror::Error;

pub type Point = cgmath::Point2<f64>;
pub type Vector = cgmath::Vector2<f64>;
pub type Vector3 = cgmath::Vector3<f64>;
pub type Matrix3 = cgmath::Matrix3<f64>;

include_cpp! {
    #include "wrapper.h"
    safety!(unsafe)
    // transfer objects
    generate_pod!("rust_helper::Point2D")
    generate_pod!("rust_helper::BoundingBox")
    generate!("rust_helper::PolygonArrayTransfer")
    generate!("rust_helper::TopLevelResult")
    generate_pod!("rust_helper::LayerInterval")

    // gdstk objects
    generate!("gdstk::Polygon")
    generate!("gdstk::Cell")
    generate!("gdstk::Library")
    generate!("gdstk::Label")
    generate!("gdstk::LayerName")
    generate!("gdstk::ErrorCode")
    generate!("gdstk::Tag")
    generate!("gdstk::make_tag")

    // library
    generate!("rust_helper::LibraryOwner")
    generate!("rust_helper::library_read_gds")
    generate!("rust_helper::library_read_oas")
    generate!("rust_helper::library_get_top_level")
    generate!("rust_helper::library_get_cell")
    generate!("rust_helper::library_get_rawcell")
    generate!("rust_helper::library_get_unit")
    generate!("rust_helper::library_get_precision")
    generate!("rust_helper::library_count_layernames")
    generate!("rust_helper::library_get_layername")

    // Label
    generate!("rust_helper::label_get_text")
    generate!("rust_helper::label_get_position")
    generate!("rust_helper::label_get_bounding_box")

    // LayerName
    generate!("rust_helper::layername_get_name")
    generate!("rust_helper::layername_get_layer")
    generate!("rust_helper::layername_get_datatype")
    generate!("rust_helper::layername_get_layer_interval")
    generate!("rust_helper::layername_get_datatype_interval")

    // cell
    generate!("rust_helper::cell_get_name")
    generate!("rust_helper::cell_get_polygons")
    generate!("rust_helper::cell_get_bounding_box")
    generate!("rust_helper::cell_count_polygon_refs")
    generate!("rust_helper::cell_get_polygon_ref")
    generate!("rust_helper::cell_count_references")
    generate!("rust_helper::cell_get_reference")
    generate!("rust_helper::cell_count_flexpaths")
    generate!("rust_helper::cell_get_flexpath")
    generate!("rust_helper::cell_count_robustpaths")
    generate!("rust_helper::cell_get_robustpath")
    generate!("rust_helper::cell_count_labels")
    generate!("rust_helper::cell_get_label")

    // polygon
    generate!("rust_helper::PolygonOwner")
    generate!("rust_helper::polygon_new")
    generate!("rust_helper::polygon_new_from_ref")
    generate!("rust_helper::polygon_copy")
    generate!("rust_helper::polygon_translate")
    generate!("rust_helper::polygon_scale")
    generate!("rust_helper::polygon_mirror")
    generate!("rust_helper::polygon_rotate")
    generate!("rust_helper::polygon_layer")
    generate!("rust_helper::polygon_datatype")
    generate!("rust_helper::polygon_foreach_point")
    generate!("rust_helper::PointCallback")
    generate!("rust_helper::polygon_get_bounding_box")
    generate!("rust_helper::polygon_to_ref")

    // polygon_ref
    generate!("rust_helper::polygon_ref_get_bounding_box")
    generate!("rust_helper::polygon_ref_get_repetition")
    generate!("rust_helper::polygon_ref_layer")
    generate!("rust_helper::polygon_ref_datatype")
    generate!("rust_helper::polygon_ref_foreach_point")

    // flexpath
    generate!("rust_helper::flexpath_to_polygons")

    // robustpath
    generate!("rust_helper::robustpath_to_polygons")

    // reference
    generate!("rust_helper::reference_get_translate")
    generate!("rust_helper::reference_get_scale")
    generate!("rust_helper::reference_get_rotation")
    generate!("rust_helper::reference_get_x_reflection")
    generate!("rust_helper::reference_get_repetition")
    generate!("rust_helper::reference_get_cell")

    // Repetition
    generate!("rust_helper::repetition_foreach_offset")

    // rawcell
    generate!("rust_helper::rawcell_get_name")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Error)]
pub enum ErrorCode {
    #[error("no error")]
    NoError,
    #[error("boolean error")]
    BooleanError,
    #[error("empty path")]
    EmptyPath,
    #[error("intersection not found")]
    IntersectionNotFound,
    #[error("missing reference")]
    MissingReference,
    #[error("unsupported record")]
    UnsupportedRecord,
    #[error("unofficial specification")]
    UnofficialSpecification,
    #[error("invalid repetition")]
    InvalidRepetition,
    #[error("overflow")]
    Overflow,
    #[error("checksum error")]
    ChecksumError,
    #[error("output file open error")]
    OutputFileOpenError,
    #[error("input file open error")]
    InputFileOpenError,
    #[error("input file error")]
    InputFileError,
    #[error("file error")]
    FileError,
    #[error("invalid file")]
    InvalidFile,
    #[error("insufficient memory")]
    InsufficientMemory,
    #[error("zlib error")]
    ZlibError,
    #[error("unknown: {0}")]
    Unknown(i32),
}

impl ErrorCode {
    pub fn from_ffi(code: ffi::gdstk::ErrorCode) -> Self {
        match code {
            ffi::gdstk::ErrorCode::NoError => Self::NoError,
            ffi::gdstk::ErrorCode::BooleanError => Self::BooleanError,
            ffi::gdstk::ErrorCode::EmptyPath => Self::EmptyPath,
            ffi::gdstk::ErrorCode::IntersectionNotFound => Self::IntersectionNotFound,
            ffi::gdstk::ErrorCode::MissingReference => Self::MissingReference,
            ffi::gdstk::ErrorCode::UnsupportedRecord => Self::UnsupportedRecord,
            ffi::gdstk::ErrorCode::UnofficialSpecification => Self::UnofficialSpecification,
            ffi::gdstk::ErrorCode::InvalidRepetition => Self::InvalidRepetition,
            ffi::gdstk::ErrorCode::Overflow => Self::Overflow,
            ffi::gdstk::ErrorCode::ChecksumError => Self::ChecksumError,
            ffi::gdstk::ErrorCode::OutputFileOpenError => Self::OutputFileOpenError,
            ffi::gdstk::ErrorCode::InputFileOpenError => Self::InputFileOpenError,
            ffi::gdstk::ErrorCode::InputFileError => Self::InputFileError,
            ffi::gdstk::ErrorCode::FileError => Self::FileError,
            ffi::gdstk::ErrorCode::InvalidFile => Self::InvalidFile,
            ffi::gdstk::ErrorCode::InsufficientMemory => Self::InsufficientMemory,
            ffi::gdstk::ErrorCode::ZlibError => Self::ZlibError,
            _ => Self::Unknown(code as i32),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LayerInterval {
    AllValues,
    UpperBound(u64),
    LowerBound(u64),
    SingleValue(u64),
    Bounded((u64, u64)),
}
impl LayerInterval {
    pub fn from_ffi(interval: ffi::rust_helper::LayerInterval) -> Self {
        match interval.interval_type {
            ffi::rust_helper::LayerIntervalType::AllValues => LayerInterval::AllValues,
            ffi::rust_helper::LayerIntervalType::UpperBound => {
                LayerInterval::UpperBound(interval.bound_a)
            }
            ffi::rust_helper::LayerIntervalType::LowerBound => {
                LayerInterval::LowerBound(interval.bound_a)
            }
            ffi::rust_helper::LayerIntervalType::SingleValue => {
                LayerInterval::SingleValue(interval.bound_a)
            }
            ffi::rust_helper::LayerIntervalType::Bounded => {
                LayerInterval::Bounded((interval.bound_a, interval.bound_b))
            }
        }
    }
}
pub trait ApplyTransform {
    fn apply_transform(&self, trans: Matrix3) -> Self;
}
impl ApplyTransform for Point {
    fn apply_transform(&self, trans: Matrix3) -> Self {
        let p_homo = Vector3::new(self.x, self.y, 1.0);
        let p = trans * p_homo;
        Point::new(p.x, p.y)
    }
}
pub trait ToPolygons {
    fn ffi_to_polygons(&self) -> UniquePtr<ffi::rust_helper::PolygonArrayTransfer>;
    fn to_polygons(&self) -> Vec<Polygon> {
        unsafe {
            let mut data = self.ffi_to_polygons();
            let mut pinned = data.pin_mut();
            let mut result: Vec<Polygon> = Vec::new();
            for i in 0..pinned.count() {
                result.push(Polygon::from_raw(pinned.as_mut().into(i)));
            }
            pinned.as_mut().cleanup();
            result
        }
    }
}
pub trait GetBoundingBox {
    fn bounding_box(&self) -> (Point, Point);
}
pub struct Polygon {
    pub(crate) inner: UniquePtr<ffi::rust_helper::PolygonOwner>,
}
extern "C" fn point_visitor_trampoline<F>(x: f64, y: f64, user_data: *mut autocxx::c_void) -> bool
where
    F: FnMut(f64, f64) -> bool,
{
    let closure = unsafe { &mut *(user_data as *mut F) };
    closure(x, y)
}
impl Polygon {
    pub fn new() -> Self {
        unsafe {
            Self {
                inner: ffi::rust_helper::polygon_new(),
            }
        }
    }
    pub(crate) fn from_raw(ptr: UniquePtr<ffi::rust_helper::PolygonOwner>) -> Self {
        Self { inner: ptr }
    }
    pub fn clone(&self) -> Self {
        unsafe {
            let mut dest = ffi::rust_helper::polygon_new();
            ffi::rust_helper::polygon_copy(&*self.inner, dest.pin_mut());
            Self { inner: dest }
        }
    }
    pub fn translate(&mut self, v: Vector) {
        unsafe {
            ffi::rust_helper::polygon_translate(self.inner.pin_mut(), v.x, v.y);
        }
    }
    pub fn scale(&mut self, scale: Vector, center: Point) {
        unsafe {
            ffi::rust_helper::polygon_scale(
                self.inner.pin_mut(),
                scale.x,
                scale.y,
                center.x,
                center.y,
            );
        }
    }
    pub fn mirror(&mut self, p0: Point, p1: Point) {
        unsafe {
            ffi::rust_helper::polygon_mirror(self.inner.pin_mut(), p0.x, p0.y, p1.x, p1.y);
        }
    }
    pub fn rotate(&mut self, angle: f64, center: Point) {
        unsafe {
            ffi::rust_helper::polygon_rotate(self.inner.pin_mut(), angle, center.x, center.y);
        }
    }
    pub fn layer(&self) -> u32 {
        unsafe { ffi::rust_helper::polygon_layer(&*self.inner) }
    }
    pub fn datatype(&self) -> u32 {
        unsafe { ffi::rust_helper::polygon_datatype(&*self.inner) }
    }
    pub fn foreach_point<F>(&self, mut f: F) -> bool
    where
        F: FnMut(f64, f64) -> bool,
    {
        unsafe {
            let callback_ptr = point_visitor_trampoline::<F> as *mut autocxx::c_void;
            let user_data_ptr = &mut f as *mut F as *mut autocxx::c_void;
            ffi::rust_helper::polygon_foreach_point(&*self.inner, callback_ptr, user_data_ptr)
        }
    }
    pub fn to_points(&self) -> Vec<Point> {
        let mut points = Vec::new();
        let _ = self.foreach_point(|x, y| {
            points.push(Point::new(x, y));
            true
        });
        points
    }
    pub fn to_ref(&self) -> PolygonRef<'_> {
        unsafe {
            PolygonRef {
                inner: ffi::rust_helper::polygon_to_ref(&*self.inner),
                _marker: std::marker::PhantomData,
            }
        }
    }
}
impl GetBoundingBox for Polygon {
    fn bounding_box(&self) -> (Point, Point) {
        unsafe {
            let bbox = ffi::rust_helper::polygon_get_bounding_box(&*self.inner);
            (
                Point::new(bbox.min.x, bbox.min.y),
                Point::new(bbox.max.x, bbox.max.y),
            )
        }
    }
}
pub struct Repetition<'a> {
    pub(crate) inner: *const ffi::gdstk::Repetition,
    pub(crate) _marker: std::marker::PhantomData<&'a ffi::gdstk::Repetition>,
}
impl<'a> Repetition<'a> {
    fn foreach_repetition_offset<F>(&self, mut f: F)
    where
        F: FnMut(f64, f64) -> bool,
    {
        unsafe {
            let callback_ptr = point_visitor_trampoline::<F> as *mut autocxx::c_void;
            let user_data_ptr = &mut f as *mut F as *mut autocxx::c_void;
            ffi::rust_helper::repetition_foreach_offset(&*self.inner, callback_ptr, user_data_ptr)
        }
    }
    fn to_offsets(&self) -> Vec<Vector> {
        let mut results = Vec::new();
        self.foreach_repetition_offset(|x, y| {
            results.push(Vector::new(x, y));
            true
        });
        results
    }
}
pub struct PolygonRef<'a> {
    pub(crate) inner: *const ffi::gdstk::Polygon,
    pub(crate) _marker: std::marker::PhantomData<&'a ffi::gdstk::Polygon>,
}
impl<'a> PolygonRef<'a> {
    pub fn to_polygon(&self) -> Polygon {
        unsafe {
            Polygon {
                inner: ffi::rust_helper::polygon_new_from_ref(&*self.inner),
            }
        }
    }
    pub fn layer(&self) -> u32 {
        unsafe { ffi::rust_helper::polygon_ref_layer(&*self.inner) }
    }
    pub fn datatype(&self) -> u32 {
        unsafe { ffi::rust_helper::polygon_ref_datatype(&*self.inner) }
    }
    pub fn foreach_point<F>(&self, mut f: F) -> bool
    where
        F: FnMut(f64, f64) -> bool,
    {
        unsafe {
            let callback_ptr = point_visitor_trampoline::<F> as *mut autocxx::c_void;
            let user_data_ptr = &mut f as *mut F as *mut autocxx::c_void;
            ffi::rust_helper::polygon_ref_foreach_point(&*self.inner, callback_ptr, user_data_ptr)
        }
    }
    pub fn to_points(&self) -> Vec<Point> {
        let mut points = Vec::new();
        let _ = self.foreach_point(|x, y| {
            points.push(Point::new(x, y));
            true
        });
        points
    }
    pub fn repetition_offsets(&self) -> Vec<Matrix3> {
        self.repetition()
            .to_offsets()
            .into_iter()
            .map(|v| Matrix3::from_translation(v))
            .collect()
    }
    pub fn repetition(&self) -> Repetition<'a> {
        unsafe {
            Repetition {
                inner: ffi::rust_helper::polygon_ref_get_repetition(&*self.inner),
                _marker: std::marker::PhantomData,
            }
        }
    }
}
impl<'a> GetBoundingBox for PolygonRef<'a> {
    fn bounding_box(&self) -> (Point, Point) {
        unsafe {
            let bbox = ffi::rust_helper::polygon_ref_get_bounding_box(&*self.inner);
            (
                Point::new(bbox.min.x, bbox.min.y),
                Point::new(bbox.max.x, bbox.max.y),
            )
        }
    }
}
pub struct FlexPath<'a> {
    pub(crate) inner: *const ffi::gdstk::FlexPath,
    pub(crate) _marker: std::marker::PhantomData<&'a ffi::gdstk::FlexPath>,
}
pub struct RobustPath<'a> {
    pub(crate) inner: *const ffi::gdstk::RobustPath,
    pub(crate) _marker: std::marker::PhantomData<&'a ffi::gdstk::RobustPath>,
}
impl<'a> ToPolygons for FlexPath<'a> {
    fn ffi_to_polygons(&self) -> UniquePtr<ffi::rust_helper::PolygonArrayTransfer> {
        unsafe { ffi::rust_helper::flexpath_to_polygons(&*self.inner).within_unique_ptr() }
    }
}
impl<'a> ToPolygons for RobustPath<'a> {
    fn ffi_to_polygons(&self) -> UniquePtr<ffi::rust_helper::PolygonArrayTransfer> {
        unsafe { ffi::rust_helper::robustpath_to_polygons(&*self.inner).within_unique_ptr() }
    }
}
pub struct Reference<'a> {
    pub(crate) inner: *const ffi::gdstk::Reference,
    pub(crate) _marker: std::marker::PhantomData<&'a Cell<'a>>,
}
impl<'a> Reference<'a> {
    pub(crate) fn traverse_shapes_recursive<V: ShapeVisitor>(
        &self,
        visitor: &mut V,
        trans: &Vec<Matrix3>,
    ) -> bool {
        let transform = self.transform();
        let offsets: Vec<_> = self
            .repetition()
            .to_offsets()
            .into_iter()
            .map(|v| Matrix3::from_translation(v))
            .collect();
        let trans2: Vec<_> = trans
            .iter()
            .flat_map(|t| offsets.iter().map(move |off| t * off * transform))
            .collect();
        if let Some(cell) = self.cell() {
            cell.traverse_shapes_recursive(visitor, &trans2);
        }
        true
    }
    fn cell(&self) -> Option<Cell<'_>> {
        unsafe {
            let ptr = ffi::rust_helper::reference_get_cell(&*self.inner);
            if ptr.is_null() {
                return None;
            }
            Some(Cell {
                inner: ptr,
                _marker: std::marker::PhantomData,
            })
        }
    }
    fn transform(&self) -> Matrix3 {
        self.translate() * self.rotation() * self.scale() * self.relfection()
    }
    fn translate(&self) -> Matrix3 {
        unsafe {
            let p = ffi::rust_helper::reference_get_translate(&*self.inner);
            Matrix3::from_translation(Vector::new(p.x, p.y))
        }
    }
    fn rotation(&self) -> Matrix3 {
        unsafe {
            let rad = ffi::rust_helper::reference_get_rotation(&*self.inner);
            Matrix3::from_angle_z(cgmath::Rad(rad))
        }
    }
    fn scale(&self) -> Matrix3 {
        unsafe {
            let s = ffi::rust_helper::reference_get_scale(&*self.inner);
            Matrix3::from_scale(s)
        }
    }
    fn relfection(&self) -> Matrix3 {
        unsafe {
            let x_ref = ffi::rust_helper::reference_get_x_reflection(&*self.inner);
            let r1 = if x_ref { -1.0 } else { 1.0 };
            Matrix3::from_nonuniform_scale(1.0, r1)
        }
    }
    pub fn repetition(&self) -> Repetition<'a> {
        unsafe {
            Repetition {
                inner: ffi::rust_helper::reference_get_repetition(&*self.inner),
                _marker: std::marker::PhantomData,
            }
        }
    }
}
pub struct Label<'a> {
    pub(crate) inner: *mut ffi::gdstk::Label,
    pub(crate) _marker: std::marker::PhantomData<&'a ffi::gdstk::Label>,
}
impl<'a> Label<'a> {
    pub fn text(&self) -> String {
        unsafe {
            let name_cxx = ffi::rust_helper::label_get_text(&*self.inner);
            name_cxx.to_string_lossy().into_owned()
        }
    }
    pub fn position(&self) -> Point {
        unsafe {
            let p = ffi::rust_helper::label_get_position(&*self.inner);
            Point::new(p.x, p.y)
        }
    }
}
impl GetBoundingBox for Label<'_> {
    fn bounding_box(&self) -> (Point, Point) {
        unsafe {
            let bbox = ffi::rust_helper::label_get_bounding_box(&*self.inner);
            (
                Point::new(bbox.min.x, bbox.min.y),
                Point::new(bbox.max.x, bbox.max.y),
            )
        }
    }
}
pub struct LayerName<'a> {
    pub(crate) inner: *const ffi::gdstk::LayerName,
    pub(crate) _marker: std::marker::PhantomData<&'a ffi::gdstk::LayerName>,
}
impl<'a> LayerName<'a> {
    pub fn name(&self) -> String {
        unsafe {
            let name_cxx = ffi::rust_helper::layername_get_name(&*self.inner);
            name_cxx.to_string_lossy().into_owned()
        }
    }
    pub fn layer(&self) -> u32 {
        unsafe { ffi::rust_helper::layername_get_layer(&*self.inner) }
    }
    pub fn datatype(&self) -> u32 {
        unsafe { ffi::rust_helper::layername_get_datatype(&*self.inner) }
    }
    pub fn layer_interval(&self) -> LayerInterval {
        unsafe {
            let interval = ffi::rust_helper::layername_get_layer_interval(&*self.inner);
            LayerInterval::from_ffi(interval)
        }
    }
    pub fn datatype_interval(&self) -> LayerInterval {
        unsafe {
            let interval = ffi::rust_helper::layername_get_datatype_interval(&*self.inner);
            LayerInterval::from_ffi(interval)
        }
    }
}
pub struct Cell<'a> {
    pub(crate) inner: *const ffi::gdstk::Cell,
    pub(crate) _marker: std::marker::PhantomData<&'a ffi::gdstk::Cell>,
}
impl<'a> Cell<'a> {
    pub fn name(&self) -> String {
        unsafe {
            let name_cxx = ffi::rust_helper::cell_get_name(&*self.inner);
            name_cxx.to_string_lossy().into_owned()
        }
    }
    pub fn get_polygons_full(
        &self,
        layer: Option<u32>,
        datatype: Option<u32>,
        apply_repetitions: Option<bool>,
        include_paths: Option<bool>,
        depth: Option<i64>,
    ) -> Vec<Polygon> {
        unsafe {
            let filter = !layer.is_none() && !datatype.is_none();
            let layer = layer.unwrap_or(0);
            let datatype = datatype.unwrap_or(0);
            let mut data = ffi::rust_helper::cell_get_polygons(
                &*self.inner,
                apply_repetitions.unwrap_or(true),
                include_paths.unwrap_or(true),
                depth.unwrap_or(-1),
                filter,
                ffi::gdstk::make_tag(layer, datatype),
            )
            .within_unique_ptr();

            let mut pinned = data.pin_mut();
            let mut result: Vec<Polygon> = Vec::new();
            for i in 0..pinned.count() {
                result.push(Polygon::from_raw(pinned.as_mut().into(i)));
            }
            pinned.as_mut().cleanup();
            result
        }
    }
    pub fn get_polygons(&self, layer: Option<u32>, datatype: Option<u32>) -> Vec<Polygon> {
        self.get_polygons_full(layer, datatype, None, None, None)
    }
    pub fn count_polygon_refs(&self) -> usize {
        unsafe { ffi::rust_helper::cell_count_polygon_refs(&*self.inner) }
    }
    pub fn polygon_ref(&self, i: usize) -> PolygonRef<'_> {
        unsafe {
            PolygonRef {
                inner: ffi::rust_helper::cell_get_polygon_ref(&*self.inner, i),
                _marker: std::marker::PhantomData,
            }
        }
    }
    pub fn count_references(&self) -> usize {
        unsafe { ffi::rust_helper::cell_count_references(&*self.inner) }
    }
    pub fn reference(&self, i: usize) -> Reference<'_> {
        unsafe {
            Reference {
                inner: ffi::rust_helper::cell_get_reference(&*self.inner, i),
                _marker: std::marker::PhantomData,
            }
        }
    }
    pub fn count_flexpaths(&self) -> usize {
        unsafe { ffi::rust_helper::cell_count_flexpaths(&*self.inner) }
    }
    pub fn flexpath(&self, i: usize) -> FlexPath<'_> {
        unsafe {
            FlexPath {
                inner: ffi::rust_helper::cell_get_flexpath(&*self.inner, i),
                _marker: std::marker::PhantomData,
            }
        }
    }
    pub fn count_robustpaths(&self) -> usize {
        unsafe { ffi::rust_helper::cell_count_robustpaths(&*self.inner) }
    }
    pub fn robustpath(&self, i: usize) -> RobustPath<'_> {
        unsafe {
            RobustPath {
                inner: ffi::rust_helper::cell_get_robustpath(&*self.inner, i),
                _marker: std::marker::PhantomData,
            }
        }
    }
    pub fn count_labels(&self) -> usize {
        unsafe { ffi::rust_helper::cell_count_labels(&*self.inner) }
    }
    pub fn label(&self, i: usize) -> Label<'_> {
        unsafe {
            Label {
                inner: ffi::rust_helper::cell_get_label(&*self.inner, i),
                _marker: std::marker::PhantomData,
            }
        }
    }
    pub fn traverse_shapes<V: ShapeVisitor>(&self, visitor: &mut V) -> bool {
        let trans = vec![Matrix3::one()];
        self.traverse_shapes_recursive(visitor, &trans)
    }
    pub(crate) fn traverse_shapes_recursive<V: ShapeVisitor>(
        &self,
        visitor: &mut V,
        trans: &Vec<Matrix3>,
    ) -> bool {
        visitor.on_start_cell(&self);
        for i in 0..self.count_polygon_refs() {
            let poly = self.polygon_ref(i);
            if !visitor.on_polygon(&poly, &self, i, trans) {
                return false;
            }
        }
        // flexpath
        for i in 0..self.count_flexpaths() {
            let path = self.flexpath(i);
            if !visitor.on_flexpath(&path, &self, i, trans) {
                return false;
            }
        }
        // robustpath
        for i in 0..self.count_robustpaths() {
            let path = self.robustpath(i);
            if !visitor.on_robustpath(&path, &self, i, trans) {
                return false;
            }
        }
        visitor.on_end_cell();

        for i in 0..self.count_references() {
            if !self.reference(i).traverse_shapes_recursive(visitor, trans) {
                return false;
            }
        }
        true
    }
    pub fn traverse_polygons<F>(&self, mut f: F) -> bool
    where
        F: FnMut(&PolygonRef, &Cell, &Vec<Matrix3>) -> bool,
    {
        let mut visitor = CellPolygonVisitor { f };
        self.traverse_shapes(&mut visitor)
    }
}
pub trait ShapeVisitor {
    fn on_start_cell(&mut self, cell: &Cell) -> bool;
    fn on_end_cell(&mut self) -> bool;
    fn on_polygon(
        &mut self,
        poly: &PolygonRef,
        parent: &Cell,
        polygon_index: usize,
        trans: &Vec<Matrix3>,
    ) -> bool;
    fn on_flexpath(
        &mut self,
        flexpath: &FlexPath,
        parent: &Cell,
        flexpath_index: usize,
        trans: &Vec<Matrix3>,
    ) -> bool;
    fn on_robustpath(
        &mut self,
        robustpath: &RobustPath,
        parent: &Cell,
        robustpath_index: usize,
        trans: &Vec<Matrix3>,
    ) -> bool;
}
struct CellPolygonVisitor<F> {
    f: F,
}
impl<F> ShapeVisitor for CellPolygonVisitor<F>
where
    F: FnMut(&PolygonRef, &Cell, &Vec<Matrix3>) -> bool,
{
    fn on_start_cell(&mut self, cell: &Cell) -> bool {
        true
    }
    fn on_end_cell(&mut self) -> bool {
        true
    }
    fn on_polygon(
        &mut self,
        poly: &PolygonRef,
        parent: &Cell,
        _polygon_index: usize,
        trans: &Vec<Matrix3>,
    ) -> bool {
        (self.f)(poly, parent, trans)
    }
    fn on_flexpath(
        &mut self,
        flexpath: &FlexPath,
        parent: &Cell,
        _flexpath_index: usize,
        trans: &Vec<Matrix3>,
    ) -> bool {
        for polygon in flexpath.to_polygons() {
            if !(self.f)(&polygon.to_ref(), parent, trans) {
                return false;
            }
        }
        true
    }
    fn on_robustpath(
        &mut self,
        robustpath: &RobustPath,
        parent: &Cell,
        _robustpath_index: usize,
        trans: &Vec<Matrix3>,
    ) -> bool {
        for polygon in robustpath.to_polygons() {
            if !(self.f)(&polygon.to_ref(), parent, trans) {
                return false;
            }
        }
        true
    }
}
impl<'a> GetBoundingBox for Cell<'a> {
    fn bounding_box(&self) -> (Point, Point) {
        unsafe {
            let bbox = ffi::rust_helper::cell_get_bounding_box(&*self.inner);
            (
                Point::new(bbox.min.x, bbox.min.y),
                Point::new(bbox.max.x, bbox.max.y),
            )
        }
    }
}
pub struct RawCell<'a> {
    pub(crate) inner: *const ffi::gdstk::RawCell,
    pub(crate) _marker: std::marker::PhantomData<&'a Library>,
}
impl<'a> RawCell<'a> {
    pub fn name(&self) -> String {
        unsafe {
            let name_cxx = ffi::rust_helper::rawcell_get_name(&*self.inner);
            name_cxx.to_string_lossy().into_owned()
        }
    }
}

pub struct Library {
    inner: UniquePtr<ffi::rust_helper::LibraryOwner>,
}
impl Library {
    pub fn from_oas(path: &str) -> Result<Self, ErrorCode> {
        let c_path = CString::new(path).map_err(|_| ErrorCode::InputFileOpenError)?;
        unsafe {
            let mut error_code = ffi::gdstk::ErrorCode::NoError;
            let ptr =
                ffi::rust_helper::library_read_oas(c_path.as_ptr(), 0.0, 1e-9, &mut error_code);
            let error_code = ErrorCode::from_ffi(error_code);
            if ptr.is_null() || error_code != ErrorCode::NoError {
                Err(error_code)
            } else {
                Ok(Self { inner: ptr })
            }
        }
    }
    pub fn from_gds(path: &str, unit: f64, tolerance: f64) -> Result<Self, ErrorCode> {
        let c_path = CString::new(path).map_err(|_| ErrorCode::InputFileOpenError)?;
        unsafe {
            let mut error_code = ffi::gdstk::ErrorCode::NoError;
            let ptr =
                ffi::rust_helper::library_read_gds(c_path.as_ptr(), 0.0, 1e-9, &mut error_code);
            let error_code = ErrorCode::from_ffi(error_code);
            if ptr.is_null() || error_code != ErrorCode::NoError {
                Err(error_code)
            } else {
                Ok(Self { inner: ptr })
            }
        }
    }
    pub fn top_level(&self) -> (Vec<Cell<'_>>, Vec<RawCell<'_>>) {
        unsafe {
            let mut result =
                ffi::rust_helper::library_get_top_level(&self.inner).within_unique_ptr();
            let mut cells: Vec<Cell> = Vec::new();
            let mut rawcells: Vec<RawCell> = Vec::new();
            for i in 0..result.n_cells() {
                cells.push(Cell {
                    inner: result.cell(i),
                    _marker: std::marker::PhantomData,
                });
            }
            for i in 0..result.n_rawcells() {
                rawcells.push(RawCell {
                    inner: result.rawcell(i),
                    _marker: std::marker::PhantomData,
                });
            }
            let mut pinned = result.pin_mut();
            pinned.as_mut().cleanup();
            (cells, rawcells)
        }
    }
    pub fn get_cell(&self, name: &str) -> Cell<'_> {
        let c_name = CString::new(name).unwrap();
        unsafe {
            Cell {
                inner: ffi::rust_helper::library_get_cell(&*self.inner, c_name.as_ptr()),
                _marker: std::marker::PhantomData,
            }
        }
    }
    pub fn get_rawcell(&self, name: &str) -> RawCell<'_> {
        let c_name = CString::new(name).unwrap();
        unsafe {
            RawCell {
                inner: ffi::rust_helper::library_get_rawcell(&*self.inner, c_name.as_ptr()),
                _marker: std::marker::PhantomData,
            }
        }
    }
    pub fn layername(&self, i: usize) -> LayerName<'_> {
        unsafe {
            LayerName {
                inner: ffi::rust_helper::library_get_layername(&*self.inner, i),
                _marker: std::marker::PhantomData,
            }
        }
    }
    pub fn count_layernames(&self) -> usize {
        unsafe { ffi::rust_helper::library_count_layernames(&*self.inner) }
    }
}
