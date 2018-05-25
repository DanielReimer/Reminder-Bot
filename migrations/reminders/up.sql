CREATE TABLE reminders (
  id SERIAL PRIMARY KEY,
  nick VARCHAR NOT NULL,
  channel VARCHAR NOT NULL,
  set_time INT NOT NULL,
  remind_time INT NOT NULL,
  scheduled BOOLEAN NOT NULL,
  recurring_number INT NOT NULL,
  recurring_time INT NOT NULL
)
