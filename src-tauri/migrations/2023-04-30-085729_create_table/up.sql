-- Your SQL goes here
CREATE TABLE IF NOT EXISTS channel (
    `id` TEXT NOT NULL PRIMARY KEY,
    `title` TEXT NOT NULL DEFAULT "",
    `desc` TEXT NOT NULL DEFAULT "",
    `link` TEXT NOT NULL DEFAULT "",
    `status` INT NOT NULL DEFAULT "",
    `item_count` INT NOT NULL DEFAULT ""
);

CREATE TABLE IF NOT EXISTS feed (
    `id` TEXT NOT NULL PRIMARY KEY,
    `channel_id` TEXT NOT NULL,
    `title` TEXT NOT NULL DEFAULT "",
    `desc` TEXT NOT NULL DEFAULT "",
    `link` TEXT NOT NULL DEFAULT "",
    `pub_date` TEXT NOT NULL DEFAULT "",
    `torrent_link` TEXT NOT NULL DEFAULT "",
    `read_flag` BOOLEAN NOT NULL DEFAULT 0,
    FOREIGN KEY (`channel_id`) REFERENCES channel(`id`) ON DELETE CASCADE ON UPDATE RESTRICT
);

CREATE TABLE IF NOT EXISTS category (
    `id` TEXT NOT NULL PRIMARY KEY,
    `title` TEXT NOT NULL DEFAULT "",
    `desc` TEXT NOT NULL DEFAULT "",
    `link` TEXT NOT NULL DEFAULT ""
);

CREATE TABLE IF NOT EXISTS category_channel (
    `category_id` TEXT NOT NULL,
    `channel_id` TEXT NOT NULL,
    PRIMARY KEY (`category_id`, `channel_id`),
    FOREIGN KEY (`channel_id`) REFERENCES channel(`id`) ON DELETE CASCADE ON UPDATE RESTRICT,
    FOREIGN KEY (`category_id`) REFERENCES category(`id`) ON DELETE CASCADE ON UPDATE RESTRICT
);
