use gpui::{div, prelude::*, rgb, Context, IntoElement, MouseButton};
use crate::db::sous_traitant::SousTraitant;

pub fn render_list<V: 'static>(
    sous_traitants: &[SousTraitant],
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
                        .child("Prestataires / Sous-traitants"),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap_2()
                .children(sous_traitants.iter().map(|st| {
                    let st_id = st.id_sous_traitant;
                    div()
                        .flex()
                        .p_4()
                        .bg(rgb(if selected_id == Some(st_id) { 0x1F4F73 } else { 0x2D2D2D }))
                        .rounded_lg()
                        .border_1()
                        .border_color(rgb(0x404040))
                        .cursor_pointer()
                        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| {
                            on_select(this, st_id, cx);
                        }))
                        .child(div().w_1_4().child(format!("#{}", st.id_sous_traitant)))
                        .child(
                            div()
                                .flex_1()
                                .font_weight(gpui::FontWeight::BOLD)
                                .child(st.nom_entreprise.clone()),
                        )
                        .child(
                            div()
                                .w_1_3()
                                .text_color(rgb(0xAAAAAA))
                                .child(st.metier.clone()),
                        )
                        .child(
                            div()
                                .px_2()
                                .py_1()
                                .bg(rgb(0x7C3AED))
                                .rounded_md()
                                .text_sm()
                                .child(format!("{:.0}€/h", st.tarif_horaire)),
                        )
                }))
                .child(if sous_traitants.is_empty() {
                    div()
                        .text_color(rgb(0x777777))
                        .child("Aucun prestataire enregistre.")
                        .into_any_element()
                } else {
                    div().into_any_element()
                }),
        )
}
