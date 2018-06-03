table! {
    articles (post_id) {
        title -> Text,
        post_id -> Integer,
        friendly_url -> Text,
        publish_date -> Integer,
        expiration_date -> Integer,
        structure_id -> Integer,
        content -> Text,
        folder_id -> Integer,
        is_trash -> Bool,
        is_draft -> Bool,
        create_date -> Integer,
        modified_date -> Integer,
        create_user_id -> Integer,
        modified_user_id -> Integer,
        site_id -> Integer,
    }
}

table! {
    files (file_id) {
        file_hotlink_url -> Text,
        file_title -> Text,
        file_id -> Integer,
        file_type_id -> Integer,
        site_id -> Integer,
        folder_id -> Integer,
    }
}

table! {
    pages (page_id) {
        site_id -> Integer,
        page_id -> Integer,
        title -> Text,
        friendly_url -> Text,
        parent_page_id -> Nullable<Integer>,
        additional_javascript -> Text,
        additional_css -> Text,
    }
}

table! {
    portlets (portlet_id) {
        portlet_id -> Integer,
        page_id -> Integer,
        page_internal_path -> Text,
        portlet_type -> Text,
        title -> Text,
        config -> Text,
    }
}

table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        published -> Bool,
    }
}

table! {
    sites (site_id) {
        site_id -> Integer,
        parent_site_id -> Nullable<Integer>,
        friendly_url -> Text,
        theme_id -> Integer,
        additional_javascript -> Text,
        additional_css -> Text,
        title -> Text,
    }
}

table! {
    structures (structure_id) {
        title -> Text,
        structure_id -> Integer,
        publish_date -> Integer,
        expiration_date -> Integer,
        jsonStructure -> Text,
        parent_structure_id -> Integer,
        is_trash -> Bool,
        is_draft -> Bool,
        create_date -> Integer,
        modified_date -> Integer,
        create_user_id -> Integer,
        modified_user_id -> Integer,
        site_id -> Integer,
    }
}

table! {
    templates (template_id) {
        template_id -> Integer,
        site_id -> Integer,
    }
}

table! {
    users (user_id) {
        user_id -> Integer,
        screen_name -> Text,
        first_name -> Text,
        last_name -> Text,
        anrede -> Text,
        geburtstag -> Integer,
        email -> Text,
        portrait -> Nullable<Integer>,
        original_site_id -> Integer,
    }
}

table! {
    vocabulary (vocabulary_id) {
        vocabulary_id -> Integer,
        title -> Text,
    }
}

table! {
    vocabulary_entry (vocabulary_entry_id) {
        vocabulary_entry_id -> Integer,
        title -> Text,
        vocabulary_id -> Integer,
        parent_vocabulary_entry_id -> Nullable<Integer>,
    }
}

joinable!(articles -> sites (site_id));
joinable!(files -> sites (site_id));
joinable!(pages -> sites (site_id));
joinable!(portlets -> pages (page_id));
joinable!(structures -> sites (site_id));
joinable!(templates -> sites (site_id));
joinable!(vocabulary_entry -> vocabulary (vocabulary_id));

allow_tables_to_appear_in_same_query!(
    articles,
    files,
    pages,
    portlets,
    posts,
    sites,
    structures,
    templates,
    users,
    vocabulary,
    vocabulary_entry,
);
