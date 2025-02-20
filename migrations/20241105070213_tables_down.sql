-- Add migration script here

-- Tables
DROP TABLE IF EXISTS tbl_acc_removed;
DROP TABLE IF EXISTS tbl_acc_session;
DROP TABLE IF EXISTS tbl_categories;
DROP TABLE IF EXISTS tbl_category;
DROP TABLE IF EXISTS tbl_details;
DROP TABLE IF EXISTS tbl_permissions;
DROP TABLE IF EXISTS tbl_removed_acc_history;
DROP TABLE IF EXISTS tbl_admin;
DROP TABLE IF EXISTS tbl_book;

-- Types

DROP TYPE IF EXISTS session_typ;
DROP TYPE IF EXISTS admin_typ;
DROP TYPE IF EXISTS admin_stat;
