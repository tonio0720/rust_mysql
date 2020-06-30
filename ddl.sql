CREATE DATABASE IF NOT EXISTS pokemon DEFAULT CHARACTER SET utf8mb4;

USE pokemon;

CREATE TABLE monster (
    monster_id INT NOT NULL,
    name VARCHAR(32) NOT NULL,
    type1_id INT NOT NULL,
    type2_id INT
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4;

CREATE TABLE type (
    type_id INT NOT NULL,
    type_name VARCHAR(32) NOT NULL
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4;

INSERT INTO
    type(type_id, type_name)
VALUES
    (1, 'ノーマル'),
    (2, 'ほのお'),
    (3, 'みず'),
    (4, 'でんき'),
    (5, 'くさ'),
    (6, 'こおり'),
    (7, 'かくとう'),
    (8, 'どく'),
    (9, 'じめん'),
    (10, 'ひこう'),
    (11, 'エスパー'),
    (12, 'むし'),
    (13, 'いわ'),
    (14, 'ゴースト'),
    (15, 'ドラゴン'),
    (16, 'あく'),
    (17, 'はがね'),
    (18, 'フェアリー');

INSERT INTO
    monster(monster_id, name, type1_id, type2_id)
VALUES
    (1, 'フシギダネ', 5, 8),
    (2, 'フシギソウ', 5, 8),
    (3, 'フシギバナ', 5, 8),
    (4, 'ヒトカゲ', 2, NULL),
    (5, 'リザード', 2, NULL),
    (6, 'リザードン', 2, 10),
    (7, 'ゼニガメ', 3, NULL),
    (8, 'カメール', 3, NULL),
    (9, 'カメックス', 3, NULL);