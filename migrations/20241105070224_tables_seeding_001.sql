-- Add migration script here
-- Seed script for initial data

-- Insert a Super Admin user into tbl_admin
INSERT INTO tbl_admin (id, uname, pwd, email, admin_role, admin_stat, cid, mid)
VALUES
  (0, 'superoot', '#02#$argon2id$v=19$m=19456,t=2,p=1$Gbt7IXMSQVeBlUQdsgoyTg$gBCvZr8+8gBwt32eH0dKD9NQn/QRRYzTncMMuWeIKnw', 'super@root.com', 'Super', 'Active', 0, 0);

-- Insert details for the Super Admin in tbl_details
INSERT INTO tbl_details (id, admin_id, first_name, last_name, birth_date, cid, mid)
VALUES
  (0, (SELECT id FROM tbl_admin WHERE uname = 'superoot'), 'Super', 'Admin', '1980-01-01', 0, 0);

-- Insert a record in tbl_permissions for the Super Admin with maximum permission level
INSERT INTO tbl_permissions (id, level, admin_id, cid, mid)
VALUES
  (0, 5, (SELECT id FROM tbl_admin WHERE uname = 'superoot'), 0, 0);


INSERT INTO tbl_roles (role, level)
VALUES 
  ('VIEWER', 0)
  , ('MANAGES USERS', 2)
  , ('MANAGES ADMINS', 3)
  , ('MANAGES BOOKS', 4)
  , ('SUPER ADMIN', 5)
