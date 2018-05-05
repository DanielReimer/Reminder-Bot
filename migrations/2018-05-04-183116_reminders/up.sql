CREATE TABLE reminders (
  id SERIAL PRIMARY KEY,
  nick VARCHAR NOT NULL,
  set_time INT NOT NULL,
  remind_time INT NOT NULL
)
