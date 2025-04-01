pub struct UserBase{
    fname:String,
}

use bcrypt::{DEFAULT_COST, hash, verify, BcryptError};
use sqlite::{Connection, Error as SqErr, State};
#[derive(Debug )]
pub enum UBaseErr{
    DbErr(SqErr),
    HashError(BcryptError)
}
impl From<SqErr> for UBaseErr{
    fn from(s:SqErr)->Self{
        UBaseErr::DbErr(s)
    }
}
        
impl From<BcryptError> for UBaseErr{
    fn from(b:BcryptError)->Self{
        UBaseErr::HashError(b)
    }
}

impl UserBase{
    pub fn add_user(&self, u_name:&str, p_word:&str)->Result<(),UBaseErr>{
        let conn=sqlite::open(&self.fname)?;
        let hpass=bcrypt::hash(p_word,DEFAULT_COST)?;
        let mut st= conn.prepare("insert into users(u_name, p_word) values (?,?);")?;
        st.bind(1,u_name)?;
        st.bind(2,&hpass as &str)?;
        st.next()?;
        Ok(())
    }

    pub fn pay(&self, u_from:&str, u_to:&str, amount:i64)->Result<(),UBaseErr>{
        let conn=sqlite::open(&self.fname)?;
        let mut st= conn.prepare("insert into transactions (u_from, u_to, t_date,
        t_amount) values(?,?,datetime(\"now\"),?);")?;
        st.bind(1,u_from)?;
        st.bind(2,u_to)?;
        st.bind(3,amount)?;
        st.next()?;
        Ok(())
    }

    pub fn delete_user(&self, u_name:&str)->Result<(),UBaseErr>{
        let conn=sqlite::open(&self.fname)?;
        let mut st= conn.prepare("delete FROM users WHERE u_name = ?;")?;
        st.bind(1,u_name)?;
        st.next()?;
        drop(st);

        let mut st= conn.prepare("delete FROM transactions WHERE u_to = ?;")?;
        st.bind(1,u_name)?;
        st.next()?;
        drop(st);

        let mut st= conn.prepare("delete FROM transactions WHERE u_from = ?;")?;
        st.bind(1,u_name)?;
        Ok(())
    }

    fn check_user(&self, u_name:&str, p_word:&str) -> Result<bool,UBaseErr> {
        let conn=sqlite::open(&self.fname)?;


        let mut statement = conn.prepare("
            SELECT u_name, p_word
            FROM users
            WHERE u_name = ?
        ")?;
        let _ = statement.bind(1, u_name);

        let mut password_hash = String::new();
        while let State::Row = statement.next()? {
            password_hash = statement.read(1)?;
        }

        if verify(p_word, &password_hash).unwrap() {
            Ok(true)
        }
        else {
            Ok(false)
        }
    }

    fn check_statement_occurances(&self, u_from:&str, u_to:&str, t_amount:i64) -> Result<i64,UBaseErr> {
        let conn=sqlite::open(&self.fname)?;

        let mut statement = conn.prepare("
            SELECT COUNT(*)
            FROM transactions
            WHERE u_from = ? AND u_to = ? AND t_amount = ?
        ")?;
        let _ = statement.bind(1, u_from);
        let _ = statement.bind(2, u_to);
        let _ = statement.bind(3,t_amount);

        let mut count = 0;
        if let Ok(row) = statement.next() {
            if row == sqlite::State::Row {
                count = statement.read(0)?;
            }
        }
    
        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Import the main module
    use serial_test::serial; // Import the serial attribute


    #[test]
    #[serial]
    fn test_pay() {
        let db = UserBase { fname: "data/users.db".to_string() };

        assert!(db.add_user("testuser1", "testpassword").is_ok());
        assert!(db.add_user("testuser2", "testpassword").is_ok());

        let count1 = db.check_statement_occurances("testuser1", "testuser2", 1000).unwrap();
        assert!(db.pay("testuser1", "testuser2", 1000).is_ok());
        let count2 = db.check_statement_occurances("testuser1", "testuser2", 1000).unwrap();
        assert!(count1 + 1 == count2);

        assert!(db.delete_user("testuser1").is_ok());
        assert!(db.delete_user("testuser2").is_ok());
    }

    #[test]
    #[serial]
    fn test_add() {
        let db = UserBase { fname: "data/users.db".to_string() };

        assert!(db.add_user("testuser1", "testpassword").is_ok());
        assert!(db.add_user("testuser2", "testpassword").is_ok());

        assert!(db.check_user("testuser1", "testpassword").unwrap());
        assert!(db.check_user("testuser2", "testpassword").unwrap());

        assert!(db.delete_user("testuser1").is_ok());
        assert!(db.delete_user("testuser2").is_ok());
    }
}
        
fn main() {

}
    