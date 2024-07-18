
#[derive(Queryable, Deserialize, Serialize, Selectable, Debug, Insertable, Iterable, PartialEq)]
#[diesel(table_name = crate::schema::chat_room_participants)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Chat_room_participants {

    pub participant_id: Int4,,

    pub chat_room_id: Int4,,

    pub user_id: Int4,,

    pub is_admin: Bool,,

}

#[derive(Queryable, Deserialize, Serialize, Selectable, Debug, Insertable, Iterable, PartialEq)]
#[diesel(table_name = crate::schema::chat_rooms)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Chat_rooms {

    pub chat_room_id: Int4,,

    pub room_name: Varchar,,

    pub room_description: Varchar,,

    pub chat_room_pubkey: Bytea,,

}

#[derive(Queryable, Deserialize, Serialize, Selectable, Debug, Insertable, Iterable, PartialEq)]
#[diesel(table_name = crate::schema::solana_wallets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Solana_wallets {

    pub wallet_id: Int4,,

    pub user_id: Int4,,

    pub wallet_backup: Bytea,,

    pub wallet_addr: Bytea,,

}

#[derive(Queryable, Deserialize, Serialize, Selectable, Debug, Insertable, Iterable, PartialEq)]
#[diesel(table_name = crate::schema::user_profiles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User_profiles {

    pub user_profile_id: Int4,,

    pub user_id: Int4,,

    pub bio: Varchar,,

    pub profile_picture: Varchar,,

}

#[derive(Queryable, Deserialize, Serialize, Selectable, Debug, Insertable, Iterable, PartialEq)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Users {

    pub user_id: Int4,,

    pub username: Varchar,,

    pub email: Varchar,,

    pub password: Varchar,,

    pub phone_number: Varchar,,

}
