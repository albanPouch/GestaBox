mod db;

fn main() {
    db::create_bdd().expect("TODO: JE PANIQUE");
    db::init_gestabox_db().expect("TODO: JE PANIQUE ENCORE PLUS");
}
