CREATE TABLE IF NOT EXISTS users
(
    user_id INTEGER PRIMARY KEY AUTOINCREMENT,
    username VARCHAR(25) NOT NULL UNIQUE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);


CREATE TABLE IF NOT EXISTS user_details
(
    user_id INTEGER PRIMARY KEY,
    phonetic_username VARCHAR(255) NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS user_credentials
(
    user_id INTEGER PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    passphrase VARCHAR(255) NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
);



CREATE TABLE IF NOT EXISTS activity_log 
(   
    log_id BLOB PRIMARY KEY,
    user_id INTEGER,
    subject_user_id INTEGER,
    type_name VARCHAR(100) NOT NULL,
    content TEXT,
    action_timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    -- CONSTRAINT unique_conditional_constraint UNIQUE (user_id, subject_user_id),
    CHECK (type_name = "FOLLOW" OR type_name = "LIKE" OR type_name = "POST"), 
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
);



CREATE TABLE IF NOT EXISTS refreshtokens
(
    user_id INTEGER PRIMARY KEY,
    refresh_token VARCHAR(255),
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
);



CREATE TABLE IF NOT EXISTS subscription
    (
        subscription_id INTEGER PRIMARY KEY AUTOINCREMENT,
        user_id INTEGER,
        subscribed_to_id INTEGER,
        FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE,
        FOREIGN KEY (subscribed_to_id) REFERENCES users(user_id)
);


CREATE TABLE IF NOT EXISTS subscription_details
(
    subscription_detail_id INTEGER PRIMARY KEY AUTOINCREMENT,
    subscription_id INTEGER,
    subscription_date DATETIME DEFAULT CURRENT_TIMESTAMP,
    subscription_duration INTEGER,
    subscription_tier INTEGER,
    FOREIGN KEY (subscription_id) REFERENCES subscription(subscription_id) ON DELETE CASCADE
);




CREATE INDEX idx_users_username ON users (username);

DELETE FROM users;
DELETE FROM user_details;
DELETE FROM user_credentials;
DELETE FROM refreshtokens;
DELETE FROM sqlite_sequence;


DROP TABLE activity_log;
