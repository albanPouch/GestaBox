use gpui::{
    div, prelude::*, rgb, Context, IntoElement, MouseButton
};
use crate::db::employee::Employee;

// =======================
// Rendu de la liste
// =======================
pub fn render_list<V: 'static>(
    employees: &[Employee],
    selected_id: Option<i32>,
    cx: &mut Context<V>,
    on_back: impl Fn(&mut V, &mut Context<V>) + 'static + Copy,
    on_select: impl Fn(&mut V, i32, &mut Context<V>) + 'static + Copy,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .flex_1()
        .p_4()
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
                        let employee_id = employee.id_employee;
                        div()
                            .flex()
                            .p_4()
                            .bg(rgb(if selected_id == Some(employee_id) { 0x6B5A11 } else { 0x2D2D2D }))
                            .rounded_lg()
                            .border_1()
                            .border_color(rgb(0x404040))
                            .cursor_pointer()
                            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| {
                                on_select(this, employee_id, cx);
                            }))
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
