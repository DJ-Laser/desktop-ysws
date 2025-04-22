use egui::Response;

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, serde::Deserialize, serde::Serialize,
)]
pub enum TodoStatus {
    NotStarted,
    InProgress,
    Completed,
}

impl std::fmt::Display for TodoStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TodoStatus::NotStarted => write!(f, "Not Started"),
            TodoStatus::InProgress => write!(f, "In Progress"),
            TodoStatus::Completed => write!(f, "Completed"),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Todo {
    pub name: String,
    pub status: TodoStatus,
    id: u64,
}

impl Todo {
    pub fn new(name: String, id: u64) -> Self {
        Todo {
            name,
            id,
            status: TodoStatus::NotStarted,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }
}

pub struct TodoUiResponse {
    pub response: Response,
    pub deleted: bool,
}

pub fn todo_item_ui(ui: &mut egui::Ui, todo: &mut Todo) -> TodoUiResponse {
    let response = ui.vertical(|ui| {
        ui.label(&todo.name);

        ui.horizontal(|ui| {
            let id = todo.id();
            let current_status = &mut todo.status;

            ui.label("Status: ");
            egui::ComboBox::from_id_salt(id)
                .selected_text(current_status.to_string())
                .show_ui(ui, |ui| {
                    ui.radio_value(
                        current_status,
                        TodoStatus::NotStarted,
                        TodoStatus::NotStarted.to_string(),
                    );
                    ui.radio_value(
                        current_status,
                        TodoStatus::InProgress,
                        TodoStatus::InProgress.to_string(),
                    );
                    ui.radio_value(
                        current_status,
                        TodoStatus::Completed,
                        TodoStatus::Completed.to_string(),
                    );
                });

            let delete_button = ui.button("🗑");

            // Don't keep this todo if the trash button was clicked
            delete_button.clicked()
        })
        .inner
        // get the value returned from inside the ui.horizontal closure
    });

    return TodoUiResponse {
        response: response.response,
        deleted: response.inner,
    };
}
