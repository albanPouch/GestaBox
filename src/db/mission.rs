use rusqlite::{params, Connection, Result};

use super::mission_historique;

#[derive(Debug, Clone)]
pub struct CandidatIntervenant {
    pub id_intervenant: i32,
    pub nom: String,
    pub prenom: String,
    pub est_prestataire: bool,
    pub id_sous_traitant: Option<i32>,
    pub nb_competences_match: i32,
    pub nb_competences_requises: i32,
    pub niveau_moyen: f64,
    pub preference_moyenne: f64,
    pub disponible: bool,
}

#[derive(Debug, Clone)]
pub struct MissionLigne {
    pub id_mission: i32,
    pub description: String,
    pub date_debut: String,
    pub date_fin: String,
    pub id_status: i32,
}

#[derive(Debug, Clone)]
pub struct EmploiDuTempsLigne {
    pub id_intervenant: i32,
    pub nom: String,
    pub prenom: String,
    pub est_prestataire: bool,
    pub missions: Vec<MissionLigne>,
}

// La Structure (Données)
#[derive(Debug, Clone)]
pub struct Mission {
    pub id_mission: i32,
    pub temps_theorique: String,
    pub description: String,
    pub date_creation: String ,
    pub date_debut: String,
    pub date_fin: String ,
    pub derniere_modif: String,
    pub ville_mission: String ,
    pub departement_mission : String,
    pub id_status: i32,
    pub id_intervenant: i32,
}

#[derive(Debug, Clone)]
pub struct AssignmentRecommendation {
    pub id_intervenant: i32,
    pub nom_intervenant: String,
    pub prenom_intervenant: String,
    pub id_competence: i32,
    pub libelle_competence: String,
    pub niveau: i32,
    pub preference: i32,
}

// Création de la table
pub fn init_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Mission(
        id_mission INT,
        temps_theorique VARCHAR(50),
        description VARCHAR(50),
        date_creation VARCHAR(50),
        date_debut VARCHAR(50),
        date_fin VARCHAR(50),
        derniere_modif VARCHAR(50),
        ville_mission VARCHAR(50),
        departement_mission VARCHAR(50),
        id_status INT NOT NULL,
        id_intervenant INT NOT NULL,
        FOREIGN KEY(id_status) REFERENCES status(id_status),
        FOREIGN KEY(id_intervenant) REFERENCES Intervenant(id_intervant),
        PRIMARY KEY(id_mission)
        )",
        [],
    )?;
    Ok(())
}

pub fn insert(
    conn: &Connection,
    id_mission: i32,
    temps_theorique:  &str,
    description:  &str,
    date_creation:  &str,
    date_debut : &str,
    date_fin :  &str,
    derniere_modif:  &str,
    ville_mission :  &str,
    departement_mission :  &str,
    id_status : i32,
    id_intervenant: i32,
) -> Result<()> {
    conn.execute(
        "INSERT INTO Mission (
            id_mission, temps_theorique, description, date_creation, date_debut, date_fin, derniere_modif, ville_mission , departement_mission, id_status, id_intervenant
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        params![id_mission, temps_theorique, description,
            date_creation, date_debut, date_fin ,derniere_modif,
            ville_mission, departement_mission, id_status , id_intervenant
        ],
    )?;
    Ok(())
}

pub fn update(
    conn: &Connection,
    id_mission: i32,
    temps_theorique: &str,
    description: &str,
    date_creation: &str,
    date_debut: &str,
    date_fin: &str,
    derniere_modif: &str,
    ville_mission: &str,
    departement_mission: &str,
    id_status: i32,
    id_intervenant: i32,
) -> Result<()> {
    conn.execute(
        "UPDATE Mission
         SET temps_theorique = ?2,
             description = ?3,
             date_creation = ?4,
             date_debut = ?5,
             date_fin = ?6,
             derniere_modif = ?7,
             ville_mission = ?8,
             departement_mission = ?9,
             id_status = ?10,
             id_intervenant = ?11
         WHERE id_mission = ?1",
        params![
            id_mission,
            temps_theorique,
            description,
            date_creation,
            date_debut,
            date_fin,
            derniere_modif,
            ville_mission,
            departement_mission,
            id_status,
            id_intervenant
        ],
    )?;
    Ok(())
}

pub fn delete(conn: &Connection, id_mission: i32) -> Result<()> {
    conn.execute("DELETE FROM Mission WHERE id_mission = ?1", params![id_mission])?;
    Ok(())
}

