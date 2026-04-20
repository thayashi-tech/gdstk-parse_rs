#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(unsafe_op_in_unsafe_fn)]

use autocxx::prelude::*;
use glam;
use std::collections::HashMap;
use std::ffi::{CString, OsStr};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
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
    generate_pod!("gdstk_parse_rs::PolygonSlice")
    generate!("gdstk_parse_rs::PolygonArrayTransfer")
    generate!("gdstk_parse_rs::TopLevelResult")
    generate!("gdstk_parse_rs::PolygonSlice")
    generate_pod!("gdstk_parse_rs::LayerInterval")
    generate_pod!("gdstk_parse_rs::RectangularRepeats")

    // gdstk objects
    generate!("gdstk::Polygon")
    generate!("gdstk::Cell")
    generate!("gdstk::Library")
    generate!("gdstk::Label")
    generate!("gdstk::LayerName")
    generate!("gdstk::ErrorCode")
    generate!("gdstk::Tag")
    generate!("gdstk::make_tag")
    generate!("gdstk::Operation")

    // Library
    generate!("gdstk_parse_rs::LibraryOwner")
    generate!("gdstk_parse_rs::library_new")
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
    generate!("gdstk_parse_rs::library_append_cell")
    generate!("gdstk_parse_rs::library_write_oas")
    generate!("gdstk_parse_rs::library_append_reference")

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
    generate!("gdstk_parse_rs::cell_append_polygon")

    // polygon
    generate!("gdstk_parse_rs::PolygonOwner")
    generate!("gdstk_parse_rs::polygon_new")
    generate!("gdstk_parse_rs::polygon_new_from_ref")
    generate!("gdstk_parse_rs::polygon_new_from_points")
    generate!("gdstk_parse_rs::polygon_copy")
    generate!("gdstk_parse_rs::polygon_translate")
    generate!("gdstk_parse_rs::polygon_scale")
    generate!("gdstk_parse_rs::polygon_mirror")
    generate!("gdstk_parse_rs::polygon_rotate")
    generate!("gdstk_parse_rs::polygon_layer")
    generate!("gdstk_parse_rs::polygon_datatype")
    generate!("gdstk_parse_rs::polygon_set_layer")
    generate!("gdstk_parse_rs::polygon_set_datatype")
    generate!("gdstk_parse_rs::polygon_foreach_point")
    generate!("gdstk_parse_rs::PointCallback")
    generate!("gdstk_parse_rs::polygon_get_bounding_box")
    generate!("gdstk_parse_rs::polygon_to_ref")
    generate!("gdstk_parse_rs::polygon_get_signed_area")
    generate!("gdstk_parse_rs::polygon_exec_boolean")
    generate!("gdstk_parse_rs::polygon_get_boolean_ptr")

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
    generate!("gdstk_parse_rs::repetition_get_rectangular_repeats")

    // rawcell
    generate!("gdstk_parse_rs::rawcell_get_name")

    // GeometryCache
    generate!("gdstk_parse_rs::GeometryCacheOwner")
    generate!("gdstk_parse_rs::geometry_cache_get_bounding_box")
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
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

    // additional
    #[error("missing cell")]
    MissingCell,
    #[error("traverse error")]
    TraverseError(String),
    #[error("traverse abort")]
    TraverseAbort,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct OasisConfig {
    property_max_counts: bool,
    property_top_level: bool,
    property_bounding_box: bool,
    property_cell_offset: bool,
    detect_rectangles: bool,
    detect_trapezoids: bool,
    include_crc32: bool,
    include_checksum32: bool,
}
impl OasisConfig {
    fn to_u16(&self) -> u16 {
        let mut flag: u16 = 0;
        if self.property_max_counts {
            flag |= 0x0001;
        }
        if self.property_top_level {
            flag |= 0x0002;
        }
        if self.property_bounding_box {
            flag |= 0x0004;
        }
        if self.property_cell_offset {
            flag |= 0x0008;
        }
        if self.detect_rectangles {
            flag |= 0x0010;
        }
        if self.detect_trapezoids {
            flag |= 0x0020;
        }
        if self.include_crc32 {
            flag |= 0x0040;
        }
        if self.include_checksum32 {
            flag |= 0x0080;
        }
        flag
    }
    fn set_standard_properties(&mut self) {
        self.property_max_counts = true;
        self.property_top_level = true;
        self.property_bounding_box = true;
        self.property_cell_offset = true;
    }
    fn set_detect_all(&mut self) {
        self.detect_rectangles = true;
        self.detect_trapezoids = true;
    }
    fn new() -> Self {
        let mut config = Self::default();
        config.set_standard_properties();
        config.set_detect_all();
        config
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
    pub fn merge(&mut self, other: &Self) {
        let (min, max) = other.min_max();
        self.min = Point::new(self.min.x.min(min.x), self.min.y.min(min.y));
        self.max = Point::new(self.max.x.max(max.x), self.max.y.max(max.y));
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
    pub fn to_points(&self) -> Vec<Point> {
        let (min, max) = self.min_max();
        vec![min, Point::new(max.x, min.y), max, Point::new(min.x, max.y)]
    }
    pub fn to_array(&self) -> [f64; 4] {
        [self.min.x, self.min.y, self.max.x, self.max.y]
    }
    pub fn from_array(array: [f64; 4]) -> Self {
        Self {
            min: Point::new(array[0], array[1]),
            max: Point::new(array[2], array[3]),
        }
    }
    pub fn width(&self) -> f64 {
        let (min, max) = self.min_max();
        max.x - min.x
    }
    pub fn height(&self) -> f64 {
        let (min, max) = self.min_max();
        max.y - min.y
    }
    pub fn is_valid(&self) -> bool {
        self.width() >= 0.0 && self.height() >= 0.0
    }
    pub fn is_empty(&self) -> bool {
        self.width() <= 0.0 || self.height() <= 0.0
    }
    pub fn and(&self, other: &Rect) -> Self {
        let (smin, smax) = self.min_max();
        let (omin, omax) = other.min_max();
        Self {
            min: Point::new(smin.x.max(omin.x), smin.y.min(omin.y)),
            max: Point::new(smax.x.max(omax.x), smax.y.min(omax.y)),
        }
    }
}
impl ApplyTransform for Rect {
    fn apply_transform(&self, trans: &Matrix3) -> Self {
        if !self.is_valid() {
            return self.clone();
        }
        let (min, max) = self.min_max();
        let p = min.apply_transform(trans);
        let q = max.apply_transform(trans);
        Rect::new(
            Point::new(p.x.min(q.x), p.y.min(q.y)),
            Point::new(p.x.max(q.x), p.y.max(q.y)),
        )
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
pub enum BooleanOperation {
    Or,
    And,
    Xor,
    Not,
}
impl BooleanOperation {
    fn to_ffi(&self) -> ffi::gdstk::Operation {
        match self {
            Self::Or => ffi::gdstk::Operation::Or,
            Self::And => ffi::gdstk::Operation::And,
            Self::Xor => ffi::gdstk::Operation::Xor,
            Self::Not => ffi::gdstk::Operation::Not,
        }
    }
}
impl Polygon {
    pub fn new() -> Self {
        unsafe {
            Self {
                inner: ffi::gdstk_parse_rs::polygon_new(),
            }
        }
    }
    pub fn from_points(points: &Vec<Point>, layer: u32, datatype: u32) -> Self {
        let transfer: Vec<ffi::gdstk_parse_rs::Point2D> = points
            .iter()
            .map(|p| ffi::gdstk_parse_rs::Point2D { x: p.x, y: p.y })
            .collect();
        unsafe {
            Self {
                inner: ffi::gdstk_parse_rs::polygon_new_from_points(
                    transfer.as_ptr(),
                    transfer.len(),
                    layer,
                    datatype,
                ),
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
        unsafe { ffi::gdstk_parse_rs::polygon_get_signed_area(&*self.inner) }
    }
    /// boolean operation (result resolved hole)
    pub fn exec_batch_boolean(
        mut a: Vec<*mut ffi::gdstk::Polygon>,
        mut b: Vec<*mut ffi::gdstk::Polygon>,
        op: BooleanOperation,
    ) -> Result<Vec<Self>, ErrorCode> {
        let a_slice = ffi::gdstk_parse_rs::PolygonSlice {
            data: a.as_mut_ptr() as usize,
            count: a.len(),
        };
        let b_slice = ffi::gdstk_parse_rs::PolygonSlice {
            data: b.as_mut_ptr() as usize,
            count: b.len(),
        };
        let scaling = 1024.0;
        let mut data = unsafe {
            ffi::gdstk_parse_rs::polygon_exec_boolean(a_slice, b_slice, op.to_ffi(), scaling)
                .within_unique_ptr()
        };
        let ecode = data.error_code();
        if ecode != ffi::gdstk::ErrorCode::NoError {
            return Err(ErrorCode::from_ffi(ecode));
        }
        let mut pinned = data.pin_mut();
        let mut polygons = Vec::new();
        for i in 0..pinned.count() {
            polygons.push(Self::from_raw(pinned.as_mut().into(i)));
        }
        pinned.as_mut().cleanup();
        Ok(polygons)
    }
    pub fn set_layer(&mut self, layer: u32) {
        unsafe {
            ffi::gdstk_parse_rs::polygon_set_layer(self.inner.pin_mut(), layer);
        }
    }
    pub fn set_datatype(&mut self, datatype: u32) {
        unsafe {
            ffi::gdstk_parse_rs::polygon_set_datatype(self.inner.pin_mut(), datatype);
        }
    }
    pub fn exec_boolean(
        &self,
        other: &Polygon,
        op: BooleanOperation,
    ) -> Result<Vec<Self>, ErrorCode> {
        let a: Vec<_> = vec![ffi::gdstk_parse_rs::polygon_get_boolean_ptr(&*self.inner)];
        let b: Vec<_> = vec![ffi::gdstk_parse_rs::polygon_get_boolean_ptr(&*other.inner)];
        Self::exec_batch_boolean(a, b, op)
    }
    pub fn clip(&self, area: Rect) -> Result<Vec<Self>, ErrorCode> {
        let clip_points = area.to_points();
        let clip_points = Polygon::from_points(&clip_points, 0, 0);
        self.exec_boolean(&clip_points, BooleanOperation::And)
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
    fn rectangular_repeats(&self) -> Option<(f64, f64, usize, usize)> {
        let results =
            unsafe { ffi::gdstk_parse_rs::repetition_get_rectangular_repeats(&*self.inner) };
        if results.enable {
            Some((results.dx, results.dy, results.nx, results.ny))
        } else {
            None
        }
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
    ) -> Result<(), ErrorCode> {
        if let Some(cell) = self.cell() {
            let trans2 = self.reference_transforms(trans);
            return cell.traverse_shapes_recursive(visitor, trans2);
        }
        Ok(())
    }
    pub fn reference_transforms(&self, trans: &Vec<Matrix3>) -> Vec<Matrix3> {
        let transform = self.transform();
        let offsets: Vec<_> = self
            .repetition()
            .to_offsets()
            .into_iter()
            .map(|v| Matrix3::from_translation(v))
            .collect();
        trans
            .iter()
            .flat_map(|t| offsets.iter().map(move |off| t * off * transform))
            .collect()
    }
    pub fn cell(&self) -> Option<Cell<'_>> {
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
    pub fn transform(&self) -> Matrix3 {
        self.translate() * self.rotation() * self.scale() * self.reflection()
    }
    pub fn translate(&self) -> Matrix3 {
        unsafe {
            let p = ffi::gdstk_parse_rs::reference_get_translate(&*self.inner);
            Matrix3::from_translation(Vector::new(p.x, p.y))
        }
    }
    pub fn rotation(&self) -> Matrix3 {
        unsafe {
            let rad = ffi::gdstk_parse_rs::reference_get_rotation(&*self.inner);
            Matrix3::from_rotation_z(rad)
        }
    }
    pub fn scale(&self) -> Matrix3 {
        unsafe {
            let s = ffi::gdstk_parse_rs::reference_get_scale(&*self.inner);
            Matrix3::from_diagonal(Vector3::new(s, s, 1.0))
        }
    }
    pub fn reflection(&self) -> Matrix3 {
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
    pub fn traverse_shapes<V: ShapeVisitor>(&self, visitor: &mut V) -> Result<(), ErrorCode> {
        let trans = vec![Matrix3::IDENTITY];
        self.traverse_shapes_recursive(visitor, trans)
    }
    pub(crate) fn traverse_shapes_recursive<V: ShapeVisitor>(
        &self,
        visitor: &mut V,
        trans: Vec<Matrix3>,
    ) -> Result<(), ErrorCode> {
        let trans = visitor.on_cell_start(&self, trans)?;
        if trans.is_empty() {
            return Ok(());
        }
        // polygon
        for i in 0..self.count_polygon_refs() {
            let poly = self.polygon_ref(i);
            visitor.on_polygon(&poly, &self, i, &trans)?;
        }
        // flexpath
        for i in 0..self.count_flexpaths() {
            let path = self.flexpath(i);
            visitor.on_flexpath(&path, &self, i, &trans)?;
        }
        // robustpath
        for i in 0..self.count_robustpaths() {
            let path = self.robustpath(i);
            visitor.on_robustpath(&path, &self, i, &trans)?;
        }
        let trans = visitor.on_cell_shape_end(&self, trans)?;
        if !trans.is_empty() {
            for i in 0..self.count_references() {
                self.reference(i)
                    .traverse_shapes_recursive(visitor, &trans)?;
            }
        }
        visitor.on_cell_end(&self, trans)?;
        Ok(())
    }
    pub fn traverse_polygons<F>(&self, mut f: F) -> Result<(), ErrorCode>
    where
        F: FnMut(&PolygonRef, &Cell, &Vec<Matrix3>) -> Result<(), ErrorCode>,
    {
        let mut visitor = CellPolygonVisitor { f };
        self.traverse_shapes(&mut visitor)
    }
    pub fn traverse_polygons_with_overlap<'b, F>(
        &self,
        area: Rect,
        cache: &'b BoundingBoxCache,
        mut f: F,
    ) -> Result<(), ErrorCode>
    where
        F: FnMut(Vec<Point>, Rect, &PolygonRef, &Cell) -> Result<(), ErrorCode>,
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
    ) -> Result<(), ErrorCode>
    where
        F: FnMut(Vec<Point>, Rect, &PolygonRef, &Cell) -> Result<(), ErrorCode>,
    {
        let mut visitor = CellPolygonVisitorWithOverlap {
            f,
            area,
            strictly: true,
            cache,
        };
        self.traverse_shapes(&mut visitor)
    }
    fn as_mut(&self) -> *mut ffi::gdstk::Cell {
        self.inner as *mut _
    }
    pub fn append_polygon(&mut self, polygon: &Polygon) {
        unsafe { ffi::gdstk_parse_rs::cell_append_polygon(self.as_mut(), &*polygon.inner) }
    }
}
pub enum ShapeTaverseStatus {
    Continue,
    Skip,
    Finish,
}
pub trait ShapeVisitor {
    fn on_cell_start(
        &mut self,
        cell: &Cell,
        trans: Vec<Matrix3>,
    ) -> Result<Vec<Matrix3>, ErrorCode> {
        Ok(trans)
    }
    fn on_cell_shape_end(
        &mut self,
        cell: &Cell,
        trans: Vec<Matrix3>,
    ) -> Result<Vec<Matrix3>, ErrorCode> {
        Ok(trans)
    }
    fn on_cell_end(&mut self, cell: &Cell, trans: Vec<Matrix3>) -> Result<(), ErrorCode> {
        Ok(())
    }
    fn on_polygon(
        &mut self,
        poly: &PolygonRef,
        parent: &Cell,
        polygon_index: usize,
        trans: &Vec<Matrix3>,
    ) -> Result<(), ErrorCode> {
        Ok(())
    }
    fn on_flexpath(
        &mut self,
        flexpath: &FlexPath,
        parent: &Cell,
        flexpath_index: usize,
        trans: &Vec<Matrix3>,
    ) -> Result<(), ErrorCode> {
        Ok(())
    }
    fn on_robustpath(
        &mut self,
        robustpath: &RobustPath,
        parent: &Cell,
        robustpath_index: usize,
        trans: &Vec<Matrix3>,
    ) -> Result<(), ErrorCode> {
        Ok(())
    }
}
struct CellPolygonVisitor<F> {
    f: F,
}
impl<F> ShapeVisitor for CellPolygonVisitor<F>
where
    F: FnMut(&PolygonRef, &Cell, &Vec<Matrix3>) -> Result<(), ErrorCode>,
{
    fn on_polygon(
        &mut self,
        poly: &PolygonRef,
        parent: &Cell,
        _polygon_index: usize,
        trans: &Vec<Matrix3>,
    ) -> Result<(), ErrorCode> {
        (self.f)(poly, parent, trans)
    }
    fn on_flexpath(
        &mut self,
        flexpath: &FlexPath,
        parent: &Cell,
        _flexpath_index: usize,
        trans: &Vec<Matrix3>,
    ) -> Result<(), ErrorCode> {
        for polygon in flexpath.to_polygons() {
            (self.f)(&polygon.to_ref(), parent, trans)?;
        }
        Ok(())
    }
    fn on_robustpath(
        &mut self,
        robustpath: &RobustPath,
        parent: &Cell,
        _robustpath_index: usize,
        trans: &Vec<Matrix3>,
    ) -> Result<(), ErrorCode> {
        for polygon in robustpath.to_polygons() {
            (self.f)(&polygon.to_ref(), parent, trans)?;
        }
        Ok(())
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
pub fn filter_overlapped_cells(
    target_area: &Rect,
    is_strictly: bool,
    area: &Rect,
    trans: Vec<Matrix3>,
) -> Vec<Matrix3> {
    trans
        .into_iter()
        .filter_map(|t| {
            let area2 = area.apply_transform(&t);
            let is_intersect = if is_strictly {
                target_area.intersect_strictly(&area2)
            } else {
                target_area.intersect(&area2)
            };
            if is_intersect {
                Some(t)
            } else {
                None
            }
        })
        .collect()
}
pub fn filter_overlapped_polygon<F>(
    mut f: F,
    target_area: &Rect,
    is_strictly: bool,
    poly: &PolygonRef,
    parent: &Cell,
    trans: &Vec<Matrix3>,
) -> Result<(), ErrorCode>
where
    F: FnMut(Vec<Point>, Rect, &PolygonRef, &Cell) -> Result<(), ErrorCode>,
{
    let points = poly.to_points();
    let bbox = points.bounding_box();
    let offs = poly.repetition_offsets();
    for t in trans {
        for off in &offs {
            let transform = t * off;
            let bbox2 = bbox.apply_transform(&transform);
            let is_intersect = if is_strictly {
                target_area.intersect_strictly(&bbox2)
            } else {
                target_area.intersect(&bbox2)
            };
            if is_intersect {
                f(points.apply_transform(&transform), bbox2, poly, parent)?;
            }
        }
    }
    Ok(())
}
pub fn filter_overlapped_flexpath_polygon<F>(
    mut f: F,
    target_area: &Rect,
    is_strictly: bool,
    flexpath: &FlexPath,
    parent: &Cell,
    trans: &Vec<Matrix3>,
) -> Result<(), ErrorCode>
where
    F: FnMut(Vec<Point>, Rect, &PolygonRef, &Cell) -> Result<(), ErrorCode>,
{
    for polygon in flexpath.to_polygons() {
        let poly = &polygon.to_ref();
        let points = poly.to_points();
        let bbox = points.bounding_box();
        let offs = poly.repetition_offsets();
        for t in trans {
            for off in &offs {
                let transform = t * off;
                let bbox2 = bbox.apply_transform(&transform);
                let is_intersect = if is_strictly {
                    target_area.intersect_strictly(&bbox2)
                } else {
                    target_area.intersect(&bbox2)
                };
                if is_intersect {
                    f(points.apply_transform(&transform), bbox2, poly, parent)?;
                }
            }
        }
    }
    Ok(())
}
pub fn filter_overlapped_robustpath_polygon<F>(
    mut f: F,
    target_area: &Rect,
    is_strictly: bool,
    robustpath: &RobustPath,
    parent: &Cell,
    trans: &Vec<Matrix3>,
) -> Result<(), ErrorCode>
where
    F: FnMut(Vec<Point>, Rect, &PolygonRef, &Cell) -> Result<(), ErrorCode>,
{
    for polygon in robustpath.to_polygons() {
        let poly = &polygon.to_ref();
        let points = poly.to_points();
        let bbox = points.bounding_box();
        let offs = poly.repetition_offsets();
        for t in trans {
            for off in &offs {
                let transform = t * off;
                let bbox2 = bbox.apply_transform(&transform);
                let is_intersect = if is_strictly {
                    target_area.intersect_strictly(&bbox2)
                } else {
                    target_area.intersect(&bbox2)
                };
                if is_intersect {
                    f(points.apply_transform(&transform), bbox2, poly, parent)?;
                }
            }
        }
    }
    Ok(())
}
impl<F> ShapeVisitor for CellPolygonVisitorWithOverlap<'_, F>
where
    F: FnMut(Vec<Point>, Rect, &PolygonRef, &Cell) -> Result<(), ErrorCode>,
{
    fn on_cell_start(
        &mut self,
        cell: &Cell,
        trans: Vec<Matrix3>,
    ) -> Result<Vec<Matrix3>, ErrorCode> {
        let area = self
            .cache
            .get(cell.id())
            .ok_or_else(|| ErrorCode::TraverseError(format!("cell not found ({})", cell.id())))?;
        Ok(filter_overlapped_cells(
            &self.area,
            self.strictly,
            &area,
            trans,
        ))
    }
    fn on_polygon(
        &mut self,
        poly: &PolygonRef,
        parent: &Cell,
        _polygon_index: usize,
        trans: &Vec<Matrix3>,
    ) -> Result<(), ErrorCode> {
        filter_overlapped_polygon(
            |a, b, c, d| (self.f)(a, b, c, d),
            &self.area,
            self.strictly,
            poly,
            parent,
            trans,
        )
    }
    fn on_flexpath(
        &mut self,
        flexpath: &FlexPath,
        parent: &Cell,
        _flexpath_index: usize,
        trans: &Vec<Matrix3>,
    ) -> Result<(), ErrorCode> {
        filter_overlapped_flexpath_polygon(
            |a, b, c, d| (self.f)(a, b, c, d),
            &self.area,
            self.strictly,
            flexpath,
            parent,
            trans,
        )
    }
    fn on_robustpath(
        &mut self,
        robustpath: &RobustPath,
        parent: &Cell,
        _robustpath_index: usize,
        trans: &Vec<Matrix3>,
    ) -> Result<(), ErrorCode> {
        filter_overlapped_robustpath_polygon(
            |a, b, c, d| (self.f)(a, b, c, d),
            &self.area,
            self.strictly,
            robustpath,
            parent,
            trans,
        )
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
    pub fn new(name: &str, unit: f64, precision: f64) -> Result<Self, ErrorCode> {
        unsafe {
            let ptr = ffi::gdstk_parse_rs::library_new(
                CString::new(name)
                    .map_err(|_| ErrorCode::InsufficientMemory)?
                    .as_ptr(),
                unit,
                precision,
            );
            if ptr.is_null() {
                Err(ErrorCode::InsufficientMemory)
            } else {
                Ok(Self { inner: ptr })
            }
        }
    }
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
    pub fn append_cell(&mut self, name: &str) -> Cell<'_> {
        let c_name = CString::new(name).unwrap();
        unsafe {
            Cell {
                inner: ffi::gdstk_parse_rs::library_append_cell(
                    self.inner.pin_mut(),
                    c_name.as_ptr(),
                ),
                _marker: std::marker::PhantomData,
            }
        }
    }
    pub fn append_reference(
        &mut self,
        parent: &str,
        child: &str,
    ) -> Result<Reference<'_>, ErrorCode> {
        unsafe {
            let ptr = ffi::gdstk_parse_rs::library_append_reference(
                self.inner.pin_mut(),
                CString::new(parent).unwrap().as_ptr(),
                CString::new(child).unwrap().as_ptr(),
            );
            if ptr.is_null() {
                Err(ErrorCode::MissingCell)
            } else {
                Ok(Reference {
                    inner: ptr,
                    _marker: std::marker::PhantomData,
                })
            }
        }
    }
    pub fn write_oas<P: AsRef<Path>>(
        &mut self,
        path: P,
        compression_level: u8,
    ) -> Result<(), ErrorCode> {
        let circle_tolerance = 0.0;
        let config = OasisConfig::new();
        let path: &OsStr = path.as_ref().as_os_str();
        let filename = CString::new(path.as_bytes()).map_err(|_| ErrorCode::OutputFileOpenError)?;
        let error_code = ErrorCode::from_ffi(unsafe {
            ffi::gdstk_parse_rs::library_write_oas(
                self.inner.pin_mut(),
                filename.as_ptr(),
                circle_tolerance,
                compression_level,
                config.to_u16(),
            )
        });
        if error_code == ErrorCode::NoError {
            Ok(())
        } else {
            Err(error_code)
        }
    }
}
unsafe impl<'a> Sync for Cell<'a> {}
unsafe impl<'a> Send for Cell<'a> {}

unsafe impl Sync for Library {}
unsafe impl Send for Library {}
