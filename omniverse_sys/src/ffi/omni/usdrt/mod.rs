use crate::usdrt;
use cpp::cpp;

crate::macros::impl_vector_element!(
    [Opaque]
    "usdrt$UsdPrim",
    "UsdPrim",
    usdrt::UsdPrim
);

crate::macros::impl_vector_element!(
    [Opaque]
    "usdrt$UsdAttribute",
    "UsdAttribute",
    usdrt::UsdAttribute
);

crate::macros::impl_vector_element!(
    [Opaque]
    "usdrt$UsdRelationship",
    "UsdRelationship",
    usdrt::UsdRelationship
);

crate::macros::impl_vector_element!(
    [Opaque]
    "usdrt$SdfPath",
    "SdfPath",
    usdrt::SdfPath
);

impl usdrt::UsdAttribute {
    pub fn set_i64(self: std::pin::Pin<&mut usdrt::UsdAttribute>, value: i64) {
        unsafe {
            cpp!([self as "usdrt::UsdAttribute *", value as "int64_t"] {
                return self->Set(value);
            })
        }
    }
    pub fn set_i32(self: std::pin::Pin<&mut usdrt::UsdAttribute>, value: i32) {
        unsafe {
            cpp!([self as "usdrt::UsdAttribute *", value as "int32_t"] {
                return self->Set(value);
            })
        }
    }
}