pub fn recommend_intervenant(
    conn: &Connection,
    id_mission: i32,
) -> Result<Option<AssignmentRecommendation>> {
    let mut stmt = conn.prepare(
        "
        WITH mission_dates AS (
            SELECT id_mission, date_debut, date_fin
            FROM Mission
            WHERE id_mission = ?1
        ),
        required_competences AS (
            SELECT DISTINCT d.id_competence
            FROM Activite a
            INNER JOIN Demander d ON d.id_activite = a.id_activite
            WHERE a.id_mission = ?1
        )
        SELECT
            i.id_intervant,
            i.nom_intervenant,
            i.prenom_intervenant,
            p.id_competence,
            c.libelle_competence,
            CAST(p.niveau AS INTEGER),
            CAST(p.preference AS INTEGER)
        FROM Intervenant i
        INNER JOIN Posseder p ON p.id_intervenant = i.id_intervant
        INNER JOIN required_competences rc ON rc.id_competence = p.id_competence
        INNER JOIN Competence c ON c.id_competence = p.id_competence
        INNER JOIN mission_dates md
        WHERE NOT EXISTS (
            SELECT 1
            FROM Occuper o
            WHERE o.id_planning = i.id_planning
              AND date(o.date_debut) <= date(md.date_fin)
              AND date(o.date_fin) >= date(md.date_debut)
        )
        AND NOT EXISTS (
            SELECT 1
            FROM Mission m2
            WHERE m2.id_intervenant = i.id_intervant
              AND m2.id_mission <> md.id_mission
              AND date(m2.date_debut) <= date(md.date_fin)
              AND date(m2.date_fin) >= date(md.date_debut)
        )
        ORDER BY p.niveau DESC, p.preference DESC, i.id_intervant ASC
        LIMIT 1
        ",
    )?;

    let mut rows = stmt.query(params![id_mission])?;
    if let Some(row) = rows.next()? {
        Ok(Some(AssignmentRecommendation {
            id_intervenant: row.get(0)?,
            nom_intervenant: row.get(1)?,
            prenom_intervenant: row.get(2)?,
            id_competence: row.get(3)?,
            libelle_competence: row.get(4)?,
            niveau: row.get(5)?,
            preference: row.get(6)?,
        }))
    } else {
        Ok(None)
    }
}

pub fn assign_best_intervenant(
    conn: &Connection,
    id_mission: i32,
) -> Result<Option<AssignmentRecommendation>> {
    let Some(recommendation) = recommend_intervenant(conn, id_mission)? else {
        return Ok(None);
    };

    conn.execute(
        "UPDATE Mission
         SET id_intervenant = ?2
         WHERE id_mission = ?1",
        params![id_mission, recommendation.id_intervenant],
    )?;

    Ok(Some(recommendation))
}

pub fn get_all(conn: &Connection) -> Result<Vec<Mission>> {
    let mut stmt = conn.prepare("SELECT id_mission, temps_theorique, description, date_creation, date_debut, date_fin, derniere_modif, ville_mission, departement_mission, id_status, id_intervenant FROM Mission")?;
    let mission_iter = stmt.query_map([], |row| {
        Ok(Mission {
            id_mission: row.get(0)?,
            temps_theorique: row.get(1)?,
            description: row.get(2)?,
            date_creation: row.get(3)?,
            date_debut: row.get(4)?,
            date_fin: row.get(5)?,
            derniere_modif: row.get(6)?,
            ville_mission: row.get(7)?,
            departement_mission: row.get(8)?,
            id_status: row.get(9)?,
            id_intervenant: row.get(10)?,
        })
    })?;

    let mut missions = Vec::new();
    for mission in mission_iter {
        missions.push(mission?);
    }
    Ok(missions)
}

// 4. Lecture
pub fn get_by_id(conn: &Connection, id_mission: i32) -> Result<Mission> {
    conn.query_row(
        "SELECT id_mission, temps_theorique, description, date_creation, date_debut, date_fin, derniere_modif,
        ville_mission , departement_mission, id_status, id_intervenant
         FROM Mission WHERE id_mission = ?1",
        params![id_mission],
        |row| {
            Ok(Mission {
                id_mission: row.get(0)?,
                temps_theorique: row.get(1)?,
                description: row.get(2)?,
                date_creation: row.get(3)?,
                date_debut: row.get(4)?,
                date_fin: row.get(5)?,
                derniere_modif:row.get(6)?,
                ville_mission: row.get(7)?,
                departement_mission: row.get(8)?,
                id_status: row.get(9)?,
                id_intervenant: row.get(10)?,
            })
        },
    )
}
pub fn assign_intervenant(conn: &Connection, id_mission: i32, id_intervenant: i32) -> Result<()> {
    conn.execute(
        "UPDATE Mission SET id_intervenant = ?2 WHERE id_mission = ?1",
        params![id_mission, id_intervenant],
    )?;
    Ok(())
}

