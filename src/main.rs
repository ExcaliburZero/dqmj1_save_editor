use dqmj1_save_editor::data_fields::DataValue;
use dqmj1_save_editor::raw_save_data::RawSaveData;
use dqmj1_save_editor::save_data_manager::SaveDataManager;
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt};
use relm4::actions::{AccelsPlus, ActionablePlus, RelmAction, RelmActionGroup};
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};
use relm4_components::{open_dialog::*, save_dialog::*};

use gtk::prelude::*;
use relm4::prelude::*;

use std::fs::File;
use std::path::PathBuf;

struct AppModel {
    data_manager: Option<SaveDataManager>,
    gold: u32,
    open_dialog: Controller<OpenDialog>,
    save_dialog: Controller<SaveDialog>,
    message: Option<String>,
}

#[derive(Debug)]
enum AppMsg {
    Save,
    Open,
    OpenRequest,
    OpenResponse(PathBuf),
    SaveRequest,
    SaveResponse(PathBuf),
    ShowMessage(String),
    Ignore,
}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Init = u32;

    type Input = AppMsg;
    type Output = ();

    view! {
        main_window = gtk::ApplicationWindow {
            set_title: Some("Simple app"),
            set_default_width: 300,
            set_default_height: 100,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,

                gtk::Label {
                    #[watch]
                    set_label: &format!("Gold: {}", model.gold),
                    set_margin_all: 5,
                },

                gtk::Button::with_label("Save") {
                    connect_clicked[sender] => move |_| {
                        sender.input(AppMsg::Save);
                    }
                }
            }
        },
    }

    // Initialize the UI.
    fn init(
        gold: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        relm4::menu! {
            main_menu: {
                "File" {
                    "Open" => OpenAction,
                    "Save" => SaveAction,
                }
            }
        };

        let open_dialog = OpenDialog::builder()
            .transient_for_native(&root)
            .launch(OpenDialogSettings::default())
            .forward(sender.input_sender(), |response| match response {
                OpenDialogResponse::Accept(path) => AppMsg::OpenResponse(path),
                OpenDialogResponse::Cancel => AppMsg::Ignore,
            });

        let save_dialog = SaveDialog::builder()
            .transient_for_native(&root)
            .launch(SaveDialogSettings::default())
            .forward(sender.input_sender(), |response| match response {
                SaveDialogResponse::Accept(path) => AppMsg::SaveResponse(path),
                SaveDialogResponse::Cancel => AppMsg::Ignore,
            });

        let app = relm4::main_application();

        app.set_accelerators_for_action::<OpenAction>(&["<primary>o"]);
        let open_sender = sender.clone();
        let open_action: RelmAction<OpenAction> = {
            RelmAction::new_stateless(move |_| {
                println!("Open!");
                open_sender.input(AppMsg::OpenRequest);
            })
        };

        app.set_menubar(Some(&main_menu));

        let model = AppModel {
            gold,
            data_manager: None,
            open_dialog,
            save_dialog,
            message: None,
        };

        // Note: view_output!() has to below the model initialization, otherwise references to
        // model in the GUI elements will produce the following error
        // error[E0425]: cannot find value `model` in this scope
        let widgets = view_output!();
        widgets.main_window.set_show_menubar(true);
        let mut group = RelmActionGroup::<WindowActionGroup>::new();
        group.add_action(open_action);
        group.register_for_widget(&widgets.main_window);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            AppMsg::Save => {
                self.gold = self.gold.wrapping_add(1);
            }
            AppMsg::Open => {
                println!("Doing open!");
            }
            AppMsg::OpenRequest => self.open_dialog.emit(OpenDialogMsg::Open),
            AppMsg::OpenResponse(path) => {
                let mut file = File::open(path).unwrap();
                let save_data_manager =
                    SaveDataManager::from_raw_save_data(&RawSaveData::from_sav(&mut file).unwrap());

                self.data_manager = Some(save_data_manager);
                self.gold = match self.data_manager.as_ref().unwrap().get("gold") {
                    DataValue::U32(v) => v,
                    _ => panic!(),
                };
            }
            _ => (),
        }
    }
}

relm4::new_action_group!(WindowActionGroup, "win");

relm4::new_stateless_action!(OpenAction, WindowActionGroup, "open");
relm4::new_stateless_action!(SaveAction, WindowActionGroup, "save");

fn main() {
    let app = RelmApp::new("relm4.test.simple");
    app.run::<AppModel>(0);
}
