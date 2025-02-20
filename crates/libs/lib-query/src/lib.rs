pub mod admin;
pub mod general;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_query() {
        let sql = admin::insert();
        let expected = "INSERT INTO tbl_admin (uname, email) VALUES ($1, $2) RETURNING id";
        assert_eq!(sql, expected);
    }

    #[test]
    fn test_get_by_uname_query() {
        let sql = admin::get_by_uname();
        let expected =
            "SELECT id, uname, email, pwd, pwd_salt, token_salt FROM tbl_admin WHERE uname = $1";
        assert_eq!(sql, expected);
    }

    #[test]
    fn test_select_all_query_without_id() {
        let sql = admin::select_all(false);
        let expected = "SELECT id, uname, email, pwd_salt, token_salt, admin_role, session_typ, admin_stat, cid, ctime, mid, mtime FROM tbl_admin";
        assert_eq!(sql, expected);
    }

    #[test]
    fn test_select_all_query_with_id() {
        let sql = admin::select_all(true);
        let expected = "SELECT id, uname, email, pwd_salt, token_salt, admin_role, session_typ, admin_stat, cid, ctime, mid, mtime FROM tbl_admin WHERE id = $1";
        assert_eq!(sql, expected);
    }

    #[test]
    fn test_update_query() {
        let sql = admin::update();
        let expected = "UPDATE tbl_admin SET uname=$1, email=$2, mtime=NOW() WHERE id=$3";
        assert_eq!(sql, expected);
    }

    #[test]
    fn test_check_removed_acc_query() {
        let sql = admin::check_removed_acc();
        let expected = "SELECT id, uname FROM tbl_acc_removed WHERE uname=$1, email=$2";
        assert_eq!(sql, expected);
    }

    #[test]
    fn test_update_pass_query() {
        let sql = admin::update_pass();
        let expected = "UPDATE tbl_admin SET pwd=$1, mtime=NOW() WHERE id=$2";
        assert_eq!(sql, expected);
    }

    #[test]
    fn test_removed_query() {
        let sql = admin::removed();
        let expected =
            "INSERT INTO tbl_acc_removed (admin_id, cid, mid) VALUES ($1, $2, $3) RETURNING id";
        assert_eq!(sql, expected);
    }

    #[test]
    fn test_general_insert_cid_mid_query() {
        let sql = general::general_insert_cid_mid("tbl_example");
        let expected = "UPDATE tbl_example SET cid=$1, mid=$2 WHERE id = $3";
        assert_eq!(sql, expected);
    }
}