pub fn get_tous_candidats(
    conn: &Connection,
    id_mission: i32,
) -> Result<Vec<CandidatIntervenant>> {
    let mut stmt = conn.prepare(
        "
        WITH required_competences AS (
            SELECT DISTINCT d.id_competence
            FROM Activite a
            INNER JOIN Demander d ON d.id_activite = a.id_activite
            WHERE a.id_mission = ?1
        ),
        req_count AS (
            SELECT COUNT(*) as n FROM required_competences
        ),
        intervenant_comp AS (
            SELECT
                i.id_intervant,
                i.nom_intervenant,
                i.prenom_intervenant,
                i.id_planning,
                i.id_sous_traitant,
                CASE WHEN i.id_sous_traitant IS NOT NULL THEN 1 ELSE 0 END as est_prestataire,
                COUNT(DISTINCT CASE WHEN rc.id_competence IS NOT NULL THEN p.id_competence ELSE NULL END) as nb_match,
                COALESCE(AVG(CASE WHEN rc.id_competence IS NOT NULL THEN CAST(p.niveau AS REAL) ELSE NULL END), 0.0) as avg_niveau,
                COALESCE(AVG(CASE WHEN rc.id_competence IS NOT NULL THEN CAST(p.preference AS REAL) ELSE NULL END), 0.0) as avg_pref
            FROM Intervenant i
            LEFT JOIN Posseder p ON p.id_intervenant = i.id_intervant
            LEFT JOIN required_competences rc ON rc.id_competence = p.id_competence
            GROUP BY i.id_intervant
        )
        SELECT
            ic.id_intervant,
            ic.nom_intervenant,
            ic.prenom_intervenant,
            ic.est_prestataire,
            ic.id_sous_traitant,
            ic.nb_match,
            rc.n as nb_required,
            ic.avg_niveau,
            ic.avg_pref,
            CASE WHEN
                NOT EXISTS (
                    SELECT 1 FROM Occuper o
                    WHERE o.id_planning = ic.id_planning
                    AND date(o.date_debut) <= date(m.date_fin)
                    AND date(o.date_fin) >= date(m.date_debut)
                )
                AND NOT EXISTS (
                    SELECT 1 FROM Mission m2
                    WHERE m2.id_intervenant = ic.id_intervant
                    AND m2.id_mission <> ?1
                    AND date(m2.date_debut) <= date(m.date_fin)
                    AND date(m2.date_fin) >= date(m.date_debut)
                )
            THEN 1 ELSE 0 END as disponible
        FROM intervenant_comp ic
        CROSS JOIN req_count rc
        CROSS JOIN Mission m
        WHERE m.id_mission = ?1
        ORDER BY disponible DESC, ic.nb_match DESC, ic.avg_niveau DESC
        ",
    )?;

    let iter = stmt.query_map(params![id_mission], |row| {
        Ok(CandidatIntervenant {
            id_intervenant: row.get(0)?,
            nom: row.get(1)?,
            prenom: row.get(2)?,
            est_prestataire: row.get::<_, i32>(3)? != 0,
            id_sous_traitant: row.get(4)?,
            nb_competences_match: row.get(5)?,
            nb_competences_requises: row.get(6)?,
            niveau_moyen: row.get(7)?,
            preference_moyenne: row.get(8)?,
            disponible: row.get::<_, i32>(9)? != 0,
        })
    })?;
    iter.collect()
}

