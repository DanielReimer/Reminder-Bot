CREATE TABLE reminders (
  id SERIAL PRIMARY KEY,
  nick VARCHAR NOT NULL,
  channel VARCHAR NOT NULL,
  set_time INT8 NOT NULL,
  remind_time INT8 NOT NULL,
  reminded BOOLEAN NOT NULL DEFAULT FALSE,
  remind_message TEXT NOT NULL
)
