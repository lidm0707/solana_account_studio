//! # Database Schema
//!
//! This module defines the database schema for the SurfDesk application.
//! It includes tables for projects, environments, accounts, and other
//! core entities using Diesel ORM.


diesel::table! {
    projects (id) {
        id -> Text,
        name -> Text,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        config -> Nullable<Json>,
        status -> Text,
        owner -> Nullable<Text>,
        tags -> Nullable<Text>,
    }
}

diesel::table! {
    environments (id) {
        id -> Text,
        project_id -> Text,
        name -> Text,
        type_ -> Text,
        config -> Json,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        rpc_url -> Text,
        ws_url -> Nullable<Text>,
        is_default -> Bool,
        priority -> Integer,
    }
}

diesel::table! {
    accounts (id) {
        id -> Text,
        environment_id -> Text,
        pubkey -> Text,
        account_data -> Json,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        label -> Nullable<Text>,
        account_type -> Text,
        is_watched -> Bool,
        balance -> Nullable<BigInt>,
        owner -> Nullable<Text>,
        executable -> Bool,
        rent_epoch -> Nullable<BigInt>,
    }
}

diesel::table! {
    transactions (id) {
        id -> Text,
        environment_id -> Text,
        signature -> Text,
        transaction_data -> Json,
        status -> Text,
        created_at -> Timestamp,
        confirmed_at -> Nullable<Timestamp>,
        slot -> Nullable<BigInt>,
        block_height -> Nullable<BigInt>,
        fee -> Nullable<BigInt>,
        error -> Nullable<Text>,
        transaction_type -> Text,
        accounts -> Nullable<Text>,
        memo -> Nullable<Text>,
    }
}

diesel::table! {
    programs (id) {
        id -> Text,
        environment_id -> Text,
        program_id -> Text,
        name -> Text,
        description -> Nullable<Text>,
        path -> Nullable<Text>,
        version -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        idl -> Nullable<Json>,
        upgrade_authority -> Nullable<Text>,
        is_deployed -> Bool,
        is_verified -> Bool,
    }
}

diesel::table! {
    settings (id) {
        id -> Text,
        key -> Text,
        value -> Json,
        category -> Text,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_user_configurable -> Bool,
        data_type -> Text,
        validation -> Nullable<Json>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    projects,
    environments,
    accounts,
    transactions,
    programs,
    settings,
);
