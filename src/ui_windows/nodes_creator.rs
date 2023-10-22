use bevy::prelude::{Assets, Color, StandardMaterial, World, Mesh, PbrBundle, Name};
use bevy_inspector_egui::egui::*;
use bevy_editor_pls_core::editor_window::{EditorWindow, EditorWindowContext, MenuBarWindow};
use crate::custom_meshes::topography_mesh::TopographyMesh;
use crate::files_manager::csv_parser::CsvFile;
use crate::files_manager::dxf_parser::DxfFile;
use crate::files_manager::files_porperties::FileProperties;
use crate::ui_windows::load_drills::LoadDrills;
use crate::ui_windows::scenes::SceneWindow;

#[derive(Default)]
pub struct NodesCreatorState{
    search: String,
    load_node_result: Option<Result<(), Box<dyn std::error::Error + Send + Sync>>>
}

pub struct NodesCreator;

impl EditorWindow for NodesCreator{
    type State = NodesCreatorState;
    const NAME: &'static str = "Create Node";
    const DEFAULT_SIZE: (f32, f32) = (700.0, 500.0);
    const RESIZABLE: bool = false;
    const COLLAPSIBLE: bool = false;
    const MENU_BAR: MenuBarWindow = MenuBarWindow::File;

    fn ui(world: &mut World, mut cx: EditorWindowContext, ui: &mut Ui) {
        make_ui(world, &mut cx, ui);
    }
}


fn make_ui(world: &mut World,
           cx: &mut EditorWindowContext,
           ui: &mut egui::Ui) {

    let mut result: Option<Result<(), Box<dyn std::error::Error + Send + Sync>>> = None;

    ui.horizontal(|ui|{
        egui::ScrollArea::vertical()
            .max_width(200.0)
            .show(ui,|ui|{
                ui.heading("\u{2605} Favourites");
            });
        ui.separator();
        ui.vertical(|ui|{
            ui.separator();
            egui::ScrollArea::vertical()
                .max_width(300.)
                .max_height(500.)
                .min_scrolled_height(500.)
                .min_scrolled_width(300.)
                .show(ui, |ui|{

                    egui::CollapsingHeader::new("\u{2B55} Node")
                        .default_open(true)
                        .show(ui, |ui|{
                            egui::CollapsingHeader::new("\u{1F5FA} Topography Mesh")
                                .default_open(true)
                                .show(ui, |ui|{
                                    if ui.selectable_label(false,"\u{1F5B9} From dxf file").clicked(){

                                        if let Some(path) = rfd::FileDialog::new().add_filter("CAD files (dxf)", &["dxf"]).pick_file() {
                                            let dxf = DxfFile{
                                                path: Some(path.display().to_string()).unwrap()
                                            };
                                            result = Option::from(generate_topography_mesh_from_dxf(&dxf, world));
                                            let state = cx.state_mut::<NodesCreator>().unwrap();
                                            state.load_node_result=result;
                                        }
                                    }

                                    if ui.selectable_label(false ,"\u{1F5B9} From csv file").clicked(){
                                        if let Some(path) = rfd::FileDialog::new().add_filter("CAD files (csv)", &["csv"]).pick_file() {
                                            let csv = CsvFile{
                                                path: Some(path.display().to_string()).unwrap(),
                                                header: true,
                                                sep: b',',
                                            };
                                            result = Option::from(generate_topography_mesh_from_csv(csv, world));
                                            let state = cx.state_mut::<NodesCreator>().unwrap();
                                            state.load_node_result=result;
                                        }
                                    }
                                });
                            if ui.selectable_label(false,"\u{1F4A2} Drill Holes").clicked(){
                                cx.open_floating_window::<LoadDrills>();
                            }
                        });
                });
        });

    });

    let state = cx.state_mut::<NodesCreator>().unwrap();
    if let Some(status) = &state.load_node_result {
        match status {
            Ok(()) => {
                ui.label(egui::RichText::new("Load Success!").color(egui::Color32::GREEN));
            }
            Err(error) => {
                ui.label(egui::RichText::new(error.to_string()).color(egui::Color32::RED));
            }
        }
    }

}



fn generate_topography_mesh_from_dxf(dxf: &DxfFile, world: &mut World) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {


    let _points: Vec<[f64;3]> = dxf.get_points();
    let (topography_mesh, topography) = TopographyMesh::from_points(_points);

    let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
    let mesh = meshes.add(topography_mesh);

    let mut materials = world
        .get_resource_mut::<Assets<StandardMaterial>>()
        .unwrap();
    let material = materials.add(
        StandardMaterial{
            base_color: Color::rgb(135.0/255.0,135.0/255.0,73.0/255.0),
            cull_mode: None,
            ..Default::default()
        }
    );

    world.spawn((PbrBundle {
        mesh,
        material,
        ..Default::default()
    }, topography, dxf.clone(), Name::new(dxf.name().unwrap())));

    Ok(())
}

fn generate_topography_mesh_from_csv(csv: CsvFile, world: &mut World) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (topography_mesh, topography) = TopographyMesh::from_csv(&csv).unwrap();

    let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
    let mesh = meshes.add(topography_mesh);

    let mut materials = world
        .get_resource_mut::<Assets<StandardMaterial>>()
        .unwrap();
    let material = materials.add(
        StandardMaterial{
            base_color: Color::rgb(135.0/255.0,135.0/255.0,73.0/255.0),
            cull_mode: None,
            ..Default::default()
        }
    );

    world.spawn((PbrBundle {
        mesh,
        material,
        ..Default::default()
    }, topography, csv.clone(), Name::new(csv.name().unwrap())));

    Ok(())
}
