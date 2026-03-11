-- 用户表
CREATE TABLE user
(
    user_id     INTEGER                                                     NOT NULL PRIMARY KEY AUTOINCREMENT,
    username    VARCHAR(10)                                                 NOT NULL
        CONSTRAINT pk_user_username UNIQUE,
    pwd_hash    VARCHAR(100)                                                NOT NULL,
    create_time DATETIME DEFAULT (DATETIME(CURRENT_TIMESTAMP, 'localtime')) NOT NULL,
    update_time DATETIME DEFAULT (DATETIME(CURRENT_TIMESTAMP, 'localtime')) NOT NULL
);
CREATE INDEX idx_user_username_pwd_hash ON user (username, pwd_hash);
