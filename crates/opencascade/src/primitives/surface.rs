use crate::primitives::make_point;
use cxx::UniquePtr;
use glam::DVec3;
use opencascade_sys::ffi;

pub enum Surface {
    Plane(Plane),
    Cylinder(Cylinder),
    Sphere(Sphere),
    Cone(Cone),
    Torus(Torus),
    BezierSurface(BezierSurface),
    BSplineSurface(BSplineSurface),
    SurfaceOfExtrusion(SurfaceOfExtrusion),
    SurfaceOfRevolution(SurfaceOfRevolution),
}

pub struct Plane {
    pub(crate) inner: UniquePtr<ffi::HandleGeomPlane>,
}

pub struct Cylinder {
    pub(crate) inner: UniquePtr<ffi::HandleGeom_CylindricalSurface>,
}

pub struct Sphere {
    pub(crate) inner: UniquePtr<ffi::HandleGeom_SphericalSurface>,
}

pub struct Cone {
    pub(crate) inner: UniquePtr<ffi::HandleGeom_ConicalSurface>,
}

pub struct Torus {
    pub(crate) inner: UniquePtr<ffi::HandleGeom_ToroidalSurface>,
}

pub struct BezierSurface {
    pub(crate) inner: UniquePtr<ffi::HandleGeomBezierSurface>,
}

pub struct BSplineSurface {
    pub(crate) inner: UniquePtr<ffi::HandleGeom_BSplineSurface>,
}

pub struct SurfaceOfExtrusion {
    pub(crate) inner: UniquePtr<ffi::HandleGeom_SurfaceOfLinearExtrusion>,
}

pub struct SurfaceOfRevolution {
    pub(crate) inner: UniquePtr<ffi::HandleGeom_SurfaceOfRevolution>,
}

impl Surface {
    pub fn make_plane(_base: DVec3, _normal: DVec3) -> Self {
        todo!()
    }

    pub fn make_cylinder(_center: DVec3, _axis: DVec3, _radius: f64) -> Self {
        todo!()
    }

    pub fn make_sphere(_center: DVec3, _radius: f64) -> Self {
        todo!()
    }

    pub fn make_cone(_apex: DVec3, _axis: DVec3, semiangle: f64) -> Self {
        todo!()
    }

    pub fn make_torus(_base: DVec3, _axis: DVec3, _major_radius: f64, _minor_radius: f64) -> Self {
        todo!()
    }

    pub fn make_bezier(poles: impl IntoIterator<Item = impl IntoIterator<Item = DVec3>>) -> Self {
        let poles: Vec<Vec<_>> =
            poles.into_iter().map(|poles| poles.into_iter().collect()).collect();

        let mut pole_array = ffi::TColgp_Array2OfPnt_ctor(
            0,
            poles.len() as i32 - 1,
            0,
            poles.first().map(|first| first.len()).unwrap_or(0) as i32 - 1,
        );

        for (row, poles) in poles.iter().enumerate() {
            for (column, pole) in poles.iter().enumerate() {
                let pole = &make_point(*pole);
                pole_array.pin_mut().SetValue(row as i32, column as i32, pole);
            }
        }

        let bezier = ffi::Geom_BezierSurface_ctor(&pole_array);
        // let inner = ffi::bezier_to_surface(&bezier);

        Self::BezierSurface(BezierSurface { inner: bezier })
    }

    pub fn make_bspline() -> Self {
        todo!()
    }

    pub fn make_revolution() -> Self {
        todo!()
    }

    pub fn make_extrusion() -> Self {
        todo!()
    }
}
