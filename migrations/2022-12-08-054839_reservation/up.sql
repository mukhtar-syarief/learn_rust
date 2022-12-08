-- Your SQL goes here

CREATE TABLE cars (
    id SERIAL PRIMARY KEY,
    type VARCHAR(50) NOT NULL
);

CREATE TABLE locations (
    id SERIAL PRIMARY KEY,
    region TEXT NOT NULL
);

CREATE TABLE reservations (
    id SERIAL PRIMARY KEY,
    vehicle_type_id INTEGER NOT NULL,
    region_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    pickup_date TIMESTAMP NOT NULL,
    return_date TIMESTAMP NOT NULL,
    CONSTRAINT fk_cars_reservations FOREIGN KEY (vehicle_type_id) REFERENCES cars(id),
    CONSTRAINT fk_locations_reservations FOREIGN KEY (region_id) REFERENCES locations(id),
    CONSTRAINT fk_users_reservations FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE TABLE return_reservation (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    region_id INTEGER NOT NULL,
    odometer INTEGER NOT NULL,
    full_tank BOOLEAN NOT NULL,
    time DATE NOT NULL,
    CONSTRAINT fk_users_return_reservation FOREIGN KEY (user_id) REFERENCES users(id),
    CONSTRAINT fk_locations_return_reservation FOREIGN KEY (region_id) REFERENCES locations(id)
);
