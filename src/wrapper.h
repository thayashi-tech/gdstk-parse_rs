#pragma once
#include <memory>
#include <vector>
#include <utility>
#include <string>
#include <iostream>
#include <cstring>
#include <cstdlib>
#include "gdstk/gdstk.hpp"

namespace gdstk_parse_rs {    
    extern "C" {
        typedef bool (*PointCallback)(double x, double y, void* user_data);
        typedef void (*OffsetCallback)(double x, double y, void* user_data);
    }

    enum LayerIntervalType {
        AllValues,
        UpperBound,
        LowerBound,
        SingleValue,
        Bounded,
    };
    // transfer objects
    struct Point2D {
        double x;
        double y;
    };
    struct BoundingBox {
        Point2D min;
        Point2D max;
    };
    struct LayerInterval {
        LayerIntervalType interval_type;
        uint64_t bound_a;
        uint64_t bound_b;
    };
    struct PolygonSlice {
        uintptr_t data; // referencee for gdstk::Polygon** (avoid pod error)
        size_t count;
    };
    struct PolygonArrayTransfer {
        gdstk::Array<gdstk::Polygon*> data;
        gdstk::ErrorCode ecode;
        inline size_t count() const { return data.count; }
        inline std::unique_ptr<class PolygonOwner> into(size_t i) { 
            auto ptr = std::make_unique<class PolygonOwner>(data[i]);
            data[i] = nullptr;
            return ptr;
        }
        inline gdstk::ErrorCode error_code() const { return ecode; }
        inline void cleanup() {
            for (size_t i = 0; i < data.count; ++i) {
                if (data[i] != nullptr) {
                    data[i]->clear();
                    free_allocation(data[i]);
                    data[i] = nullptr;
                }
            }
            data.clear();
        }
    };
    struct TopLevelResult {
        gdstk::Array<gdstk::Cell*> cells;
        gdstk::Array<gdstk::RawCell*> rawcells;

