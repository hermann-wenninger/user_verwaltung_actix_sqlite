-- Erstellt die Users-Tabelle
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    role TEXT NOT NULL,
    password_hash TEXT NOT NULL
);

-- Erstellt die Profiles-Tabelle mit einer Beziehung zur Users-Tabelle
CREATE TABLE profiles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    profile_photo TEXT,
    age INTEGER NOT NULL,
    description TEXT,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

-- Füge 10 Testuser ein
INSERT INTO users (name, email, role, password_hash) VALUES
('Alice Example', 'alice@example.com', 'user', '$2b$12$abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'), 
('Bob Example', 'bob@example.com', 'admin', '$2b$12$abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'), 
('Charlie Test', 'charlie@test.com', 'user', '$2b$12$abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),
('David Sample', 'david@sample.com', 'user', '$2b$12$abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),
('Eve Hacker', 'eve@hacker.com', 'user', '$2b$12$abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),
('Frank Admin', 'frank@admin.com', 'admin', '$2b$12$abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),
('Grace Secure', 'grace@secure.com', 'user', '$2b$12$abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),
('Hank Dev', 'hank@dev.com', 'user', '$2b$12$abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),
('Ivy User', 'ivy@user.com', 'user', '$2b$12$abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),
('Jack Example', 'jack@example.com', 'admin', '$2b$12$abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij');

-- Füge Test-Profile für die Benutzer ein
INSERT INTO profiles (user_id, profile_photo, age, description) VALUES
(1, '/images/alice.jpg', 25, 'Alice ist eine Testbenutzerin.'),
(2, '/images/bob.jpg', 30, 'Bob ist ein Administrator.'),
(3, '/images/charlie.jpg', 22, 'Charlie liebt Rust und Actix.'),
(4, '/images/david.jpg', 35, 'David ist ein erfahrener Entwickler.'),
(5, '/images/eve.jpg', 28, 'Eve interessiert sich für Sicherheit.'),
(6, '/images/frank.jpg', 40, 'Frank ist Systemadministrator.'),
(7, '/images/grace.jpg', 27, 'Grace arbeitet in der Softwareentwicklung.'),
(8, '/images/hank.jpg', 29, 'Hank ist ein Backend-Entwickler.'),
(9, '/images/ivy.jpg', 24, 'Ivy ist ein neugieriger Tester.'),
(10, '/images/jack.jpg', 33, 'Jack leitet ein Entwicklerteam.');
