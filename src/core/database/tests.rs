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

#[test]
fn when_insert_command_must_insert_data() {
    
    let mut database = Database::new();
    
    let result = database.execute("
        CREATE TABLE distributors (
            id      Int CONSTRAINT no_null NOT NULL,
            name    String NOT NULL
        );
    ");

    assert_eq!(Ok(0), result);

    let result = database.execute("
        INSERT INTO distributors (id, name) values(1, \"test\");
    ");

    assert_eq!(Ok(0), result);
}