        inline size_t n_cells() const { return cells.count; }
        inline size_t n_rawcells() const { return rawcells.count; }
        const gdstk::Cell* cell(size_t i) const { 
            assert(i < cells.count);
            return cells[i]; 
        }
        const gdstk::RawCell* rawcell(size_t i) const { 
            assert(i < rawcells.count);
            return rawcells[i]; 
        }
        void cleanup() {
            cells.clear();
            rawcells.clear();
        }
    };
    static char *cstring_dedup(const char *src) {
        assert(src != nullptr);
        size_t len = strlen(src) + 1;
        char *dest = (char*)gdstk::allocate_clear(len);
        if (!dest) {
            throw std::bad_alloc();
        }
        std::memcpy(dest, src, len);
        return dest;
    }
    // library
    struct LibraryOwner {
    public:
        gdstk::Library core;
        LibraryOwner(gdstk::Library ins):core(ins) {}
        LibraryOwner(const char *name, double unit, double precision):core{} {
            core.name = cstring_dedup(name);
            core.unit = unit;
            core.precision = precision;
        }
        ~LibraryOwner() {
            core.free_all();
        }
    };
    inline std::unique_ptr<LibraryOwner> library_new(const char *name, double unit, double precision) {
        return std::make_unique<LibraryOwner>(name, unit, precision);
    }
    inline std::unique_ptr<LibraryOwner> library_read_gds(
        const char *filename, double unit, double tolerance, gdstk::ErrorCode* error_code) {
        return std::make_unique<LibraryOwner>(gdstk::read_gds(filename, unit, tolerance, nullptr, error_code));
    }
    inline std::unique_ptr<LibraryOwner> library_read_oas(
        const char *filename, double unit, double tolerance, gdstk::ErrorCode* error_code) {
        return std::make_unique<LibraryOwner>(gdstk::read_oas(filename, unit, tolerance, error_code));
    }
    inline TopLevelResult library_get_top_level(
        const LibraryOwner& self
    ) {
        TopLevelResult result{};
        self.core.top_level(result.cells, result.rawcells);
        return result;
    }
    inline gdstk::Cell* library_get_cell(
        const LibraryOwner& self,
        const char *name
    ) {
        return self.core.get_cell(name);
    }
    inline gdstk::RawCell* library_get_rawcell(
        const LibraryOwner& self,
        const char *name
    ) {
        return self.core.get_rawcell(name);
    }
    inline double library_get_unit(const LibraryOwner& self) {
        return self.core.unit;
    }
    inline double library_get_precision(const LibraryOwner& self) {
        return self.core.precision;
    }
    inline size_t library_count_layernames(const LibraryOwner& self) {
        return self.core.layer_names.count;
    }
    inline const gdstk::LayerName* library_get_layername(const LibraryOwner& self, size_t i) {
        assert(i < self.core.layer_names.count);
        return &self.core.layer_names[i];
    }
    inline size_t library_count_cells(const LibraryOwner& self) {
        return self.core.cell_array.count;
    }
    inline gdstk::Cell* library_get_cell_by_index(const LibraryOwner& self, size_t i) {
        assert(i < self.core.cell_array.count);
        return self.core.cell_array.items[i];
    }
    inline gdstk::ErrorCode library_write_oas(
        LibraryOwner& self, const char *filename, double circle_tolerance, 
        uint8_t compression_level, uint16_t config_flags) {
        return self.core.write_oas(filename, circle_tolerance, compression_level, config_flags);
    }
    inline gdstk::Cell* library_append_cell(
        LibraryOwner& self,
        const char *name
    ) {
        auto cell = (gdstk::Cell*)gdstk::allocate_clear(sizeof(gdstk::Cell));
        if (!cell) {
            throw std::bad_alloc();
        }
        cell->name = cstring_dedup(name);
        if (!cell->name) {
            gdstk::free_allocation(cell);
            throw std::bad_alloc();
        }
        self.core.cell_array.append(cell);
        return cell;
    }
    inline gdstk::Reference* library_append_reference(LibraryOwner &self, const char *parent, const char *child) {
        assert(parent != nullptr);
        assert(child != nullptr);

        auto parent_cell = library_get_cell(self, parent);
        if (!parent_cell) {
            return nullptr;
        }
        auto child_cell = library_get_cell(self, child);
        if (!child_cell) {
            return nullptr;
        }
        auto reference = (gdstk::Reference*)gdstk::allocate_clear(sizeof(gdstk::Reference));
        if (!reference) {
            throw std::bad_alloc();
        }
        reference->type = gdstk::ReferenceType::Cell;
        reference->cell = child_cell;
        reference->magnification = 1.0;
        parent_cell->reference_array.append(reference);
        return reference;
    }
    // Label
    inline std::string label_get_text(const gdstk::Label& label) {
        if (label.text == nullptr) {
            return "";
        }
        return std::string(label.text);
    }
    inline BoundingBox label_get_bounding_box(const gdstk::Label &label) {
        gdstk::Vec2 min, max;
        label.bounding_box(min, max);
        return {{min.x, min.y}, {max.x, max.y}};
    }
    inline Point2D label_get_position(const gdstk::Label &label) {
        return {label.origin.x, label.origin.y};
    }
    // LayerName
    inline std::string layername_get_name(const gdstk::LayerName& layername) {
        if (layername.name == nullptr) {
            return "";
        }
        return std::string(layername.name);
    }
    inline uint32_t layername_get_layer(const gdstk::LayerName& layername) {
        return layername.layer_interval.bound_a;
    }
    inline uint32_t layername_get_datatype(const gdstk::LayerName& layername) {
        return layername.type_interval.bound_a;
    }
    inline LayerInterval gdstk_layer_interval_to(const gdstk::LayerNameInterval &interval) {
        LayerIntervalType type = LayerIntervalType::AllValues;
        switch (interval.type) {
            case gdstk::OasisInterval::AllValues:
                type = LayerIntervalType::AllValues;
                break;
            case gdstk::OasisInterval::UpperBound:
                type = LayerIntervalType::UpperBound;
                break;
            case gdstk::OasisInterval::LowerBound:
                type = LayerIntervalType::LowerBound;
                break;
            case gdstk::OasisInterval::SingleValue:
                type = LayerIntervalType::SingleValue;
                break;
            case gdstk::OasisInterval::Bounded:
                type = LayerIntervalType::Bounded;
                break;
        }
        return {type, interval.bound_a, interval.bound_b};
    }
    inline LayerInterval layername_get_layer_interval(const gdstk::LayerName& layername) {
        return gdstk_layer_interval_to(layername.layer_interval);
    }
    inline LayerInterval layername_get_datatype_interval(const gdstk::LayerName& layername) {
        return gdstk_layer_interval_to(layername.type_interval);
    }
    // Cell
    inline std::string cell_get_name(const gdstk::Cell& cell) {
        if (cell.name == nullptr) {
            return "";
        }
        return std::string(cell.name);
    }
    inline PolygonArrayTransfer cell_get_polygons(
        const gdstk::Cell &cell,
        bool apply_repetitions, 
        bool include_paths, 
        int64_t depth, 
        bool filter,
        gdstk::Tag tag
    ) {
        PolygonArrayTransfer result = {};
        cell.get_polygons(apply_repetitions, include_paths, depth, filter, tag, result.data);
        return result;
    }
    inline BoundingBox cell_get_bounding_box(const gdstk::Cell &cell) {
        gdstk::Vec2 min, max;
        cell.bounding_box(min, max);
        return {{min.x, min.y}, {max.x, max.y}};
    }
    inline size_t cell_count_polygon_refs(const gdstk::Cell &cell) {
        return cell.polygon_array.count;
    }
    inline gdstk::Polygon *cell_get_polygon_ref(const gdstk::Cell &cell, size_t i) {
        assert(i < cell.polygon_array.count);
        return cell.polygon_array[i];
    }
    inline size_t cell_count_references(const gdstk::Cell &cell) {
        return cell.reference_array.count;
    }
    inline gdstk::Reference *cell_get_reference(const gdstk::Cell &cell, size_t i) {
        assert(i < cell.reference_array.count);
        return cell.reference_array[i];
    }
    inline size_t cell_count_flexpaths(const gdstk::Cell &cell) {
        return cell.flexpath_array.count;
    }
    inline gdstk::FlexPath *cell_get_flexpath(const gdstk::Cell &cell, size_t i) {
        assert(i < cell.flexpath_array.count);
        return cell.flexpath_array[i];
    }
    inline size_t cell_count_robustpaths(const gdstk::Cell &cell) {
        return cell.robustpath_array.count;
    }
    inline gdstk::RobustPath *cell_get_robustpath(const gdstk::Cell &cell, size_t i) {
        assert(i < cell.robustpath_array.count);
        return cell.robustpath_array[i];
    }
    inline size_t cell_count_labels(const gdstk::Cell &cell) {
        return cell.label_array.count;
    }
    inline gdstk::Label *cell_get_label(const gdstk::Cell &cell, size_t i) {
        assert(i < cell.label_array.count);
        return cell.label_array[i];
    }
    // PolygonRef
    inline BoundingBox polygon_ref_get_bounding_box(const gdstk::Polygon &self) {
        gdstk::Vec2 min, max;
        self.bounding_box(min, max);
        return {{min.x, min.y}, {max.x, max.y}};
    }
    inline void polygon_ref_copy(const gdstk::Polygon &src, gdstk::Polygon &dest) {
        dest.copy_from(src);
    }
    inline void polygon_ref_translate(gdstk::Polygon &self, double x, double y) {
        self.translate(gdstk::Vec2{x, y});
    }
    inline void polygon_ref_scale(gdstk::Polygon &self, double scale_x, double scale_y, double center_x, double center_y) {
        self.scale(gdstk::Vec2{scale_x, scale_y}, gdstk::Vec2{center_x, center_y});
    }
    inline void polygon_ref_mirror(gdstk::Polygon &self, double x0, double y0, double x1, double y1) {
        self.mirror(gdstk::Vec2{x0, y0}, gdstk::Vec2{x1, y1});
    }
    inline void polygon_ref_rotate(gdstk::Polygon &self, double angle, double x, double y) {
        self.rotate(angle, gdstk::Vec2{x, y});
    }
    inline uint32_t polygon_ref_layer(const gdstk::Polygon &self) {
        return gdstk::get_layer(self.tag);
    }
    inline uint32_t polygon_ref_datatype(const gdstk::Polygon &self) {
        return gdstk::get_type(self.tag);
    }
    inline bool polygon_ref_foreach_point(
        const gdstk::Polygon &self,
        void* callback_ptr,
        void* user_data
    ) {
        assert(callback_ptr != nullptr);
        auto callback = reinterpret_cast<PointCallback>(callback_ptr);
        for (auto i = 0; i < self.point_array.count; ++i) {
            auto p = self.point_array[i];
            if ( !callback(p.x, p.y, user_data) )
                return false;
        }
        return true;
    }
    inline void repetition_foreach_offset(
        const gdstk::Repetition &self,
        void* callback_ptr,
        void* user_data
    ) {
        assert(callback_ptr != nullptr);
        auto callback = reinterpret_cast<OffsetCallback>(callback_ptr);
        if (self.type == gdstk::RepetitionType::None) {
            callback(0, 0, user_data);
            return;
        }
        gdstk::Array<gdstk::Vec2> offsets = {};
        self.get_offsets(offsets);
        for (uint64_t i = 0; i < offsets.count; ++i) {
            auto p = offsets[i];
            callback(p.x, p.y, user_data);
        }
        offsets.clear();
    }
    struct RectangularRepeats {
        bool enable;
        double dx;
        double dy;
        size_t nx;
        size_t ny;
    };
    inline RectangularRepeats repetition_get_rectangular_repeats(
        const gdstk::Repetition &self) {
        RectangularRepeats results{};
        if (self.type == gdstk::RepetitionType::Rectangular) {
            results.enable = true;
            results.dx = self.spacing.x;
            results.dy = self.spacing.y;
            results.nx = self.columns;
            results.ny = self.rows;
        } else {
            results.enable = false;
        }
        return results;            
    }
    inline size_t repetition_get_count(
        const gdstk::Repetition &self) {
        return self.get_count();
    }
    struct RepetitionExtremaResult {
        gdstk::Array<gdstk::Vec2> data;
        Point2D get(size_t i) const {
            assert(i < data.count);
            return Point2D{data.items[i].x, data.items[i].y};
        }
        size_t count() const {
            return data.count;
        }
    };
    inline RepetitionExtremaResult repetition_get_extrema(
        const gdstk::Repetition &self) {
        RepetitionExtremaResult result{};
        self.get_extrema(result.data);
        return result;
    }
    inline const gdstk::Repetition *polygon_ref_get_repetition(const gdstk::Polygon &self) {
        return &self.repetition;
    }
    inline double polygon_ref_get_signed_area(const gdstk::Polygon &self) {
        return self.signed_area();
    }
    // Polygon
    class PolygonOwner {
        public:
        gdstk::Polygon *core;
        PolygonOwner() {
            core = (gdstk::Polygon*)gdstk::allocate_clear(sizeof(gdstk::Polygon));
            if (!core) {
                throw std::bad_alloc();
            }
        }
        PolygonOwner(gdstk::Polygon *raw) {
            core = raw;
        }
        ~PolygonOwner() {
            if (core) {
                core->clear();
                gdstk::free_allocation(core);
                core = nullptr;
            }
        }
        gdstk::Polygon* poly() { 
            assert(core != nullptr);
            return core;
        }
        const gdstk::Polygon* poly() const { 
            assert(core != nullptr);
            return core;
        }
    };
    inline std::unique_ptr<PolygonOwner> polygon_new() {
        return std::make_unique<PolygonOwner>();
    }
    inline std::unique_ptr<PolygonOwner> polygon_new_from_points(
        const Point2D *points, size_t count, uint32_t layer, uint32_t datatype
    ) {
        auto ptr = std::make_unique<PolygonOwner>();
        ptr->poly()->point_array.ensure_slots(count);
        for (size_t i = 0; i < count; ++i) {
            ptr->poly()->point_array.append(gdstk::Vec2{points[i].x, points[i].y});
        }
        ptr->poly()->tag = gdstk::make_tag(layer, datatype);
        return ptr;
    }
    inline void polygon_set_layer(PolygonOwner &self, uint32_t layer) {
        uint32_t datatype = polygon_ref_datatype(*self.poly());
        self.poly()->tag = gdstk::make_tag(layer, datatype);
    }
    inline void polygon_set_datatype(PolygonOwner &self, uint32_t datatype) {
        uint32_t layer = polygon_ref_layer(*self.poly());
        self.poly()->tag = gdstk::make_tag(layer, datatype);
    }
    inline std::unique_ptr<PolygonOwner> polygon_new_from_ref(const gdstk::Polygon *raw) {
        auto ptr = std::make_unique<PolygonOwner>();
        ptr->poly()->copy_from(*raw);
        return ptr;
    }
    inline void polygon_copy(const PolygonOwner &src, PolygonOwner &dest) {
        polygon_ref_copy(*src.poly(), *dest.poly());
    }
    inline void polygon_translate(PolygonOwner &self, double x, double y) {
        polygon_ref_translate(*self.poly(), x, y);
    }
    inline void polygon_scale(PolygonOwner &self, double scale_x, double scale_y, double center_x, double center_y) {
        polygon_ref_scale(*self.poly(), scale_x, scale_y, center_x, center_y);
    }
    inline void polygon_mirror(PolygonOwner &self, double x0, double y0, double x1, double y1) {
        polygon_ref_mirror(*self.poly(), x0, y0, x1, y1);
    }
    inline void polygon_rotate(PolygonOwner &self, double angle, double x, double y) {
        polygon_ref_rotate(*self.poly(), angle, x, y);
    }
    inline uint32_t polygon_layer(const PolygonOwner &self) {
        return polygon_ref_layer(*self.poly());
    }
    inline uint32_t polygon_datatype(const PolygonOwner &self) {
        return polygon_ref_datatype(*self.poly());
    }
    inline BoundingBox polygon_get_bounding_box(const PolygonOwner &self) {
        return polygon_ref_get_bounding_box(*self.poly());
    }
    inline bool polygon_foreach_point(
        const PolygonOwner &self,
        void* callback_ptr,
        void* user_data
    ) {
        auto poly = self.poly();
        return polygon_ref_foreach_point(*poly, callback_ptr, user_data);
    }
    inline gdstk::Polygon* polygon_to_ref(
        const PolygonOwner &self
    ) {
        return self.core;
    }
    inline double polygon_get_signed_area(const PolygonOwner &self) {
        return (*self.poly()).signed_area();
    }
    inline gdstk::Polygon *polygon_get_boolean_ptr(const PolygonOwner &self) {
        return (gdstk::Polygon*)self.poly();
    }
    inline PolygonArrayTransfer polygon_exec_boolean(
        PolygonSlice a,
        PolygonSlice b,                
        gdstk::Operation op,
        double scaling
    ) {
        gdstk::Array<gdstk::Polygon*> a_poly{};
        gdstk::Array<gdstk::Polygon*> b_poly{};
        
        a_poly.ensure_slots(a.count);
        b_poly.ensure_slots(b.count);
        for (size_t i = 0; i < a.count; ++i) {
            a_poly.append(reinterpret_cast<gdstk::Polygon**>(a.data)[i]);
        }
        for (size_t i = 0; i < b.count; ++i) {
            b_poly.append(reinterpret_cast<gdstk::Polygon**>(b.data)[i]);
        }
        PolygonArrayTransfer results{};
        results.ecode = gdstk::boolean(a_poly, b_poly, op, scaling, results.data);
        a_poly.clear();
        b_poly.clear();
        return results;
    }
    inline void cell_append_polygon(gdstk::Cell *cell, const PolygonOwner &polygon) {
        auto dup = (gdstk::Polygon*)gdstk::allocate_clear(sizeof(gdstk::Polygon));
        if (!dup) {
            throw std::bad_alloc();
        }
        dup->copy_from(*polygon.poly());
        cell->polygon_array.append(dup);
    }
    // FlexPath
    inline PolygonArrayTransfer flexpath_to_polygons(const gdstk::FlexPath &self) {
        PolygonArrayTransfer result{};
        gdstk::FlexPath dup{};
        dup.copy_from(self);
        dup.to_polygons(false, 0, result.data);
        dup.clear();
        return result;
    }
    // RobustPath
    inline PolygonArrayTransfer robustpath_to_polygons(const gdstk::RobustPath &self) {
        PolygonArrayTransfer result{};
        self.to_polygons(false, 0, result.data);
        return result;
    }
    // Reference
    inline gdstk::Cell *reference_get_cell(const gdstk::Reference &self) {
        if (self.type != gdstk::ReferenceType::Cell) {
            return nullptr;
        }
        return self.cell;
    }
    inline Point2D reference_get_translate(const gdstk::Reference &self) {
        return {self.origin.x, self.origin.y};
    }
    inline double reference_get_scale(const gdstk::Reference &self) {
        return self.magnification;
    }
    inline double reference_get_rotation(const gdstk::Reference &self) {
        return self.rotation;
    }
    inline bool reference_get_x_reflection(const gdstk::Reference &self) {
        return self.x_reflection;
    }
    inline const gdstk::Repetition *reference_get_repetition(const gdstk::Reference &self) {
        return &self.repetition;
    }
    // RawCell
    inline std::string rawcell_get_name(const gdstk::RawCell& cell) {
        if (cell.name == nullptr) {
            return "";
        }
        return std::string(cell.name);
    }
    struct GeometryCacheOwner {
    public:
        gdstk::Map<gdstk::GeometryInfo> *core;
        GeometryCacheOwner() {
            core = (gdstk::Map<gdstk::GeometryInfo>*)gdstk::allocate_clear(sizeof(gdstk::Map<gdstk::GeometryInfo>));
            if (!core) {
                throw std::bad_alloc();
            }
        }
        ~GeometryCacheOwner() {
            if (core) {
                for (auto item = core->next(NULL); item; item = core->next(item)) {
                    if (item) {
                        item->value.clear();
                    }
                }
                core->clear();
                gdstk::free_allocation(core);
                core = nullptr;
            }
        }
    };
    inline BoundingBox geometry_cache_get_bounding_box(
        const GeometryCacheOwner& self,
        const char *name
    ) {
        auto info = self.core->get(name);
        auto min = info.bounding_box_min;
        auto max = info.bounding_box_max;
        return {{min.x, min.y}, {max.x, max.y}};
    }
    inline std::unique_ptr<GeometryCacheOwner> library_create_geometry_cache(const LibraryOwner& self) {
        auto ptr = std::make_unique<GeometryCacheOwner>();
        assert(ptr->core != nullptr);
        for (uint64_t i = 0; i < self.core.cell_array.count; i++) {
            auto cell = self.core.cell_array.items[i];
            if (cell) {
                cell->bounding_box(*ptr->core);
            }
        }
        return ptr;
    }
}