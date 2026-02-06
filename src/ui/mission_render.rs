use gpui::{
    div, prelude::*, rgb, Context, IntoElement, MouseButton
};
use crate::db::{mission, mission::Mission, get_connection};

pub fn load_data() -> Vec<Mission> {
    if let Ok(conn) = get_connection() {
        if let Ok(data) = mission::get_all(&conn) {
            return data;
        }
    }

    Vec::new()
}

pub fn render_list<V: 'static>(
    missions: &[Mission],
    cx: &mut Context<V>,
    on_back: impl Fn(&mut V, &mut Context<V>) + 'static + Copy,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .size_full()
        .p_8()
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
                    div()
                        .flex()
                        .p_4()
                        .bg(rgb(0x2D2D2D))
                        .rounded_lg()
                        .border_1()
                        .border_color(rgb(0x404040))
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