pub fn get_emploi_du_temps(conn: &Connection) -> Result<Vec<EmploiDuTempsLigne>> {
    let mut stmt = conn.prepare(
        "
        SELECT i.id_intervant, i.nom_intervenant, i.prenom_intervenant,
               CASE WHEN i.id_sous_traitant IS NOT NULL THEN 1 ELSE 0 END as est_prestataire,
               m.id_mission, m.description, m.date_debut, m.date_fin, m.id_status
        FROM Intervenant i
        LEFT JOIN Mission m ON m.id_intervenant = i.id_intervant
        ORDER BY i.id_intervant, m.date_debut
        ",
    )?;

    let mut map: std::collections::BTreeMap<i32, EmploiDuTempsLigne> =
        std::collections::BTreeMap::new();

    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, i32>(3)? != 0,
            row.get::<_, Option<i32>>(4)?,
            row.get::<_, Option<String>>(5)?,
            row.get::<_, Option<String>>(6)?,
            row.get::<_, Option<String>>(7)?,
            row.get::<_, Option<i32>>(8)?,
        ))
    })?;

    for row in rows {
        let (id_int, nom, prenom, est_prestataire, id_mission, description, date_debut, date_fin, id_status) =
            row?;
        let entry = map.entry(id_int).or_insert(EmploiDuTempsLigne {
            id_intervenant: id_int,
            nom: nom.clone(),
            prenom: prenom.clone(),
            est_prestataire,
            missions: Vec::new(),
        });
        if let (Some(id_m), Some(desc), Some(dd), Some(df), Some(st)) =
            (id_mission, description, date_debut, date_fin, id_status)
        {
            entry.missions.push(MissionLigne {
                id_mission: id_m,
                description: desc,
                date_debut: dd,
                date_fin: df,
                id_status: st,
            });
        }
    }

    Ok(map.into_values().collect())
}

// 5. On initialise la table et ajoute des données par défaut
pub fn init_db(conn: &Connection) -> Result<()> {
    init_table(&conn)?;
    mission_historique::init_table(conn)?;
    create_trigger(conn)?;
    create_trigger_update(conn)?;
    create_trigger_block_delete_by_status(conn)?;
    create_trigger_del_mission(conn)?;
    Ok(())
}

fn create_trigger_del_mission(conn: &Connection) -> Result<()> {
    conn.execute(
        "
        CREATE TRIGGER IF NOT EXISTS archive_deleted_mission
        BEFORE DELETE ON Mission
        FOR EACH ROW
        BEGIN
            INSERT INTO Mission_historique (
                id_mission,
                temps_theorique,
                description,
                date_creation,
                date_debut,
                date_fin,
                derniere_modif,
                ville_mission,
                departement_mission,
                id_status,
                id_intervenant,
                date_suppression
            ) VALUES (
                OLD.id_mission,
                OLD.temps_theorique,
                OLD.description,
                OLD.date_creation,
                OLD.date_debut,
                OLD.date_fin,
                OLD.derniere_modif,
                OLD.ville_mission,
                OLD.departement_mission,
                OLD.id_status,
                OLD.id_intervenant,
                datetime('now')
            );
        END;
        ",
        [],
    )?;
    Ok(())
}

pub fn create_trigger_update(conn: &Connection) -> Result<()> {
    conn.execute(
        "
        CREATE TRIGGER IF NOT EXISTS check_dates_before_update
        BEFORE UPDATE ON Mission
        FOR EACH ROW
        WHEN NEW.date_debut > NEW.date_fin
        BEGIN
            SELECT RAISE(
                ABORT,
                'Erreur : date_debut ne peut pas être supérieure à date_fin'
            );
        END;
        ",
        [],
    )?;
    Ok(())
}

pub fn create_trigger(conn: &Connection) -> Result<()> {
    conn.execute(
        "
        CREATE TRIGGER IF NOT EXISTS check_dates_before_insert
        BEFORE INSERT ON Mission
        FOR EACH ROW
        WHEN NEW.date_debut > NEW.date_fin
        BEGIN
            SELECT RAISE(
                ABORT,
                'Erreur : date_debut ne peut pas être supérieure à date_fin'
            );
        END;
        ",
        [],
    )?;
    Ok(())
}

// bloqué la suppression si en cours [ uniquement si annulé ou fini]
pub fn create_trigger_block_delete_by_status(conn: &Connection) -> Result<()> {
    conn.execute(
        "
        CREATE TRIGGER IF NOT EXISTS block_delete_mission_by_status
        BEFORE DELETE ON Mission
        FOR EACH ROW
        WHEN OLD.id_status  NOT IN (2, 3)
        BEGIN
            SELECT RAISE(
                ABORT,
                'Suppression interdite : mission en cours'
            );
        END;
        ",
        [],
    )?;
    Ok(())
}
