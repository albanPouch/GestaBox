use gpui::{
    div, prelude::*, rgb, Context, IntoElement, MouseButton
};
use crate::db::{employee, employee::Employee, get_connection};

// =======================
// Chargement des données
// =======================
pub fn load_data() -> Vec<Employee> {
    if let Ok(conn) = get_connection() {
        if let Ok(data) = employee::get_all(&conn) {
            return data;
        }
    }
    Vec::new()
}

// =======================
// Rendu de la liste
// =======================
pub fn render_list<V: 'static>(
    employees: &[Employee],
    cx: &mut Context<V>,
    on_back: impl Fn(&mut V, &mut Context<V>) + 'static + Copy,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .size_full()
        .p_8()
        // ===== Header =====
        .child(
            div()
                .flex()
                .items_center()
                .mb_4()
                .child(
                    div()
                        .p_2()
                        .mr_4()
                        .bg(rgb(0x404040))
                        .rounded_md()
                        .cursor_pointer()
                        .child("Retour")
                        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| {
                            on_back(this, cx);
                        })),
                )
                .child(
                    div()
                        .text_2xl()
                        .font_weight(gpui::FontWeight::BOLD)
                        .child("Liste des Employés")
                )
        )
        // ===== Liste =====
        .child(
            div()
                .flex()
                .flex_col()
                .gap_2()
                .children(
                    employees.iter().map(|employee| {
                        div()
                            .flex()
                            .p_4()
                            .bg(rgb(0x2D2D2D))
                            .rounded_lg()
                            .border_1()
                            .border_color(rgb(0x404040))
                            .child(
                                div()
                                    .w_1_4()
                                    .child(format!(
                                        "Employé #{} | Catégorie {}",
                                        employee.id_employee,
                                        employee.id_category
                                    ))
                            )
                    })
                )
        )
}
