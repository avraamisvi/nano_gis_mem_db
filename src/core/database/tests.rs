use super::*;

#[test]
fn when_create_table_command_must_create_new_table() {
    
    let mut database = Database::new();
    
    let result = database.execute("
        CREATE TABLE distributors (
            id      Int CONSTRAINT no_null NOT NULL,
            name    String NOT NULL
        );
    ");

    assert_eq!(Ok(0), result);
}