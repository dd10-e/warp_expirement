CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email TEXT NOT NULL,
    username TEXT NOT NULL,
    hashed_password TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

SELECT diesel_manage_updated_at('users');

INSERT INTO public.users(id, email, username, hashed_password, created_at, updated_at) VALUES 
  (1, 'elo210@outlook.fr', 'John Doe 1', '$2y$12$yXTjrGePVLBPUH6YVs2f5OsUEGSotZxdL5Uu/70r63I5GtynVVjkK1', '2018-07-21 16:20:10.203031', '2018-07-21 16:20:10.203031'),
  (2, 'elo210@outlook.fr', 'John Doe 2', '$2y$12$yXTjrGePVLBPUH6YVs2f5OsUEGSotZxdL5Uu/70r63I5GtynVVjkK2', '2018-07-21 16:30:10.203031', '2018-07-21 16:30:10.203031'),
  (3, 'elo210@outlook.fr', 'John Doe 3', '$2y$12$yXTjrGePVLBPUH6YVs2f5OsUEGSotZxdL5Uu/70r63I5GtynVVjkK3', '2018-07-21 16:40:10.203031', '2018-07-21 16:40:10.203031'),
  (4, 'elo210@outlook.fr', 'John Doe 4', '$2y$12$yXTjrGePVLBPUH6YVs2f5OsUEGSotZxdL5Uu/70r63I5GtynVVjkK4', '2018-07-21 16:50:10.203031', '2018-07-21 16:50:10.203031'),
  (5, 'elo210@outlook.fr', 'John Doe 5', '$2y$12$yXTjrGePVLBPUH6YVs2f5OsUEGSotZxdL5Uu/70r63I5GtynVVjkK5', '2018-07-21 16:59:10.203031', '2018-07-21 16:59:10.203031');