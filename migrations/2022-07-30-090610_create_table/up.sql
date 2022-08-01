CREATE TABLE groups (
   group_id VARCHAR PRIMARY KEY,
   name TEXT NOT NULL,
   detail TEXT NOT NULL,
   is_active BOOLEAN NOT NULL,
   updated_at TIMESTAMP NOT NULL,
   created_at TIMESTAMP NOT NULL DEFAULT current_timestamp
);