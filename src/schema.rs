pub mod pauth {
    table! {
        pauth.config (id) {
            id -> Int4,
            config_key -> Nullable<Varchar>,
            config_value -> Nullable<Varchar>,
        }
    }

    table! {
        pauth.default_config (id) {
            id -> Int4,
            config_id -> Nullable<Int4>,
        }
    }

    table! {
        pauth.pw_reset (id) {
            id -> Int4,
            user_id -> Int4,
            user_token_hash -> Text,
            expires -> Nullable<Timestamp>,
        }
    }

    table! {
        pauth.user_config (id) {
            id -> Int4,
            user_id -> Nullable<Int4>,
            config_id -> Nullable<Int4>,
        }
    }

    table! {
        pauth.user_login_tokens (id) {
            id -> Int4,
            user_id -> Int4,
            token -> Text,
            created -> Timestamp,
            last_used -> Timestamp,
        }
    }

    table! {
        pauth.users (id) {
            id -> Int4,
            chosen_name -> Varchar,
            email -> Varchar,
            pass_hash -> Text,
            last_login -> Timestamp,
        }
    }

    joinable!(default_config -> config (config_id));
    joinable!(pw_reset -> users (user_id));
    joinable!(user_config -> config (config_id));
    joinable!(user_config -> users (user_id));
    joinable!(user_login_tokens -> users (user_id));

    allow_tables_to_appear_in_same_query!(
        config,
        default_config,
        pw_reset,
        user_config,
        user_login_tokens,
        users,
    );
}
