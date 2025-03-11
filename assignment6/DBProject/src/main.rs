use std::thread::current;
use std::env;
use bcrypt::{DEFAULT_COST, hash, verify, BcryptError};
use sqlite::{Connection, Error as SqErr, State};
use chrono::Utc;
use std::io::{self, Write};

pub struct UserBase{
    fname:String,
}

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

    pub fn pay(&self, u_from:&str, u_to:&str, amount:i64)->Result<(),UBaseErr>{
        let conn=sqlite::open(&self.fname)?;

        let current_balance = self.get_balance(u_from, &conn)?;


        if current_balance >= amount {
            let mut st= conn.prepare("insert into transactions (u_from, u_to, t_date, t_amount) values(?,?,datetime(\"now\"),?);")?;
            st.bind(1,u_from)?;
            st.bind(2,u_to)?;
            st.bind(3,amount)?;
            st.next()?;
        }
        else {
            print!("Too poor\n");
        }
        Ok(())
    }

    fn get_transactions_history(&self, u_name:&str) -> Result<(),UBaseErr> {
        let conn=sqlite::open(&self.fname)?;

        let mut statement = conn.prepare("
            SELECT u_from, u_to, t_date, t_amount
            FROM transactions
            WHERE u_from = ?
        ")?;
        let _ = statement.bind(1, u_name);
        while let State::Row = statement.next()? {
            let sender: String = statement.read(0)?;
            let reciever: String = statement.read(1)?;
            let date: i64 = statement.read(2)?;
            let amount: i64 = statement.read(3)?;

            let datetime = Utc::now();

            println!("{} sent ${} to {} on {}", sender, amount, reciever, datetime);
        }

        let mut statement = conn.prepare("
            SELECT u_from, u_to, t_date, t_amount
            FROM transactions
            WHERE u_to = ?
        ")?;
        let _ = statement.bind(1, u_name);
        while let State::Row = statement.next()? {
            let sender: String = statement.read(0)?;
            let reciever: String = statement.read(1)?;
            let date: i64 = statement.read(2)?;
            let amount: i64 = statement.read(3)?;

            let datetime = Utc::now();

            println!("{} recieved ${} from {} on {}", reciever, amount, sender, datetime);
        }

        Ok(())
    }

    fn get_balance(&self, u_name:&str, conn: &Connection) -> Result<i64,UBaseErr> {
        let mut balance  = 1000;

        let mut statement = conn.prepare("
            SELECT t_amount
            FROM transactions
            WHERE u_from = ?
        ")?;
        let _ = statement.bind(1, u_name);
        while let State::Row = statement.next()? {
            let amount: i64 = statement.read(0)?;
            balance -= amount;
        }

        let mut statement = conn.prepare("
            SELECT t_amount
            FROM transactions
            WHERE u_to = ?
        ")?;
        let _ = statement.bind(1, u_name);
        while let State::Row = statement.next()? {
            let amount: i64 = statement.read(0)?;
            balance += amount;
        }

        Ok(balance)
    }

    fn get_password(&self, u_name:&str) -> Result<i64,UBaseErr> {
        let conn=sqlite::open(&self.fname)?;
        
        let mut balance  = 1000;

        let mut statement = conn.prepare("
            SELECT t_amount
            FROM transactions
            WHERE u_from = ?
        ")?;
        let _ = statement.bind(1, u_name);
        while let State::Row = statement.next()? {
            let amount: i64 = statement.read(0)?;
            balance -= amount;
        }

        let mut statement = conn.prepare("
            SELECT t_amount
            FROM transactions
            WHERE u_to = ?
        ")?;
        let _ = statement.bind(1, u_name);
        while let State::Row = statement.next()? {
            let amount: i64 = statement.read(0)?;
            balance += amount;
        }

        Ok(balance)
    }

}

fn test_pay() {
    let db = UserBase {fname: "data/users.db".to_string()};
    db.add_user("testuser1", "testpassword").unwrap();
    db.add_user("testuser2", "testpassword").unwrap();
    db.pay("testuser1", "testuser2", 1000).unwrap();
    db.pay("testuser1", "testuser2", 1000).unwrap();
    db.delete_user("testuser1").unwrap();
    db.delete_user("testuser2").unwrap();
}

fn test_history() {
    let db = UserBase {fname: "data/users.db".to_string()};
    db.add_user("testuser1", "testpassword").unwrap();
    db.add_user("testuser2", "testpassword").unwrap();
    db.pay("testuser1", "testuser2", 1000).unwrap();
    db.pay("testuser2", "testuser1", 500).unwrap();
    db.get_transactions_history("testuser1").unwrap();
    db.delete_user("testuser1").unwrap();
    db.delete_user("testuser2").unwrap();
}

fn prompt_password() -> bool {
    let mut input = String::new();
    print!("Password: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();
    


    return true;
}

// Should return:
// Too poor
// testuser1 sent $1000 to testuser2 on [close to current time] UTC
// testuser1 recieved $500 from testuser2 on [close to current time] UTC
// 
fn main() {
    let args: Vec<String> = env::args().collect();

    let db = UserBase {fname: "data/users.db".to_string()};

    if args[1] == "new" {
        db.add_user(&args[2], &args[3]).unwrap();
    }
    else if args[1] == "transfer" {
        println!("transfer money");
    }
    else if args[1] == "balance" {
        println!("get amount");
    }
}