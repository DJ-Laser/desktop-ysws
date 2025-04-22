use egui::{CollapsingHeader, ahash::HashMap};
use todos::{Todo, TodoStatus};

mod todos;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    todos: Vec<Todo>,
    latest_todo_id: u64,

    section_colapse_state: HashMap<TodoStatus, bool>,

    #[serde(skip)] // This how you opt-out of serialization of a field
    new_todo_text: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            todos: Vec::new(),
            latest_todo_id: 0,
            section_colapse_state: Default::default(),
            new_todo_text: "".into(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn todos_ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            self.todos.sort_by_key(|todo| todo.status);

            let mut deleted_ids = Vec::new();

            // Generate ui code from each todo object
            for section_status in [
                TodoStatus::InProgress,
                TodoStatus::NotStarted,
                TodoStatus::Completed,
            ] {
                let collapse_state = self
                    .section_colapse_state
                    .get(&section_status)
                    .copied()
                    .unwrap_or(true);

                let section = CollapsingHeader::new(section_status.to_string())
                    .open(Some(collapse_state))
                    .show(ui, |ui| {
                        let section_todos: Vec<_> = self
                            .todos
                            .iter_mut()
                            .filter(|todo| todo.status == section_status)
                            .collect();

                        if section_todos.is_empty() {
                            ui.label("No todos yet..");
                            return;
                        }

                        for todo in section_todos {
                            if todos::todo_item_ui(ui, todo).deleted {
                                // Keep track of deleted todos
                                deleted_ids.push(todo.id());
                            }
                        }
                    });

                if section.header_response.clicked() {
                    self.section_colapse_state
                        .insert(section_status, !collapse_state);
                }
            }

            // Remove any deleted todos
            self.todos.retain(|todo| !deleted_ids.contains(&todo.id()));
        });
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Todos");

            ui.horizontal(|ui| {
                ui.label("Add Todo: ");

                let input = ui.text_edit_singleline(&mut self.new_todo_text);

                // Check if enter was pressed at the same time as the input lost focus, indicating a submit action
                let input_submitted =
                    input.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));

                if ui.button("+").clicked() || input_submitted {
                    // Add a new todo
                    self.todos
                        .push(Todo::new(self.new_todo_text.clone(), self.latest_todo_id));
                    self.new_todo_text.clear();
                    self.latest_todo_id += 1;
                }
            });

            ui.separator();

            self.todos_ui(ui);

            ui.separator();

            ui.add(egui::github_link_file!(
                "https://github.com/DJ-Laser/desktop-ysws/blob/main/",
                "Source code."
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
