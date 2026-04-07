#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(unsafe_op_in_unsafe_fn)]

use autocxx::prelude::*;
use glam;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::CString;
use thiserror::Error;

pub type Point = glam::DVec2;
pub type Vector = glam::DVec2;
pub type Vector3 = glam::DVec3;
pub type Matrix3 = glam::DMat3;

include_cpp! {
    #include "wrapper.h"
    safety!(unsafe)
    // transfer objects
    generate_pod!("gdstk_parse_rs::Point2D")
    generate_pod!("gdstk_parse_rs::BoundingBox")
    generate!("gdstk_parse_rs::PolygonArrayTransfer")
    generate!("gdstk_parse_rs::TopLevelResult")
    generate_pod!("gdstk_parse_rs::LayerInterval")

    // gdstk objects
    generate!("gdstk::Polygon")
    generate!("gdstk::Cell")
    generate!("gdstk::Library")
    generate!("gdstk::Label")
    generate!("gdstk::LayerName")
    generate!("gdstk::ErrorCode")
    generate!("gdstk::Tag")
    generate!("gdstk::make_tag")

    // Library
    generate!("gdstk_parse_rs::LibraryOwner")
    generate!("gdstk_parse_rs::library_read_gds")
    generate!("gdstk_parse_rs::library_read_oas")
    generate!("gdstk_parse_rs::library_get_top_level")
    generate!("gdstk_parse_rs::library_get_cell")
    generate!("gdstk_parse_rs::library_get_rawcell")
    generate!("gdstk_parse_rs::library_get_unit")
    generate!("gdstk_parse_rs::library_get_precision")
    generate!("gdstk_parse_rs::library_count_layernames")
    generate!("gdstk_parse_rs::library_get_layername")
    generate!("gdstk_parse_rs::library_create_geometry_cache")
    generate!("gdstk_parse_rs::library_count_cells")
    generate!("gdstk_parse_rs::library_get_cell_by_index")

    // Label
    generate!("gdstk_parse_rs::label_get_text")
    generate!("gdstk_parse_rs::label_get_position")
    generate!("gdstk_parse_rs::label_get_bounding_box")

    // LayerName
    generate!("gdstk_parse_rs::layername_get_name")
    generate!("gdstk_parse_rs::layername_get_layer")
    generate!("gdstk_parse_rs::layername_get_datatype")
    generate!("gdstk_parse_rs::layername_get_layer_interval")
    generate!("gdstk_parse_rs::layername_get_datatype_interval")

    // cell
    generate!("gdstk_parse_rs::cell_get_name")
    generate!("gdstk_parse_rs::cell_get_polygons")
    generate!("gdstk_parse_rs::cell_get_bounding_box")
    generate!("gdstk_parse_rs::cell_count_polygon_refs")
    generate!("gdstk_parse_rs::cell_get_polygon_ref")
    generate!("gdstk_parse_rs::cell_count_references")
    generate!("gdstk_parse_rs::cell_get_reference")
    generate!("gdstk_parse_rs::cell_count_flexpaths")
    generate!("gdstk_parse_rs::cell_get_flexpath")
    generate!("gdstk_parse_rs::cell_count_robustpaths")
    generate!("gdstk_parse_rs::cell_get_robustpath")
    generate!("gdstk_parse_rs::cell_count_labels")
    generate!("gdstk_parse_rs::cell_get_label")

    // polygon
    generate!("gdstk_parse_rs::PolygonOwner")
    generate!("gdstk_parse_rs::polygon_new")
    generate!("gdstk_parse_rs::polygon_new_from_ref")
    generate!("gdstk_parse_rs::polygon_copy")
    generate!("gdstk_parse_rs::polygon_translate")
    generate!("gdstk_parse_rs::polygon_scale")
    generate!("gdstk_parse_rs::polygon_mirror")
    generate!("gdstk_parse_rs::polygon_rotate")
    generate!("gdstk_parse_rs::polygon_layer")
    generate!("gdstk_parse_rs::polygon_datatype")
    generate!("gdstk_parse_rs::polygon_foreach_point")
    generate!("gdstk_parse_rs::PointCallback")
    generate!("gdstk_parse_rs::polygon_get_bounding_box")
    generate!("gdstk_parse_rs::polygon_to_ref")
    generate!("gdstk_parse_rs::polygon_get_signed_area")

    // polygon_ref
    generate!("gdstk_parse_rs::polygon_ref_get_bounding_box")
    generate!("gdstk_parse_rs::polygon_ref_get_repetition")
    generate!("gdstk_parse_rs::polygon_ref_layer")
    generate!("gdstk_parse_rs::polygon_ref_datatype")
    generate!("gdstk_parse_rs::polygon_ref_foreach_point")
    generate!("gdstk_parse_rs::polygon_ref_get_signed_area")

    // flexpath
    generate!("gdstk_parse_rs::flexpath_to_polygons")

    // robustpath
    generate!("gdstk_parse_rs::robustpath_to_polygons")

    // reference
    generate!("gdstk_parse_rs::reference_get_translate")
    generate!("gdstk_parse_rs::reference_get_scale")
    generate!("gdstk_parse_rs::reference_get_rotation")
    generate!("gdstk_parse_rs::reference_get_x_reflection")
    generate!("gdstk_parse_rs::reference_get_repetition")
    generate!("gdstk_parse_rs::reference_get_cell")

    // Repetition
    generate!("gdstk_parse_rs::repetition_foreach_offset")

    // rawcell
    generate!("gdstk_parse_rs::rawcell_get_name")

    // GeometryCache
    generate!("gdstk_parse_rs::GeometryCacheOwner")
    generate!("gdstk_parse_rs::geometry_cache_get_bounding_box")
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
    pub fn from_ffi(interval: ffi::gdstk_parse_rs::LayerInterval) -> Self {
        match interval.interval_type {
            ffi::gdstk_parse_rs::LayerIntervalType::AllValues => LayerInterval::AllValues,
            ffi::gdstk_parse_rs::LayerIntervalType::UpperBound => {
                LayerInterval::UpperBound(interval.bound_a)
            }
            ffi::gdstk_parse_rs::LayerIntervalType::LowerBound => {
                LayerInterval::LowerBound(interval.bound_a)
            }
            ffi::gdstk_parse_rs::LayerIntervalType::SingleValue => {
                LayerInterval::SingleValue(interval.bound_a)
            }
            ffi::gdstk_parse_rs::LayerIntervalType::Bounded => {
                LayerInterval::Bounded((interval.bound_a, interval.bound_b))
            }
        }
    }
}
pub trait ApplyTransform {
    fn apply_transform(&self, trans: &Matrix3) -> Self;
}
impl ApplyTransform for Point {
    fn apply_transform(&self, trans: &Matrix3) -> Self {
        let p_homo = Vector3::new(self.x, self.y, 1.0);
        let p = trans * p_homo;
        Point::new(p.x, p.y)
    }
}
impl ApplyTransform for Vec<Point> {
    fn apply_transform(&self, trans: &Matrix3) -> Self {
        self.iter().map(|p| p.apply_transform(trans)).collect()
    }
}
pub trait ToPolygons {
    fn ffi_to_polygons(&self) -> UniquePtr<ffi::gdstk_parse_rs::PolygonArrayTransfer>;
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
#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub min: Point,
    pub max: Point,
}
impl Rect {
    pub fn new(min: Point, max: Point) -> Self {
        Self { min, max }
    }
    pub fn invalid_new() -> Self {
        Self {
            min: Point::new(f64::MAX, f64::MAX),
            max: Point::new(-f64::MAX, -f64::MAX),
        }
    }
    pub fn expand(&mut self, p: Point) {
        self.min = Point::new(self.min.x.min(p.x), self.min.y.min(p.y));
        self.max = Point::new(self.max.x.max(p.x), self.max.y.max(p.y));
    }
    pub fn min_max(&self) -> (Point, Point) {
        (self.min, self.max)
    }
    pub fn intersect(&self, other: &Self) -> bool {
        let (min1, max1) = self.min_max();
        let (min2, max2) = other.min_max();
        if max1.x < min2.x || max2.x < min1.x {
            return false;
        }
        if max1.y < min2.y || max2.y < min1.y {
            return false;
        }
        true
    }
    pub fn intersect_strictly(&self, other: &Self) -> bool {
        let (min1, max1) = self.min_max();
        let (min2, max2) = other.min_max();
        if max1.x <= min2.x || max2.x <= min1.x {
            return false;
        }
        if max1.y <= min2.y || max2.y <= min1.y {
            return false;
        }
        true
    }
}
impl ApplyTransform for Rect {
    fn apply_transform(&self, trans: &Matrix3) -> Self {
        let (min, max) = self.min_max();
        Rect::new(min.apply_transform(trans), max.apply_transform(trans))
    }
}
pub trait GetBoundingBox {
    fn bounding_box(&self) -> Rect;
}
impl GetBoundingBox for Vec<Point> {
    fn bounding_box(&self) -> Rect {
        let mut bbox = Rect::invalid_new();
        for p in self.iter() {
            bbox.expand(*p);
        }
        bbox
    }
}
pub struct Polygon {
    pub(crate) inner: UniquePtr<ffi::gdstk_parse_rs::PolygonOwner>,
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
                inner: ffi::gdstk_parse_rs::polygon_new(),
            }
        }
    }
    pub(crate) fn from_raw(ptr: UniquePtr<ffi::gdstk_parse_rs::PolygonOwner>) -> Self {
        Self { inner: ptr }
    }
    pub fn clone(&self) -> Self {
        unsafe {
            let mut dest = ffi::gdstk_parse_rs::polygon_new();
            ffi::gdstk_parse_rs::polygon_copy(&*self.inner, dest.pin_mut());
            Self { inner: dest }
        }
    }
    pub fn translate(&mut self, v: Vector) {
        unsafe {
            ffi::gdstk_parse_rs::polygon_translate(self.inner.pin_mut(), v.x, v.y);
        }
    }
    pub fn scale(&mut self, scale: Vector, center: Point) {
        unsafe {
            ffi::gdstk_parse_rs::polygon_scale(
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
            ffi::gdstk_parse_rs::polygon_mirror(self.inner.pin_mut(), p0.x, p0.y, p1.x, p1.y);
        }
    }
    pub fn rotate(&mut self, angle: f64, center: Point) {
        unsafe {
            ffi::gdstk_parse_rs::polygon_rotate(self.inner.pin_mut(), angle, center.x, center.y);
        }
    }
    pub fn layer(&self) -> u32 {
        unsafe { ffi::gdstk_parse_rs::polygon_layer(&*self.inner) }
    }
    pub fn datatype(&self) -> u32 {
        unsafe { ffi::gdstk_parse_rs::polygon_datatype(&*self.inner) }
    }
    pub fn foreach_point<F>(&self, mut f: F) -> bool
    where
        F: FnMut(f64, f64) -> bool,
    {
        unsafe {
            let callback_ptr = point_visitor_trampoline::<F> as *mut autocxx::c_void;
            let user_data_ptr = &mut f as *mut F as *mut autocxx::c_void;
            ffi::gdstk_parse_rs::polygon_foreach_point(&*self.inner, callback_ptr, user_data_ptr)
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
                inner: ffi::gdstk_parse_rs::polygon_to_ref(&*self.inner),
                _marker: std::marker::PhantomData,
            }
        }
    }
    /// return signed area of polygon
    /// Polygon area excluding repetitions with sign indicating orientation
    /// (positive for counter clockwise)
    pub fn signed_area(&self) -> f64 {
        unsafe { unsafe { ffi::gdstk_parse_rs::polygon_get_signed_area(&*self.inner) } }
    }
}
impl GetBoundingBox for Polygon {
    fn bounding_box(&self) -> Rect {
        unsafe {
            let bbox = ffi::gdstk_parse_rs::polygon_get_bounding_box(&*self.inner);
            Rect::new(
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
            ffi::gdstk_parse_rs::repetition_foreach_offset(
                &*self.inner,
                callback_ptr,
                user_data_ptr,
            )
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
                inner: ffi::gdstk_parse_rs::polygon_new_from_ref(&*self.inner),
            }
        }
    }
    pub fn layer(&self) -> u32 {
        unsafe { ffi::gdstk_parse_rs::polygon_ref_layer(&*self.inner) }
    }
    pub fn datatype(&self) -> u32 {
        unsafe { ffi::gdstk_parse_rs::polygon_ref_datatype(&*self.inner) }
    }
    pub fn foreach_point<F>(&self, mut f: F) -> bool
    where
        F: FnMut(f64, f64) -> bool,
    {
        unsafe {
            let callback_ptr = point_visitor_trampoline::<F> as *mut autocxx::c_void;
            let user_data_ptr = &mut f as *mut F as *mut autocxx::c_void;
            ffi::gdstk_parse_rs::polygon_ref_foreach_point(
                &*self.inner,
                callback_ptr,
                user_data_ptr,
            )
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
                inner: ffi::gdstk_parse_rs::polygon_ref_get_repetition(&*self.inner),
                _marker: std::marker::PhantomData,
            }
        }
    }
    /// return signed area of polygon
    /// Polygon area excluding repetitions with sign indicating orientation
    /// (positive for counter clockwise)
    pub fn signed_area(&self) -> f64 {
        unsafe { ffi::gdstk_parse_rs::polygon_ref_get_signed_area(&*self.inner) }
    }
}
impl<'a> GetBoundingBox for PolygonRef<'a> {
    /// return bounding box
    /// repetitions are taken into account
    fn bounding_box(&self) -> Rect {
        unsafe {
            let bbox = ffi::gdstk_parse_rs::polygon_ref_get_bounding_box(&*self.inner);
            Rect::new(
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
    fn ffi_to_polygons(&self) -> UniquePtr<ffi::gdstk_parse_rs::PolygonArrayTransfer> {
        unsafe { ffi::gdstk_parse_rs::flexpath_to_polygons(&*self.inner).within_unique_ptr() }
    }
}
impl<'a> ToPolygons for RobustPath<'a> {
    fn ffi_to_polygons(&self) -> UniquePtr<ffi::gdstk_parse_rs::PolygonArrayTransfer> {
        unsafe { ffi::gdstk_parse_rs::robustpath_to_polygons(&*self.inner).within_unique_ptr() }
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
        if let Some(cell) = self.cell() {
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
            let trans2 = visitor.filter_reference(&cell, trans2);
            if trans2.len() > 0 {
                return cell.traverse_shapes_recursive(visitor, &trans2);
            }
        }
        true
    }
    fn cell(&self) -> Option<Cell<'_>> {
        unsafe {
            let ptr = ffi::gdstk_parse_rs::reference_get_cell(&*self.inner);
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
            let p = ffi::gdstk_parse_rs::reference_get_translate(&*self.inner);
            Matrix3::from_translation(Vector::new(p.x, p.y))
        }
    }
    fn rotation(&self) -> Matrix3 {
        unsafe {
            let rad = ffi::gdstk_parse_rs::reference_get_rotation(&*self.inner);
            Matrix3::from_rotation_z(rad)
        }
    }
    fn scale(&self) -> Matrix3 {
        unsafe {
            let s = ffi::gdstk_parse_rs::reference_get_scale(&*self.inner);
            Matrix3::from_diagonal(Vector3::new(s, s, 1.0))
        }
    }
    fn relfection(&self) -> Matrix3 {
        unsafe {
            let x_ref = ffi::gdstk_parse_rs::reference_get_x_reflection(&*self.inner);
            let r1 = if x_ref { -1.0 } else { 1.0 };
            Matrix3::from_diagonal(Vector3::new(1.0, r1, 1.0))
        }
    }
    pub fn repetition(&self) -> Repetition<'a> {
        unsafe {
            Repetition {
                inner: ffi::gdstk_parse_rs::reference_get_repetition(&*self.inner),
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
            let name_cxx = ffi::gdstk_parse_rs::label_get_text(&*self.inner);
            name_cxx.to_string_lossy().into_owned()
        }
    }
    pub fn position(&self) -> Point {
        unsafe {
            let p = ffi::gdstk_parse_rs::label_get_position(&*self.inner);
            Point::new(p.x, p.y)
        }
    }
}
impl GetBoundingBox for Label<'_> {
    fn bounding_box(&self) -> Rect {
        unsafe {
            let bbox = ffi::gdstk_parse_rs::label_get_bounding_box(&*self.inner);
            Rect::new(
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
            let name_cxx = ffi::gdstk_parse_rs::layername_get_name(&*self.inner);
            name_cxx.to_string_lossy().into_owned()
        }
    }
    pub fn layer(&self) -> u32 {
        unsafe { ffi::gdstk_parse_rs::layername_get_layer(&*self.inner) }
    }
    pub fn datatype(&self) -> u32 {
        unsafe { ffi::gdstk_parse_rs::layername_get_datatype(&*self.inner) }
    }
    pub fn layer_interval(&self) -> LayerInterval {
        unsafe {
            let interval = ffi::gdstk_parse_rs::layername_get_layer_interval(&*self.inner);
            LayerInterval::from_ffi(interval)
        }
    }
    pub fn datatype_interval(&self) -> LayerInterval {
        unsafe {
            let interval = ffi::gdstk_parse_rs::layername_get_datatype_interval(&*self.inner);
            LayerInterval::from_ffi(interval)
        }
    }
}
pub enum TraverseStatus {
    Continue,
    Skip,
    Finish,
}
pub struct Cell<'a> {
    pub(crate) inner: *const ffi::gdstk::Cell,
    pub(crate) _marker: std::marker::PhantomData<&'a ffi::gdstk::Cell>,
}
impl<'a> Cell<'a> {
    pub fn name(&self) -> String {
        unsafe {
            let name_cxx = ffi::gdstk_parse_rs::cell_get_name(&*self.inner);
            name_cxx.to_string_lossy().into_owned()
        }
    }
    pub fn id(&self) -> usize {
        self.inner as usize
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
            let mut data = ffi::gdstk_parse_rs::cell_get_polygons(
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
        unsafe { ffi::gdstk_parse_rs::cell_count_polygon_refs(&*self.inner) }
    }
    pub fn polygon_ref(&self, i: usize) -> PolygonRef<'_> {
        unsafe {
            PolygonRef {
                inner: ffi::gdstk_parse_rs::cell_get_polygon_ref(&*self.inner, i),
                _marker: std::marker::PhantomData,
            }
        }
    }
    pub fn count_references(&self) -> usize {
        unsafe { ffi::gdstk_parse_rs::cell_count_references(&*self.inner) }
    }
    pub fn reference(&self, i: usize) -> Reference<'_> {
        unsafe {
            Reference {
                inner: ffi::gdstk_parse_rs::cell_get_reference(&*self.inner, i),
                _marker: std::marker::PhantomData,
            }
        }
    }
    pub fn count_flexpaths(&self) -> usize {
        unsafe { ffi::gdstk_parse_rs::cell_count_flexpaths(&*self.inner) }
    }
    pub fn flexpath(&self, i: usize) -> FlexPath<'_> {
        unsafe {
            FlexPath {
                inner: ffi::gdstk_parse_rs::cell_get_flexpath(&*self.inner, i),
                _marker: std::marker::PhantomData,
            }
        }
    }
    pub fn count_robustpaths(&self) -> usize {
        unsafe { ffi::gdstk_parse_rs::cell_count_robustpaths(&*self.inner) }
    }
    pub fn robustpath(&self, i: usize) -> RobustPath<'_> {
        unsafe {
            RobustPath {
                inner: ffi::gdstk_parse_rs::cell_get_robustpath(&*self.inner, i),
                _marker: std::marker::PhantomData,
            }
        }
    }
    pub fn count_labels(&self) -> usize {
        unsafe { ffi::gdstk_parse_rs::cell_count_labels(&*self.inner) }
    }
    pub fn label(&self, i: usize) -> Label<'_> {
        unsafe {
            Label {
                inner: ffi::gdstk_parse_rs::cell_get_label(&*self.inner, i),
                _marker: std::marker::PhantomData,
            }
        }
    }
    pub fn traverse_shapes<V: ShapeVisitor>(&self, visitor: &mut V) -> bool {
        let trans = vec![Matrix3::IDENTITY];
        self.traverse_shapes_recursive(visitor, &trans)
    }
    pub(crate) fn traverse_shapes_recursive<V: ShapeVisitor>(
        &self,
        visitor: &mut V,
        trans: &Vec<Matrix3>,
    ) -> bool {
        match visitor.on_start_cell(&self) {
            TraverseStatus::Continue => {}
            TraverseStatus::Skip => return true,
            TraverseStatus::Finish => return false,
        }
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
        match visitor.on_end_cell(&self) {
            TraverseStatus::Continue => {}
            TraverseStatus::Skip => return true,
            TraverseStatus::Finish => return false,
        }

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
    pub fn traverse_polygons_with_overlap<'b, F>(
        &self,
        area: Rect,
        cache: &'b BoundingBoxCache,
        mut f: F,
    ) -> bool
    where
        F: FnMut(Vec<Point>, Rect, &PolygonRef, &Cell) -> bool,
    {
        let mut visitor = CellPolygonVisitorWithOverlap {
            f,
            area,
            strictly: false,
            cache,
        };
        self.traverse_shapes(&mut visitor)
    }
    pub fn traverse_polygons_with_overlap_strictly<'b, F>(
        &self,
        area: Rect,
        cache: &'b BoundingBoxCache,
        mut f: F,
    ) -> bool
    where
        F: FnMut(Vec<Point>, Rect, &PolygonRef, &Cell) -> bool,
    {
        let mut visitor = CellPolygonVisitorWithOverlap {
            f,
            area,
            strictly: true,
            cache,
        };
        self.traverse_shapes(&mut visitor)
    }
}
pub enum ShapeTaverseStatus {
    Continue,
    Skip,
    Finish,
}
pub trait ShapeVisitor {
    fn on_start_cell(&mut self, cell: &Cell) -> TraverseStatus;
    fn on_end_cell(&mut self, cell: &Cell) -> TraverseStatus;
    fn filter_reference(&mut self, cell: &Cell, trans: Vec<Matrix3>) -> Vec<Matrix3>;
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
    fn on_start_cell(&mut self, _cell: &Cell) -> TraverseStatus {
        TraverseStatus::Continue
    }
    fn on_end_cell(&mut self, _cell: &Cell) -> TraverseStatus {
        TraverseStatus::Continue
    }
    fn filter_reference(&mut self, _cell: &Cell, trans: Vec<Matrix3>) -> Vec<Matrix3> {
        trans
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
pub struct BoundingBoxCache<'a> {
    map: HashMap<usize, Rect>,
    pub(crate) _marker: std::marker::PhantomData<&'a ()>,
}
impl<'a> BoundingBoxCache<'a> {
    fn add(&mut self, id: usize, area: Rect) {
        self.map.entry(id).or_insert(area);
    }
    fn get(&self, id: usize) -> Option<Rect> {
        self.map.get(&id).copied()
    }
}
fn area_is_overlap(area1: (Point, Point), area2: (Point, Point)) -> bool {
    let (min1, max1) = area1;
    let (min2, max2) = area2;
    if max1.x < min2.x || max2.x < min1.x {
        return false;
    }
    if max1.y < min2.y || max2.y < min1.y {
        return false;
    }
    true
}
fn area_is_overlap_strictly(area1: (Point, Point), area2: (Point, Point)) -> bool {
    let (min1, max1) = area1;
    let (min2, max2) = area2;
    if max1.x <= min2.x || max2.x <= min1.x {
        return false;
    }
    if max1.y <= min2.y || max2.y <= min1.y {
        return false;
    }
    true
}
struct CellPolygonVisitorWithOverlap<'a, F> {
    f: F,
    area: Rect,
    strictly: bool,
    cache: &'a BoundingBoxCache<'a>,
}
impl<'a, F> CellPolygonVisitorWithOverlap<'a, F> {
    fn has_intersect(&self, area: &Rect) -> bool {
        if self.strictly {
            self.area.intersect(area)
        } else {
            self.area.intersect_strictly(area)
        }
    }
}
impl<F> ShapeVisitor for CellPolygonVisitorWithOverlap<'_, F>
where
    F: FnMut(Vec<Point>, Rect, &PolygonRef, &Cell) -> bool,
{
    fn on_start_cell(&mut self, cell: &Cell) -> TraverseStatus {
        TraverseStatus::Continue
    }
    fn on_end_cell(&mut self, cell: &Cell) -> TraverseStatus {
        TraverseStatus::Continue
    }
    fn filter_reference(&mut self, cell: &Cell, trans: Vec<Matrix3>) -> Vec<Matrix3> {
        let area = self.cache.get(cell.id()).expect(&format!(
            "not found cell id {} ({})",
            cell.id(),
            cell.name()
        ));
        trans
            .into_iter()
            .filter_map(|t| {
                let area2 = area.apply_transform(&t);
                if self.has_intersect(&area2) {
                    Some(t)
                } else {
                    None
                }
            })
            .collect()
    }
    fn on_polygon(
        &mut self,
        poly: &PolygonRef,
        parent: &Cell,
        _polygon_index: usize,
        trans: &Vec<Matrix3>,
    ) -> bool {
        let points = poly.to_points();
        let bbox = points.bounding_box();
        let offs = poly.repetition_offsets();
        for t in trans {
            for off in &offs {
                let transform = t * off;
                let bbox2 = bbox.apply_transform(&transform);
                if self.has_intersect(&bbox2) {
                    if !(self.f)(points.apply_transform(&transform), bbox2, poly, parent) {
                        return false;
                    }
                }
            }
        }
        true
    }
    fn on_flexpath(
        &mut self,
        flexpath: &FlexPath,
        parent: &Cell,
        _flexpath_index: usize,
        trans: &Vec<Matrix3>,
    ) -> bool {
        for polygon in flexpath.to_polygons() {
            let poly = &polygon.to_ref();
            let points = poly.to_points();
            let bbox = points.bounding_box();
            let offs = poly.repetition_offsets();
            for t in trans {
                for off in &offs {
                    let transform = t * off;
                    let bbox2 = bbox.apply_transform(&transform);
                    if self.has_intersect(&bbox2) {
                        if !(self.f)(points.apply_transform(&transform), bbox2, poly, parent) {
                            return false;
                        }
                    }
                }
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
            let poly = &polygon.to_ref();
            let points = poly.to_points();
            let bbox = points.bounding_box();
            let offs = poly.repetition_offsets();
            for t in trans {
                for off in &offs {
                    let transform = t * off;
                    let bbox2 = bbox.apply_transform(&transform);
                    if self.has_intersect(&bbox2) {
                        if !(self.f)(points.apply_transform(&transform), bbox2, poly, parent) {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
}
impl<'a> GetBoundingBox for Cell<'a> {
    fn bounding_box(&self) -> Rect {
        unsafe {
            let bbox = ffi::gdstk_parse_rs::cell_get_bounding_box(&*self.inner);
            Rect::new(
                Point::new(bbox.min.x, bbox.min.y),
                Point::new(bbox.max.x, bbox.max.y),
            )
        }
    }
}
pub struct RawCell<'a> {
    pub(crate) inner: *const ffi::gdstk::RawCell,
    pub(crate) _marker: std::marker::PhantomData<&'a ffi::gdstk::RawCell>,
}
impl<'a> RawCell<'a> {
    pub fn name(&self) -> String {
        unsafe {
            let name_cxx = ffi::gdstk_parse_rs::rawcell_get_name(&*self.inner);
            name_cxx.to_string_lossy().into_owned()
        }
    }
}
pub struct GeometryCache {
    pub(crate) inner: UniquePtr<ffi::gdstk_parse_rs::GeometryCacheOwner>,
}
impl GeometryCache {
    pub fn new(ptr: UniquePtr<ffi::gdstk_parse_rs::GeometryCacheOwner>) -> Self {
        Self { inner: ptr }
    }
    pub fn bounding_box(&self, name: &str) -> Rect {
        unsafe {
            let cname = CString::new(name).unwrap();
            let bbox =
                ffi::gdstk_parse_rs::geometry_cache_get_bounding_box(&*self.inner, cname.as_ptr());
            Rect::new(
                Point::new(bbox.min.x, bbox.min.y),
                Point::new(bbox.max.x, bbox.max.y),
            )
        }
    }
}
pub struct Library {
    inner: UniquePtr<ffi::gdstk_parse_rs::LibraryOwner>,
}
impl Library {
    pub fn from_oas(path: &str) -> Result<Self, ErrorCode> {
        let c_path = CString::new(path).map_err(|_| ErrorCode::InputFileOpenError)?;
        unsafe {
            let mut error_code = ffi::gdstk::ErrorCode::NoError;
            let ptr =
                ffi::gdstk_parse_rs::library_read_oas(c_path.as_ptr(), 0.0, 1e-9, &mut error_code);
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
                ffi::gdstk_parse_rs::library_read_gds(c_path.as_ptr(), 0.0, 1e-9, &mut error_code);
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
                ffi::gdstk_parse_rs::library_get_top_level(&self.inner).within_unique_ptr();
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
                inner: ffi::gdstk_parse_rs::library_get_cell(&*self.inner, c_name.as_ptr()),
                _marker: std::marker::PhantomData,
            }
        }
    }
    pub fn get_rawcell(&self, name: &str) -> RawCell<'_> {
        let c_name = CString::new(name).unwrap();
        unsafe {
            RawCell {
                inner: ffi::gdstk_parse_rs::library_get_rawcell(&*self.inner, c_name.as_ptr()),
                _marker: std::marker::PhantomData,
            }
        }
    }
    pub fn layername(&self, i: usize) -> LayerName<'_> {
        unsafe {
            LayerName {
                inner: ffi::gdstk_parse_rs::library_get_layername(&*self.inner, i),
                _marker: std::marker::PhantomData,
            }
        }
    }
    pub fn count_layernames(&self) -> usize {
        unsafe { ffi::gdstk_parse_rs::library_count_layernames(&*self.inner) }
    }
    pub fn cell(&self, i: usize) -> Cell<'_> {
        unsafe {
            Cell {
                inner: ffi::gdstk_parse_rs::library_get_cell_by_index(&*self.inner, i),
                _marker: std::marker::PhantomData,
            }
        }
    }
    pub fn count_cells(&self) -> usize {
        unsafe { ffi::gdstk_parse_rs::library_count_cells(&*self.inner) }
    }
    pub fn create_bounding_box_cache(&self) -> BoundingBoxCache<'_> {
        let mut result = BoundingBoxCache {
            map: HashMap::new(),
            _marker: std::marker::PhantomData,
        };
        let cache = unsafe {
            GeometryCache::new(ffi::gdstk_parse_rs::library_create_geometry_cache(
                &*self.inner,
            ))
        };
        for i in 0..self.count_cells() {
            let cell = self.cell(i);
            let bbox = cache.bounding_box(&cell.name());
            result.add(cell.id(), bbox);
        }
        result
    }
}
unsafe impl<'a> Sync for Cell<'a> {}
unsafe impl<'a> Send for Cell<'a> {}

unsafe impl Sync for Library {}
unsafe impl Send for Library {}
