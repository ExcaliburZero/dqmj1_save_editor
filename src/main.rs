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
    open_dialog: Controller<OpenDialog>,
    save_dialog: Controller<SaveDialog>,
    message: Option<String>,
    gold_buffer: gtk::EntryBuffer,
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
    SetGold,
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

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 5,
                    set_margin_all: 5,

                    gtk::Label {
                        set_text: "Gold",
                        set_margin_all: 5,
                    },

                    gtk::Entry {
                        #[watch]
                        set_buffer: &model.gold_buffer, //&format!("Gold: {}", model.data_manager.as_ref().map(|m| m.get("gold").get_u32().to_string()).unwrap_or_else(|| "".to_string())),
                        set_margin_all: 5,
                        connect_changed => AppMsg::SetGold,
                    },
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
            data_manager: None,
            open_dialog,
            save_dialog,
            message: None,
            gold_buffer: gtk::EntryBuffer::default(),
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
            AppMsg::Save => {}
            AppMsg::Open => {
                println!("Doing open!");
            }
            AppMsg::OpenRequest => self.open_dialog.emit(OpenDialogMsg::Open),
            AppMsg::OpenResponse(path) => {
                let mut file = File::open(path).unwrap();
                let save_data_manager =
                    SaveDataManager::from_raw_save_data(&RawSaveData::from_sav(&mut file).unwrap());

                self.data_manager = Some(save_data_manager);
                self.gold_buffer.set_text(
                    &self
                        .data_manager
                        .as_ref()
                        .unwrap()
                        .get("gold")
                        .get_u32()
                        .to_string(),
                );
            }
            AppMsg::SetGold => match self.gold_buffer.text().parse::<u32>() {
                Ok(new_value) => {
                    println!("Changing gold: {}", new_value);
                    if let Some(dm) = &mut self.data_manager {
                        dm.set("gold", &DataValue::U32(new_value));
                        println!("Successfully changed gold: {}", new_value);
                    }
                }
                Err(_) => (),
            },
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
