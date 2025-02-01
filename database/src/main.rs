use rusqlite::{params, Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("database.sqlite")?;

    // Erstelle die Users-Tabelle
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT UNIQUE NOT NULL,
            role TEXT NOT NULL,
            password_hash TEXT NOT NULL
        );",
        [],
    )?;

    // Erstelle die Profiles-Tabelle
    conn.execute(
        "CREATE TABLE IF NOT EXISTS profiles (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            profile_photo TEXT,
            age INTEGER NOT NULL,
            description TEXT,
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
        );",
        [],
    )?;

    // Beispielpasswörter (unsicher, in Produktion bcrypt verwenden!)
    let users = vec![
        ("Alice Example", "alice@example.com", "user", "hashed_pw1"),
        ("Bob Example", "bob@example.com", "admin", "hashed_pw2"),
        ("Charlie Test", "charlie@test.com", "user", "hashed_pw3"),
        ("David Sample", "david@sample.com", "user", "hashed_pw4"),
        ("Eve Hacker", "eve@hacker.com", "user", "hashed_pw5"),
        ("Frank Admin", "frank@admin.com", "admin", "hashed_pw6"),
        ("Grace Secure", "grace@secure.com", "user", "hashed_pw7"),
        ("Hank Dev", "hank@dev.com", "user", "hashed_pw8"),
        ("Ivy User", "ivy@user.com", "user", "hashed_pw9"),
        ("Jack Example", "jack@example.com", "admin", "hashed_pw10")
    ];

    for (name, email, role, password) in &users {
        conn.execute(
            "INSERT INTO users (name, email, role, password_hash) VALUES (?1, ?2, ?3, ?4)",
            params![name, email, role, password],
        )?;
    }

    // Beispiel-Profile für Benutzer
    let profiles = vec![
        (1, "/images/alice.jpg", 25, "Alice ist eine Testbenutzerin."),
        (2, "/images/bob.jpg", 30, "Bob ist ein Administrator."),
        (3, "/images/charlie.jpg", 22, "Charlie liebt Rust und Actix."),
        (4, "/images/david.jpg", 35, "David ist ein erfahrener Entwickler."),
        (5, "/images/eve.jpg", 28, "Eve interessiert sich für Sicherheit."),
        (6, "/images/frank.jpg", 40, "Frank ist Systemadministrator."),
        (7, "/images/grace.jpg", 27, "Grace arbeitet in der Softwareentwicklung."),
        (8, "/images/hank.jpg", 29, "Hank ist ein Backend-Entwickler."),
        (9, "/images/ivy.jpg", 24, "Ivy ist ein neugieriger Tester."),
        (10, "/images/jack.jpg", 33, "Jack leitet ein Entwicklerteam.")
    ];

    for (user_id, profile_photo, age, description) in &profiles {
        conn.execute(
            "INSERT INTO profiles (user_id, profile_photo, age, description) VALUES (?1, ?2, ?3, ?4)",
            params![user_id, profile_photo, age, description],
        )?;
    }

    println!("Datenbank und Testbenutzer mit Profilen erfolgreich erstellt.");
    Ok(())
}
