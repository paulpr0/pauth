pub mod pauth {
    table! {
        pauth.auth_history (id) {
            id -> Int4,
            user_id -> Nullable<Int4>,
            auth_time -> Nullable<Timestamp>,
            source -> Nullable<Int4>,
            route -> Nullable<Text>,
        }
    }

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
        pauth.login_history (id) {
            id -> Int4,
            user_id -> Nullable<Int4>,
            login_time -> Nullable<Timestamp>,
            source -> Nullable<Int4>,
            route -> Nullable<Text>,
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
        pauth.source (id) {
            id -> Int4,
            ip -> Nullable<Inet>,
            mac -> Nullable<Macaddr>,
            identifier -> Nullable<Text>,
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
        pauth.user_history (id) {
            id -> Int4,
            user_id -> Nullable<Int4>,
            change_time -> Nullable<Timestamp>,
            old_chosen_name -> Nullable<Varchar>,
            old_email -> Nullable<Varchar>,
            old_pass -> Nullable<Text>,
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

    joinable!(auth_history -> source (source));
    joinable!(auth_history -> users (user_id));
    joinable!(default_config -> config (config_id));
    joinable!(login_history -> source (source));
    joinable!(login_history -> users (user_id));
    joinable!(pw_reset -> users (user_id));
    joinable!(user_config -> config (config_id));
    joinable!(user_config -> users (user_id));
    joinable!(user_history -> users (user_id));
    joinable!(user_login_tokens -> users (user_id));

    allow_tables_to_appear_in_same_query!(
        auth_history,
        config,
        default_config,
        login_history,
        pw_reset,
        source,
        user_config,
        user_history,
        user_login_tokens,
        users,
    );
}
