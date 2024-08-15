/*

CREATE TABLE themes (
video_id INTEGER UNIQUE NOT NULL,
basename TEXT NOT NULL,
link TEXT UNIQUE NOT NULL,
views_count INTEGER NOT NULL,
anime_id INTEGER NOT NULL,
anime_name TEXT NOT NULL,
anime_year INTEGER NOT NULL
);*/

struct Theme {
    video_id: usize,
    basename: String,
    link: String,
    views_count: usize,
    anime_id: usize,
    anime_name: String,
    anime_year: usize
}