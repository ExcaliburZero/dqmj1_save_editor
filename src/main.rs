use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt};
use relm4::actions::{AccelsPlus, ActionablePlus, RelmAction, RelmActionGroup};
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};
use relm4_components::{open_dialog::*, save_dialog::*};

use gtk::prelude::*;
use relm4::prelude::*;

use std::path::PathBuf;

struct AppModel {
    pub gold: u32,
    open_dialog: Controller<OpenDialog>,
}

#[derive(Debug)]
enum AppMsg {
    Save,
    Open,
    OpenRequest,
    OpenResponse(PathBuf),
    SaveRequest,
    SaveResponse(PathBuf),
    Ignore
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
                    //#[watch]
                    //set_label: &format!("Gold: {}", model.gold),
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
        // Insert the macro code generation here
        let widgets = view_output!();

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

        let app = relm4::main_application();

        app.set_accelerators_for_action::<SaveAction>(&["<primary>s"]);
        let open_action: RelmAction<OpenAction> = {
            RelmAction::new_stateless(move |_| {
                println!("Open!");
                sender.input(AppMsg::Open);
            })
        };

        widgets.main_window.set_show_menubar(true);
        let mut group = RelmActionGroup::<WindowActionGroup>::new();
        group.add_action(open_action);
        group.register_for_widget(&widgets.main_window);

        let file_chooser = gtk::FileChooserNative::builder().transient_for(root);

        app.set_menubar(Some(&main_menu));

        let model = AppModel {
            gold,
            open_dialog,
        };
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
            AppMsg::OpenResponse(path) => match std::fs::read_to_string(&path) {
                Ok(contents) => {
                    self.buffer.set_text(&contents);
                    self.file_name = Some(
                        path.file_name()
                            .expect("The path has no file name")
                            .to_str()
                            .expect("Cannot convert file name to string")
                            .to_string(),
                    );
                }
                Err(e) => sender.input(AppMsg::ShowMessage(e.to_string())),
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
