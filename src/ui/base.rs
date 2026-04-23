use gpui::{
    App, Application, Bounds, Context, Entity, IntoElement, MouseButton, Window, WindowBounds,
    WindowOptions, div, prelude::*, px, rgb, size,
};

use crate::db::{
    client, client::Client,
    employee, employee::Employee,
    mission, mission::{Mission, CandidatIntervenant, EmploiDuTempsLigne},
    sous_traitant, sous_traitant::SousTraitant,
};

use super::text_input::TextInput;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ActiveView {
    Home,
    ClientList,
    MissionList,
    EmployeeList,
    PlanningView,
    SousTraitantList,
    AffectationView,
}

struct ClientForm {
    selected_id: Option<i32>,
    nom: Entity<TextInput>,
    prenom: Entity<TextInput>,
    raison_social: Entity<TextInput>,
    telephone: Entity<TextInput>,
    message: String,
}

struct MissionForm {
    selected_id: Option<i32>,
    temps_theorique: Entity<TextInput>,
    description: Entity<TextInput>,
    date_debut: Entity<TextInput>,
    date_fin: Entity<TextInput>,
    ville_mission: Entity<TextInput>,
    departement_mission: Entity<TextInput>,
    id_status: i32,
    id_intervenant: i32,
    message: String,
}

struct EmployeeForm {
    selected_id: Option<i32>,
    id_category: Entity<TextInput>,
    message: String,
}

struct SousTraitantForm {
    selected_id: Option<i32>,
    nom_entreprise: Entity<TextInput>,
    metier: Entity<TextInput>,
    tarif_horaire: Entity<TextInput>,
    message: String,
}

struct GestaBoxApp {
    active_view: ActiveView,
    clients: Vec<Client>,
    missions: Vec<Mission>,
    employees: Vec<Employee>,
    sous_traitants: Vec<SousTraitant>,
    emploi_du_temps: Vec<EmploiDuTempsLigne>,
    candidats_affectation: Vec<CandidatIntervenant>,
    mission_en_affectation_id: Option<i32>,
    affectation_message: String,
    planning_filtre: Entity<TextInput>,
    client_form: ClientForm,
    mission_form: MissionForm,
    employee_form: EmployeeForm,
    sous_traitant_form: SousTraitantForm,
}

