use serde::Deserialize;

#[derive(Deserialize)]
pub struct TodoEventForm {
    /// Todo ID
    todo_id: String,
    /// Notification type
    /// - 1: todo details change
    /// - 2: todo collaborators change
    /// - 3: todo followers change
    /// - 4: todo reminder time changes
    /// - 5: todo completion
    /// - 6: todo cancellation completion
    /// - 7: todo deletion
    obj_type: i32,
}
