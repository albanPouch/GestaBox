use gpui::{
    div, prelude::*, rgb, Context, IntoElement, MouseButton
};
use crate::db::mission::Mission;

pub fn render_list<V: 'static>(
    missions: &[Mission],
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
                        .child("Liste des Missions")
                )
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap_2()
                .children(missions.iter().map(|mission| {
                    let mission_id = mission.id_mission;
                    div()
                        .flex()
                        .p_4()
                        .bg(rgb(if selected_id == Some(mission_id) { 0x1F4F73 } else { 0x2D2D2D }))
                        .rounded_lg()
                        .border_1()
                        .border_color(rgb(0x404040))
                        .cursor_pointer()
                        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| {
                            on_select(this, mission_id, cx);
                        }))
                        .child(
                            div().w_1_4().child(format!("Mission #{}", mission.id_mission))
                        )
                        .child(
                            div().w_1_4().child(
                                if mission.description.len() > 30 {
                                    format!("{}...", &mission.description[..30])
                                } else {
                                    mission.description.clone()
                                }
                            )
                        )
                        .child(
                            div().w_1_4().child(format!("{}h", mission.temps_theorique))
                        )
                }))
        )
}
