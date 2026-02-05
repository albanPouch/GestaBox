mod db;

fn main() {
    db::init_gestabox_db().expect("TODO: JE PANIQUE ENCORE PLUS");
}