impl GestaBoxApp {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            active_view: ActiveView::Home,
            clients: Vec::new(),
            missions: Vec::new(),
            employees: Vec::new(),
            sous_traitants: Vec::new(),
            emploi_du_temps: Vec::new(),
            candidats_affectation: Vec::new(),
            mission_en_affectation_id: None,
            affectation_message: String::new(),
            planning_filtre: cx.new(|cx| TextInput::new(cx, "Filtrer par nom...")),
            client_form: ClientForm {
                selected_id: None,
                nom: cx.new(|cx| TextInput::new(cx, "Nom")),
                prenom: cx.new(|cx| TextInput::new(cx, "Prenom")),
                raison_social: cx.new(|cx| TextInput::new(cx, "Raison sociale")),
                telephone: cx.new(|cx| TextInput::new(cx, "Telephone")),
                message: String::new(),
            },
            mission_form: MissionForm {
                selected_id: None,
                temps_theorique: cx.new(|cx| TextInput::new(cx, "Temps theorique")),
                description: cx.new(|cx| TextInput::new(cx, "Description")),
                date_debut: cx.new(|cx| TextInput::new(cx, "Date debut YYYY-MM-DD")),
                date_fin: cx.new(|cx| TextInput::new(cx, "Date fin YYYY-MM-DD")),
                ville_mission: cx.new(|cx| TextInput::new(cx, "Ville")),
                departement_mission: cx.new(|cx| TextInput::new(cx, "Departement")),
                id_status: 1,
                id_intervenant: 1,
                message: String::new(),
            },
            employee_form: EmployeeForm {
                selected_id: None,
                id_category: cx.new(|cx| TextInput::new(cx, "Id categorie")),
                message: String::new(),
            },
            sous_traitant_form: SousTraitantForm {
                selected_id: None,
                nom_entreprise: cx.new(|cx| TextInput::new(cx, "Nom entreprise")),
                metier: cx.new(|cx| TextInput::new(cx, "Metier (ex: Developpement Web)")),
                tarif_horaire: cx.new(|cx| TextInput::new(cx, "Tarif horaire (ex: 75.0)")),
                message: String::new(),
            },
        }
    }

    fn switch_view(&mut self, view: ActiveView, cx: &mut Context<Self>) {
        self.active_view = view;
        self.reload_current_view(cx);
        cx.notify();
    }

    fn reload_current_view(&mut self, cx: &mut Context<Self>) {
        match self.active_view {
            ActiveView::ClientList => self.reload_clients(cx),
            ActiveView::MissionList => self.reload_missions(cx),
            ActiveView::EmployeeList => self.reload_employees(cx),
            ActiveView::SousTraitantList => self.reload_sous_traitants(cx),
            ActiveView::PlanningView => self.reload_planning(cx),
            ActiveView::AffectationView => {}
            ActiveView::Home => {}
        }
    }

    fn reload_clients(&mut self, _: &mut Context<Self>) {
        self.clients = crate::db::get_connection()
            .and_then(|conn| client::get_all(&conn))
            .unwrap_or_default();
    }

    fn reload_missions(&mut self, _: &mut Context<Self>) {
        self.missions = crate::db::get_connection()
            .and_then(|conn| mission::get_all(&conn))
            .unwrap_or_default();
    }

    fn reload_employees(&mut self, _: &mut Context<Self>) {
        self.employees = crate::db::get_connection()
            .and_then(|conn| employee::get_all(&conn))
            .unwrap_or_default();
    }

    fn reload_sous_traitants(&mut self, _: &mut Context<Self>) {
        self.sous_traitants = crate::db::get_connection()
            .and_then(|conn| sous_traitant::get_all(&conn))
            .unwrap_or_default();
    }

    fn reload_planning(&mut self, _: &mut Context<Self>) {
        self.emploi_du_temps = crate::db::get_connection()
            .and_then(|conn| mission::get_emploi_du_temps(&conn))
            .unwrap_or_default();
    }

    fn set_input(input: &Entity<TextInput>, cx: &mut Context<Self>, value: impl Into<String>) {
        let value = value.into();
        input.update(cx, |input, _cx| input.set_value(value));
    }

    fn read_input(input: &Entity<TextInput>, cx: &mut Context<Self>) -> String {
        input.read(cx).value()
    }

    fn next_client_id(&self) -> i32 {
        self.clients.iter().map(|c| c.id_client).max().unwrap_or(0) + 1
    }

    fn next_mission_id(&self) -> i32 {
        self.missions.iter().map(|m| m.id_mission).max().unwrap_or(0) + 1
    }

    fn next_employee_id(&self) -> i32 {
        self.employees.iter().map(|e| e.id_employee).max().unwrap_or(0) + 1
    }

    fn next_sous_traitant_id(&self) -> i32 {
        self.sous_traitants
            .iter()
            .map(|s| s.id_sous_traitant)
            .max()
            .unwrap_or(0)
            + 1
    }

    fn clear_client_form(&mut self, cx: &mut Context<Self>) {
        self.client_form.selected_id = None;
        self.client_form.message.clear();
        Self::set_input(&self.client_form.nom, cx, "");
        Self::set_input(&self.client_form.prenom, cx, "");
        Self::set_input(&self.client_form.raison_social, cx, "");
        Self::set_input(&self.client_form.telephone, cx, "");
    }

    fn clear_mission_form(&mut self, cx: &mut Context<Self>) {
        self.mission_form.selected_id = None;
        self.mission_form.message.clear();
        Self::set_input(&self.mission_form.temps_theorique, cx, "");
        Self::set_input(&self.mission_form.description, cx, "");
        Self::set_input(&self.mission_form.date_debut, cx, "");
        Self::set_input(&self.mission_form.date_fin, cx, "");
        Self::set_input(&self.mission_form.ville_mission, cx, "");
        Self::set_input(&self.mission_form.departement_mission, cx, "");
        self.mission_form.id_status = 1;
        self.mission_form.id_intervenant = 1;
    }

    fn clear_employee_form(&mut self, cx: &mut Context<Self>) {
        self.employee_form.selected_id = None;
        self.employee_form.message.clear();
        Self::set_input(&self.employee_form.id_category, cx, "");
    }

    fn clear_sous_traitant_form(&mut self, cx: &mut Context<Self>) {
        self.sous_traitant_form.selected_id = None;
        self.sous_traitant_form.message.clear();
        Self::set_input(&self.sous_traitant_form.nom_entreprise, cx, "");
        Self::set_input(&self.sous_traitant_form.metier, cx, "");
        Self::set_input(&self.sous_traitant_form.tarif_horaire, cx, "");
    }

    fn select_client(&mut self, id: i32, cx: &mut Context<Self>) {
        if let Some(c) = self.clients.iter().find(|c| c.id_client == id) {
            self.client_form.selected_id = Some(id);
            self.client_form.message.clear();
            Self::set_input(&self.client_form.nom, cx, c.nom_client.clone());
            Self::set_input(&self.client_form.prenom, cx, c.prenom_client.clone());
            Self::set_input(
                &self.client_form.raison_social,
                cx,
                c.raison_social.clone().unwrap_or_default(),
            );
            Self::set_input(&self.client_form.telephone, cx, c.telephone_client.clone());
        }
    }

    fn select_mission(&mut self, id: i32, cx: &mut Context<Self>) {
        if let Some(m) = self.missions.iter().find(|m| m.id_mission == id) {
            self.mission_form.selected_id = Some(id);
            self.mission_form.message.clear();
            Self::set_input(&self.mission_form.temps_theorique, cx, m.temps_theorique.clone());
            Self::set_input(&self.mission_form.description, cx, m.description.clone());
            Self::set_input(&self.mission_form.date_debut, cx, m.date_debut.clone());
            Self::set_input(&self.mission_form.date_fin, cx, m.date_fin.clone());
            Self::set_input(&self.mission_form.ville_mission, cx, m.ville_mission.clone());
            Self::set_input(
                &self.mission_form.departement_mission,
                cx,
                m.departement_mission.clone(),
            );
            self.mission_form.id_status = m.id_status;
            self.mission_form.id_intervenant = m.id_intervenant;
        }
    }

    fn select_employee(&mut self, id: i32, cx: &mut Context<Self>) {
        if let Some(e) = self.employees.iter().find(|e| e.id_employee == id) {
            self.employee_form.selected_id = Some(id);
            self.employee_form.message.clear();
            Self::set_input(&self.employee_form.id_category, cx, e.id_category.to_string());
        }
    }

    fn select_sous_traitant(&mut self, id: i32, cx: &mut Context<Self>) {
        if let Some(s) = self.sous_traitants.iter().find(|s| s.id_sous_traitant == id) {
            self.sous_traitant_form.selected_id = Some(id);
            self.sous_traitant_form.message.clear();
            Self::set_input(&self.sous_traitant_form.nom_entreprise, cx, s.nom_entreprise.clone());
            Self::set_input(&self.sous_traitant_form.metier, cx, s.metier.clone());
            Self::set_input(
                &self.sous_traitant_form.tarif_horaire,
                cx,
                format!("{:.2}", s.tarif_horaire),
            );
        }
    }

    fn today_iso() -> String {
        "2026-04-23".to_string()
    }

    // ── CRUD Clients ────────────────────────────────────────────────────────

    fn save_client(&mut self, _: &gpui::MouseDownEvent, _: &mut Window, cx: &mut Context<Self>) {
        let nom = Self::read_input(&self.client_form.nom, cx);
        let prenom = Self::read_input(&self.client_form.prenom, cx);
        let raison_social_raw = Self::read_input(&self.client_form.raison_social, cx);
        let telephone = Self::read_input(&self.client_form.telephone, cx);

        if nom.trim().is_empty() || prenom.trim().is_empty() || telephone.trim().is_empty() {
            self.client_form.message = "Nom, prenom et telephone sont obligatoires.".into();
            cx.notify();
            return;
        }

        let raison_social = if raison_social_raw.trim().is_empty() {
            None
        } else {
            Some(raison_social_raw.trim())
        };

        let result = if let Ok(conn) = crate::db::get_connection() {
            if let Some(id) = self.client_form.selected_id {
                client::update(&conn, id, nom.trim(), prenom.trim(), raison_social, telephone.trim())
            } else {
                client::insert(
                    &conn,
                    self.next_client_id(),
                    nom.trim(),
                    prenom.trim(),
                    raison_social,
                    telephone.trim(),
                )
            }
        } else {
            self.client_form.message = "Connexion DB impossible.".into();
            cx.notify();
            return;
        };

        match result {
            Ok(_) => {
                self.reload_clients(cx);
                self.client_form.message = "Client enregistre.".into();
                self.clear_client_form(cx);
            }
            Err(err) => self.client_form.message = format!("Erreur: {err}"),
        }
        cx.notify();
    }

    fn delete_client(&mut self, _: &gpui::MouseDownEvent, _: &mut Window, cx: &mut Context<Self>) {
        let Some(id) = self.client_form.selected_id else {
            self.client_form.message = "Selectionne un client.".into();
            cx.notify();
            return;
        };
        match crate::db::get_connection().and_then(|conn| client::delete(&conn, id)) {
            Ok(_) => {
                self.reload_clients(cx);
                self.client_form.message = "Client supprime.".into();
                self.clear_client_form(cx);
            }
            Err(err) => self.client_form.message = format!("Erreur: {err}"),
        }
        cx.notify();
    }

    // ── CRUD Employees ───────────────────────────────────────────────────────

    fn save_employee(&mut self, _: &gpui::MouseDownEvent, _: &mut Window, cx: &mut Context<Self>) {
        let id_category_raw = Self::read_input(&self.employee_form.id_category, cx);
        let Ok(id_category) = id_category_raw.trim().parse::<i32>() else {
            self.employee_form.message = "Id categorie invalide.".into();
            cx.notify();
            return;
        };

        let result = if let Ok(conn) = crate::db::get_connection() {
            if let Some(id) = self.employee_form.selected_id {
                employee::update(&conn, id, id_category)
            } else {
                employee::insert(&conn, self.next_employee_id(), id_category)
            }
        } else {
            self.employee_form.message = "Connexion DB impossible.".into();
            cx.notify();
            return;
        };

        match result {
            Ok(_) => {
                self.reload_employees(cx);
                self.employee_form.message = "Employe enregistre.".into();
                self.clear_employee_form(cx);
            }
            Err(err) => self.employee_form.message = format!("Erreur: {err}"),
        }
        cx.notify();
    }

    fn delete_employee(
        &mut self,
        _: &gpui::MouseDownEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(id) = self.employee_form.selected_id else {
            self.employee_form.message = "Selectionne un employe.".into();
            cx.notify();
            return;
        };
        match crate::db::get_connection().and_then(|conn| employee::delete(&conn, id)) {
            Ok(_) => {
                self.reload_employees(cx);
                self.employee_form.message = "Employe supprime.".into();
                self.clear_employee_form(cx);
            }
            Err(err) => self.employee_form.message = format!("Erreur: {err}"),
        }
        cx.notify();
    }

    // ── CRUD Missions ────────────────────────────────────────────────────────

    fn save_mission(&mut self, _: &gpui::MouseDownEvent, _: &mut Window, cx: &mut Context<Self>) {
        let temps_theorique = Self::read_input(&self.mission_form.temps_theorique, cx);
        let description = Self::read_input(&self.mission_form.description, cx);
        let date_debut = Self::read_input(&self.mission_form.date_debut, cx);
        let date_fin = Self::read_input(&self.mission_form.date_fin, cx);
        let ville_mission = Self::read_input(&self.mission_form.ville_mission, cx);
        let departement_mission = Self::read_input(&self.mission_form.departement_mission, cx);
        let date_creation = self
            .mission_form
            .selected_id
            .and_then(|id| {
                self.missions
                    .iter()
                    .find(|m| m.id_mission == id)
                    .map(|m| m.date_creation.clone())
            })
            .unwrap_or_else(Self::today_iso);
        let derniere_modif = Self::today_iso();

        if temps_theorique.trim().is_empty()
            || description.trim().is_empty()
            || date_debut.trim().is_empty()
            || date_fin.trim().is_empty()
            || ville_mission.trim().is_empty()
            || departement_mission.trim().is_empty()
        {
            self.mission_form.message = "Tous les champs sont obligatoires.".into();
            cx.notify();
            return;
        }

        let is_new = self.mission_form.selected_id.is_none();
        let new_id = self.next_mission_id();

        let result = if let Ok(conn) = crate::db::get_connection() {
            if let Some(id) = self.mission_form.selected_id {
                mission::update(
                    &conn, id,
                    temps_theorique.trim(), description.trim(),
                    date_creation.trim(), date_debut.trim(), date_fin.trim(),
                    derniere_modif.trim(), ville_mission.trim(), departement_mission.trim(),
                    self.mission_form.id_status, self.mission_form.id_intervenant,
                )
            } else {
                mission::insert(
                    &conn, new_id,
                    temps_theorique.trim(), description.trim(),
                    date_creation.trim(), date_debut.trim(), date_fin.trim(),
                    derniere_modif.trim(), ville_mission.trim(), departement_mission.trim(),
                    self.mission_form.id_status, self.mission_form.id_intervenant,
                )
            }
        } else {
            self.mission_form.message = "Connexion DB impossible.".into();
            cx.notify();
            return;
        };

        match result {
            Ok(_) => {
                self.reload_missions(cx);
                self.clear_mission_form(cx);

                if is_new {
                    // Charger les candidats et naviguer vers l'affectation
                    self.mission_en_affectation_id = Some(new_id);
                    self.affectation_message = String::new();
                    self.reload_sous_traitants(cx);
                    self.candidats_affectation = crate::db::get_connection()
                        .and_then(|conn| mission::get_tous_candidats(&conn, new_id))
                        .unwrap_or_default();
                    self.active_view = ActiveView::AffectationView;
                } else {
                    self.mission_form.message = "Mission mise a jour.".into();
                }
            }
            Err(err) => self.mission_form.message = format!("Erreur mission: {err}"),
        }
        cx.notify();
    }

    fn delete_mission(
        &mut self,
        _: &gpui::MouseDownEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(id) = self.mission_form.selected_id else {
            self.mission_form.message = "Selectionne une mission.".into();
            cx.notify();
            return;
        };
        match crate::db::get_connection().and_then(|conn| mission::delete(&conn, id)) {
            Ok(_) => {
                self.reload_missions(cx);
                self.mission_form.message = "Mission supprimee.".into();
                self.clear_mission_form(cx);
            }
            Err(err) => self.mission_form.message = format!("Erreur: {err}"),
        }
        cx.notify();
    }

    fn assign_best_mission_intervenant(
        &mut self,
        _: &gpui::MouseDownEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(id_mission) = self.mission_form.selected_id else {
            self.mission_form.message = "Selectionne d'abord une mission.".into();
            cx.notify();
            return;
        };

        match crate::db::get_connection()
            .and_then(|conn| mission::assign_best_intervenant(&conn, id_mission))
        {
            Ok(Some(rec)) => {
                self.reload_missions(cx);
                self.select_mission(id_mission, cx);
                self.mission_form.message = format!(
                    "Affecte a {} {} (niveau {}, motivation {}).",
                    rec.prenom_intervenant, rec.nom_intervenant, rec.niveau, rec.preference
                );
            }
            Ok(None) => {
                self.mission_form.message =
                    "Aucun intervenant disponible avec les competences requises.".into();
            }
            Err(err) => self.mission_form.message = format!("Erreur: {err}"),
        }
        cx.notify();
    }

    // ── Affectation manuelle ─────────────────────────────────────────────────

    fn affecter_candidat(&mut self, id_intervenant: i32, cx: &mut Context<Self>) {
        let Some(id_mission) = self.mission_en_affectation_id else {
            return;
        };
        match crate::db::get_connection()
            .and_then(|conn| mission::assign_intervenant(&conn, id_mission, id_intervenant))
        {
            Ok(_) => {
                self.reload_missions(cx);
                self.mission_en_affectation_id = None;
                self.candidats_affectation.clear();
                self.active_view = ActiveView::MissionList;
            }
            Err(err) => {
                self.affectation_message = format!("Erreur affectation: {err}");
            }
        }
        cx.notify();
    }

    fn ignorer_affectation(&mut self, cx: &mut Context<Self>) {
        self.mission_en_affectation_id = None;
        self.candidats_affectation.clear();
        self.affectation_message.clear();
        self.switch_view(ActiveView::MissionList, cx);
    }

    // ── CRUD Sous-traitants ──────────────────────────────────────────────────

    fn save_sous_traitant(
        &mut self,
        _: &gpui::MouseDownEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let nom = Self::read_input(&self.sous_traitant_form.nom_entreprise, cx);
        let metier = Self::read_input(&self.sous_traitant_form.metier, cx);
        let tarif_raw = Self::read_input(&self.sous_traitant_form.tarif_horaire, cx);

        if nom.trim().is_empty() || metier.trim().is_empty() {
            self.sous_traitant_form.message = "Nom et metier sont obligatoires.".into();
            cx.notify();
            return;
        }

        let tarif: f64 = tarif_raw.trim().replace(',', ".").parse().unwrap_or(0.0);

        let result = if let Ok(conn) = crate::db::get_connection() {
            if let Some(id) = self.sous_traitant_form.selected_id {
                sous_traitant::update(&conn, id, nom.trim(), metier.trim(), tarif)
            } else {
                sous_traitant::insert(
                    &conn,
                    self.next_sous_traitant_id(),
                    nom.trim(),
                    metier.trim(),
                    tarif,
                )
            }
        } else {
            self.sous_traitant_form.message = "Connexion DB impossible.".into();
            cx.notify();
            return;
        };

        match result {
            Ok(_) => {
                self.reload_sous_traitants(cx);
                self.sous_traitant_form.message = "Prestataire enregistre.".into();
                self.clear_sous_traitant_form(cx);
            }
            Err(err) => self.sous_traitant_form.message = format!("Erreur: {err}"),
        }
        cx.notify();
    }

    fn delete_sous_traitant(
        &mut self,
        _: &gpui::MouseDownEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(id) = self.sous_traitant_form.selected_id else {
            self.sous_traitant_form.message = "Selectionne un prestataire.".into();
            cx.notify();
            return;
        };
        match crate::db::get_connection().and_then(|conn| sous_traitant::delete(&conn, id)) {
            Ok(_) => {
                self.reload_sous_traitants(cx);
                self.sous_traitant_form.message = "Prestataire supprime.".into();
                self.clear_sous_traitant_form(cx);
            }
            Err(err) => self.sous_traitant_form.message = format!("Erreur: {err}"),
        }
        cx.notify();
    }

    // ── Helpers de rendu ─────────────────────────────────────────────────────

    fn render_button(
        &self,
        label: &str,
        color: u32,
        cx: &mut Context<Self>,
        on_click: impl Fn(&mut Self, &gpui::MouseDownEvent, &mut Window, &mut Context<Self>) + 'static,
    ) -> impl IntoElement {
        div()
            .px_3()
            .py_2()
            .bg(rgb(color))
            .rounded_md()
            .cursor_pointer()
            .child(label.to_string())
            .on_mouse_down(MouseButton::Left, cx.listener(on_click))
    }

    fn render_back_button(&self, cx: &mut Context<Self>) -> impl IntoElement {
        self.render_button("Retour", 0x505862, cx, |this, _, _, cx| {
            this.switch_view(ActiveView::Home, cx);
        })
    }

    fn render_dashboard_card(
        &self,
        title: &str,
        subtitle: &str,
        color_hex: u32,
        target_view: ActiveView,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_64()
            .h_40()
            .bg(rgb(0x2D2D2D))
            .rounded_lg()
            .border_1()
            .border_color(rgb(0x404040))
            .shadow_md()
            .cursor_pointer()
            .hover(|s| s.bg(rgb(0x383838)))
            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| {
                this.switch_view(target_view, cx);
            }))
            .child(div().h_2().w_full().rounded_t_lg().bg(rgb(color_hex)))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .p_4()
                    .gap_2()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child(title.to_string()),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0xAAAAAA))
                            .child(subtitle.to_string()),
                    ),
            )
    }

    // ── Vues ─────────────────────────────────────────────────────────────────

    fn render_home(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .p_8()
            .gap_8()
            .child(
                div()
                    .text_2xl()
                    .font_weight(gpui::FontWeight::BOLD)
                    .text_color(rgb(0xAAAAAA))
                    .child("Tableau de bord"),
            )
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_6()
                    .child(self.render_dashboard_card(
                        "Clients", "Gestion clients", 0x4CAF50, ActiveView::ClientList, cx,
                    ))
                    .child(self.render_dashboard_card(
                        "Missions", "Gestion missions", 0x2196F3, ActiveView::MissionList, cx,
                    ))
                    .child(self.render_dashboard_card(
                        "Employes", "Gestion employes", 0xFFC107, ActiveView::EmployeeList, cx,
                    ))
                    .child(self.render_dashboard_card(
                        "Prestataires", "Sous-traitants & tarifs", 0x7C3AED, ActiveView::SousTraitantList, cx,
                    ))
                    .child(self.render_dashboard_card(
                        "Planning", "Emploi du temps general", 0x0D7377, ActiveView::PlanningView, cx,
                    )),
            )
    }

    fn render_client_list(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .size_full()
            .gap_6()
            .p_6()
            .child(super::client_render::render_list(
                &self.clients,
                self.client_form.selected_id,
                cx,
                |this, cx| this.switch_view(ActiveView::Home, cx),
                |this, id, cx| this.select_client(id, cx),
            ))
            .child(
                div()
                    .w_96()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(
                        div()
                            .text_xl()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child(if self.client_form.selected_id.is_some() {
                                "Modifier le client"
                            } else {
                                "Nouveau client"
                            }),
                    )
                    .child(self.client_form.nom.clone())
                    .child(self.client_form.prenom.clone())
                    .child(self.client_form.raison_social.clone())
                    .child(self.client_form.telephone.clone())
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .child(self.render_button("Enregistrer", 0x2E8B57, cx, Self::save_client))
                            .child(self.render_button("Nouveau", 0x3B82F6, cx, |this, _, _, cx| {
                                this.clear_client_form(cx)
                            }))
                            .child(self.render_button("Supprimer", 0xB91C1C, cx, Self::delete_client)),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0xFCA5A5))
                            .child(self.client_form.message.clone()),
                    ),
            )
    }

    fn render_mission_list(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .size_full()
            .gap_6()
            .p_6()
            .child(super::mission_render::render_list(
                &self.missions,
                self.mission_form.selected_id,
                cx,
                |this, cx| this.switch_view(ActiveView::Home, cx),
                |this, id, cx| this.select_mission(id, cx),
            ))
            .child(
                div()
                    .w_96()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(
                        div()
                            .text_xl()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child(if self.mission_form.selected_id.is_some() {
                                "Modifier la mission"
                            } else {
                                "Nouvelle mission"
                            }),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0xA3A3A3))
                            .child("Creer une nouvelle mission lance le workflow d'affectation."),
                    )
                    .child(self.mission_form.temps_theorique.clone())
                    .child(self.mission_form.description.clone())
                    .child(self.mission_form.date_debut.clone())
                    .child(self.mission_form.date_fin.clone())
                    .child(self.mission_form.ville_mission.clone())
                    .child(self.mission_form.departement_mission.clone())
                    .child(
                        div()
                            .flex()
                            .flex_wrap()
                            .gap_2()
                            .child(self.render_button(
                                "Auto-affecter", 0x8B5CF6, cx,
                                Self::assign_best_mission_intervenant,
                            ))
                            .child(self.render_button("Enregistrer", 0x2E8B57, cx, Self::save_mission))
                            .child(self.render_button("Nouveau", 0x3B82F6, cx, |this, _, _, cx| {
                                this.clear_mission_form(cx)
                            }))
                            .child(self.render_button("Supprimer", 0xB91C1C, cx, Self::delete_mission)),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0xFCA5A5))
                            .child(self.mission_form.message.clone()),
                    ),
            )
    }

    fn render_employee_list(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .size_full()
            .gap_6()
            .p_6()
            .child(super::employee_render::render_list(
                &self.employees,
                self.employee_form.selected_id,
                cx,
                |this, cx| this.switch_view(ActiveView::Home, cx),
                |this, id, cx| this.select_employee(id, cx),
            ))
            .child(
                div()
                    .w_96()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(
                        div()
                            .text_xl()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child(if self.employee_form.selected_id.is_some() {
                                "Modifier l'employe"
                            } else {
                                "Nouvel employe"
                            }),
                    )
                    .child(self.employee_form.id_category.clone())
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .child(self.render_button("Enregistrer", 0x2E8B57, cx, Self::save_employee))
                            .child(self.render_button("Nouveau", 0x3B82F6, cx, |this, _, _, cx| {
                                this.clear_employee_form(cx)
                            }))
                            .child(self.render_button("Supprimer", 0xB91C1C, cx, Self::delete_employee)),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0xFCA5A5))
                            .child(self.employee_form.message.clone()),
                    ),
            )
    }

    fn render_sous_traitant_list(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .size_full()
            .gap_6()
            .p_6()
            .child(super::sous_traitant_render::render_list(
                &self.sous_traitants,
                self.sous_traitant_form.selected_id,
                cx,
                |this, cx| this.switch_view(ActiveView::Home, cx),
                |this, id, cx| this.select_sous_traitant(id, cx),
            ))
            .child(
                div()
                    .w_96()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(
                        div()
                            .text_xl()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child(if self.sous_traitant_form.selected_id.is_some() {
                                "Modifier le prestataire"
                            } else {
                                "Nouveau prestataire"
                            }),
                    )
                    .child(self.sous_traitant_form.nom_entreprise.clone())
                    .child(self.sous_traitant_form.metier.clone())
                    .child(self.sous_traitant_form.tarif_horaire.clone())
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .child(self.render_button(
                                "Enregistrer", 0x2E8B57, cx, Self::save_sous_traitant,
                            ))
                            .child(self.render_button("Nouveau", 0x3B82F6, cx, |this, _, _, cx| {
                                this.clear_sous_traitant_form(cx)
                            }))
                            .child(self.render_button(
                                "Supprimer", 0xB91C1C, cx, Self::delete_sous_traitant,
                            )),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0xFCA5A5))
                            .child(self.sous_traitant_form.message.clone()),
                    ),
            )
    }

    fn render_planning(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let filtre = Self::read_input(&self.planning_filtre, cx);

        div()
            .flex()
            .flex_col()
            .size_full()
            .p_6()
            .gap_4()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_4()
                    .child(
                        div()
                            .p_2()
                            .bg(rgb(0x404040))
                            .rounded_md()
                            .cursor_pointer()
                            .child("Retour")
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                this.switch_view(ActiveView::Home, cx);
                            })),
                    )
                    .child(
                        div()
                            .text_2xl()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("Emploi du temps general"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0xAAAAAA))
                            .child("Filtrer:"),
                    )
                    .child(self.planning_filtre.clone()),
            )
            .child(
                div()
                    .flex_1()
                    .child(super::planning_render::render_liste::<Self>(
                        &self.emploi_du_temps,
                        &filtre,
                    )),
            )
    }

    fn render_affectation(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(id_mission) = self.mission_en_affectation_id else {
            return div().child("Aucune mission en cours d'affectation.").into_any_element();
        };

        let mission_desc = self
            .missions
            .iter()
            .find(|m| m.id_mission == id_mission)
            .map(|m| m.description.clone())
            .unwrap_or_else(|| format!("Mission #{}", id_mission));

        // Séparer les candidats en groupes
        let dispo_match: Vec<&CandidatIntervenant> = self
            .candidats_affectation
            .iter()
            .filter(|c| c.disponible && c.nb_competences_match >= c.nb_competences_requises && c.nb_competences_requises > 0)
            .collect();

        let dispo_partiel: Vec<&CandidatIntervenant> = self
            .candidats_affectation
            .iter()
            .filter(|c| c.disponible && (c.nb_competences_match < c.nb_competences_requises || c.nb_competences_requises == 0))
            .collect();

        let non_dispo: Vec<&CandidatIntervenant> = self
            .candidats_affectation
            .iter()
            .filter(|c| !c.disponible)
            .collect();

        div()
            .flex()
            .flex_col()
            .size_full()
            .p_6()
            .gap_4()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_4()
                    .child(
                        div()
                            .p_2()
                            .bg(rgb(0x404040))
                            .rounded_md()
                            .cursor_pointer()
                            .child("Passer")
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                this.ignorer_affectation(cx);
                            })),
                    )
                    .child(
                        div()
                            .text_2xl()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child(format!("Affectation — Mission #{}: {}", id_mission, mission_desc)),
                    ),
            )
            // Section match parfait
            .child(self.render_section_candidats(
                "Disponibles — Match parfait",
                0x16A34A,
                &dispo_match,
                true,
                cx,
            ))
            // Section partiel
            .child(self.render_section_candidats(
                "Disponibles — Partiellement qualifies ou sans competences requises",
                0xD97706,
                &dispo_partiel,
                true,
                cx,
            ))
            // Section non dispo
            .child(self.render_section_candidats(
                "Non disponibles",
                0x6B7280,
                &non_dispo,
                false,
                cx,
            ))
            // Prestataires
            .child(self.render_section_prestataires(cx))
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(0xFCA5A5))
                    .child(self.affectation_message.clone()),
            )
            .into_any_element()
    }

    fn render_section_candidats(
        &self,
        titre: &str,
        color: u32,
        candidats: &[&CandidatIntervenant],
        avec_bouton: bool,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(div().w_3().h_3().rounded_full().bg(rgb(color)))
                    .child(
                        div()
                            .text_base()
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_color(rgb(color))
                            .child(titre.to_string()),
                    ),
            )
            .child(if candidats.is_empty() {
                div()
                    .px_4()
                    .text_sm()
                    .text_color(rgb(0x555555))
                    .child("Aucun")
                    .into_any_element()
            } else {
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .children(candidats.iter().map(|c| {
                        let id_int = c.id_intervenant;
                        let badge = if c.est_prestataire { "Prestataire" } else { "Employe" };
                        let badge_color = if c.est_prestataire { 0x7C3AED } else { 0x0D7377 };
                        let comp_label = if c.nb_competences_requises == 0 {
                            "Pas de comp. requises".to_string()
                        } else {
                            format!(
                                "{}/{} competences",
                                c.nb_competences_match, c.nb_competences_requises
                            )
                        };

                        let row = div()
                            .flex()
                            .items_center()
                            .gap_3()
                            .px_4()
                            .py_2()
                            .bg(rgb(0x2D2D2D))
                            .rounded_md()
                            .child(
                                div()
                                    .px_2()
                                    .py_1()
                                    .bg(rgb(badge_color))
                                    .rounded_md()
                                    .text_xs()
                                    .child(badge),
                            )
                            .child(
                                div()
                                    .w_40()
                                    .font_weight(gpui::FontWeight::BOLD)
                                    .child(format!("{} {}", c.prenom, c.nom)),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .text_sm()
                                    .text_color(rgb(0xAAAAAA))
                                    .child(comp_label),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(0xAAAAAA))
                                    .child(format!(
                                        "Niv {:.1} | Motiv {:.1}",
                                        c.niveau_moyen, c.preference_moyenne
                                    )),
                            );

                        if avec_bouton {
                            row.child(
                                div()
                                    .px_3()
                                    .py_1()
                                    .bg(rgb(0x2563EB))
                                    .rounded_md()
                                    .cursor_pointer()
                                    .text_sm()
                                    .child("Affecter")
                                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| {
                                        this.affecter_candidat(id_int, cx);
                                    })),
                            ).into_any_element()
                        } else {
                            row.into_any_element()
                        }
                    }))
                    .into_any_element()
            })
    }

    fn render_section_prestataires(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let prestataires_dispo: Vec<(&CandidatIntervenant, Option<&SousTraitant>)> = self
            .candidats_affectation
            .iter()
            .filter(|c| c.est_prestataire && c.disponible)
            .map(|c| {
                let st = c.id_sous_traitant.and_then(|st_id| {
                    self.sous_traitants.iter().find(|s| s.id_sous_traitant == st_id)
                });
                (c, st)
            })
            .collect();

        div()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(div().w_3().h_3().rounded_full().bg(rgb(0x7C3AED)))
                    .child(
                        div()
                            .text_base()
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_color(rgb(0x7C3AED))
                            .child("Prestataires disponibles"),
                    ),
            )
            .child(if prestataires_dispo.is_empty() {
                div()
                    .px_4()
                    .text_sm()
                    .text_color(rgb(0x555555))
                    .child("Aucun prestataire disponible — pensez a en ajouter dans la section Prestataires.")
                    .into_any_element()
            } else {
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .children(prestataires_dispo.into_iter().map(|(c, st)| {
                        let id_int = c.id_intervenant;
                        let tarif_label = st
                            .map(|s| format!("{} — {:.0}€/h", s.metier, s.tarif_horaire))
                            .unwrap_or_else(|| "Tarif non renseigne".to_string());

                        div()
                            .flex()
                            .items_center()
                            .gap_3()
                            .px_4()
                            .py_2()
                            .bg(rgb(0x2D2D2D))
                            .rounded_md()
                            .child(
                                div()
                                    .px_2()
                                    .py_1()
                                    .bg(rgb(0x7C3AED))
                                    .rounded_md()
                                    .text_xs()
                                    .child("Prestataire"),
                            )
                            .child(
                                div()
                                    .w_40()
                                    .font_weight(gpui::FontWeight::BOLD)
                                    .child(format!("{} {}", c.prenom, c.nom)),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .text_sm()
                                    .text_color(rgb(0xAAAAAA))
                                    .child(tarif_label),
                            )
                            .child(
                                div()
                                    .px_3()
                                    .py_1()
                                    .bg(rgb(0x7C3AED))
                                    .rounded_md()
                                    .cursor_pointer()
                                    .text_sm()
                                    .child("Affecter")
                                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| {
                                        this.affecter_candidat(id_int, cx);
                                    })),
                            )
                    }))
                    .into_any_element()
            })
    }
}

