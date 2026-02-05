use gpui::{
    div, prelude::*, rgb, Context, IntoElement, MouseButton
};
use crate::db::{client, client::Client, get_connection};

pub fn load_data() -> Vec<Client> {
    if let Ok(conn) = get_connection() {
        if let Ok(data) = client::get_all(&conn) {
            return data;
        }
    }
    Vec::new()
}

pub fn render_list<V: 'static>(
    clients: &[Client],
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
                        .child("Liste des Clients")
                )
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap_2()
                .children(clients.iter().map(|client| {
                    div()
                        .flex()
                        .p_4()
                        .bg(rgb(0x2D2D2D))
                        .rounded_lg()
                        .border_1()
                        .border_color(rgb(0x404040))
                        .child(
                            div().w_1_4().child(format!("{} {}", client.nom_client, client.prenom_client))
                        )
                        .child(
                            div().w_1_4().child(
                                client.raison_social.clone().unwrap_or_else(|| "-".to_string())
                            )
                        )
                        .child(
                            div().w_1_4().child(client.telephone_client.clone())
                        )
                }))
        )
}
