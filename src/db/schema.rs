diesel::table! {
    blogs (id) {
        id      -> Integer,
        title   -> Text,
        date    -> Text,
        preview -> Text,
        body    -> Text,
    }
}