impl Render for GestaBoxApp {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x1E1E1E))
            .text_color(rgb(0xFFFFFF))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .p_4()
                    .bg(rgb(0x2D2D2D))
                    .border_b_1()
                    .border_color(rgb(0x404040))
                    .child(
                        div()
                            .text_xl()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("GestaBox"),
                    )
                    .child(if self.active_view == ActiveView::Home {
                        div().into_any_element()
                    } else {
                        self.render_back_button(cx).into_any_element()
                    }),
            )
            .child(match self.active_view {
                ActiveView::Home => self.render_home(cx).into_any_element(),
                ActiveView::ClientList => self.render_client_list(cx).into_any_element(),
                ActiveView::MissionList => self.render_mission_list(cx).into_any_element(),
                ActiveView::EmployeeList => self.render_employee_list(cx).into_any_element(),
                ActiveView::SousTraitantList => self.render_sous_traitant_list(cx).into_any_element(),
                ActiveView::PlanningView => self.render_planning(cx).into_any_element(),
                ActiveView::AffectationView => self.render_affectation(cx).into_any_element(),
            })
    }
}

pub(crate) fn run() {
    Application::new().run(|cx: &mut App| {
        super::text_input::bind_keys(cx);

        let bounds = Bounds::centered(None, size(px(1280.0), px(720.0)), cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Maximized(bounds)),
                titlebar: None,
                ..Default::default()
            },
            |_, cx| cx.new(|cx| GestaBoxApp::new(cx)),
        )
        .unwrap();
    });
}
