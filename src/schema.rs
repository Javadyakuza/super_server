diesel::table! {
    chat_room_participants (participant_id) {
        participant_id -> Int4,
        chat_room_id -> Int4,
        user_id -> Int4,
        is_admin -> Bool,
    }
}

diesel::table! {
    chat_rooms (chat_room_id) {
        chat_room_id -> Int4,
        #[max_length = 255]
        room_name -> Varchar,
        #[max_length = 255]
        room_description -> Varchar,
        chat_room_pubkey -> Bytea,
    }
}

diesel::table! {
    solana_wallets (wallet_id) {
        wallet_id -> Int4,
        user_id -> Int4,
        wallet_backup -> Bytea,
        wallet_addr -> Bytea,
    }
}

diesel::table! {
    user_profiles (user_profile_id) {
        user_profile_id -> Int4,
        user_id -> Int4,
        #[max_length = 255]
        bio -> Nullable<Varchar>,
        #[max_length = 255]
        profile_picture -> Nullable<Varchar>,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 15]
        phone_number -> Varchar,
    }
}


