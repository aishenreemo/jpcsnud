-- Add migration script here

-- =========================
-- Users table
-- =========================
CREATE TABLE IF NOT EXISTS "User" (
    user_id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL
);

-- =========================
-- RFID table
-- =========================
CREATE TABLE IF NOT EXISTS "RFID" (
    rfid_tag_id VARCHAR(50) PRIMARY KEY,
    user_id INTEGER NOT NULL,
    CONSTRAINT fk_rfid_user FOREIGN KEY(user_id) REFERENCES "User"(user_id) ON DELETE CASCADE
);

-- =========================
-- Roles table
-- =========================
CREATE TABLE IF NOT EXISTS "Role" (
    role_id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    permission_mask INTEGER NOT NULL
);

-- =========================
-- UserRoles table
-- =========================
CREATE TABLE IF NOT EXISTS "UserRole" (
    user_role_id SERIAL PRIMARY KEY,
    role_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    CONSTRAINT fk_userrole_role FOREIGN KEY(role_id) REFERENCES "Role"(role_id) ON DELETE CASCADE,
    CONSTRAINT fk_userrole_user FOREIGN KEY(user_id) REFERENCES "User"(user_id) ON DELETE CASCADE
);

-- =========================
-- Events table
-- =========================
CREATE TABLE IF NOT EXISTS "Event" (
    event_id SERIAL PRIMARY KEY,
    event_name VARCHAR(255) NOT NULL,
    event_date DATE NOT NULL
);

-- =========================
-- Attendance table
-- =========================
CREATE TABLE IF NOT EXISTS "Attendance" (
    attendance_id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    event_id INTEGER NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_attendance_user FOREIGN KEY(user_id) REFERENCES "User"(user_id) ON DELETE CASCADE,
    CONSTRAINT fk_attendance_event FOREIGN KEY(event_id) REFERENCES "Event"(event_id) ON DELETE CASCADE
);
