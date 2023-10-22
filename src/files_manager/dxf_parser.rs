use bevy::prelude::*;
use polars::prelude::*;
use crate::files_manager::files_porperties::FileProperties;
use dxf::Drawing;
use dxf::entities::EntityType;

#[derive(Component, Clone)]
pub struct DxfFile{
    pub path: String
}

impl FileProperties for DxfFile{
    fn path(&self) -> String {
        self.path.clone()
    }
}

impl DxfFile {
    pub fn get_points(&self) -> Vec<[f64;3]>{
        let mut _points : Vec<[f64;3]> = Vec::new();
        let path = self.path.clone();
        let drawing = Drawing::load_file(&path).unwrap();
        for e in drawing.entities() {
            match e.specific {
                EntityType::Line(ref _line) => {
                    let p1 = _line.p1.clone();
                    _points.push([p1.x, p1.y, p1.z]);

                    let p2 = _line.p2.clone();
                    _points.push([p2.x, p2.y, p2.z]);
                },
                EntityType::LwPolyline(ref _lw_polyline) => {
                    let vertices = &_lw_polyline.vertices;
                    let z = _lw_polyline.elevation;
                    for point in vertices{
                        _points.push([point.x, point.y, z]);
                    }
                },
                EntityType::Polyline(ref p_line) => {
                    let vertices = p_line.vertices();
                    for ver in vertices{
                        let p = ver.location.clone();
                        _points.push([p.x, p.y, p.z]);
                    }
                },
                _ => (),
            }

        }
        _points
    }

}
