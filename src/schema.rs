pub mod pauth {
    table! {
        pauth.pw_reset (id) {
            id -> Int4,
            user_id -> Int4,
            user_token_hash -> Text,
            expires -> Nullable<Timestamp>,
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

    joinable!(pw_reset -> users (user_id));
    joinable!(user_login_tokens -> users (user_id));

    allow_tables_to_appear_in_same_query!(
        pw_reset,
        user_login_tokens,
        users,
    );
}
