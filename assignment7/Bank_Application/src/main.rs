use std::sync::Arc;
use std::sync::Mutex;
use std::vec;
use std::thread;

#[derive(Clone,Debug)]
struct Bank{
    accounts:Arc<Mutex<Vec<i32>>>
}

impl Bank{
    fn new(n:usize)->Self{
        let mut v = Vec::with_capacity(n);
        for _ in 0..n{
            v.push(0);
        }
        Bank{
            accounts:Arc::new(Mutex::new(v)),
        }
    }

    fn transfer(&self, from:usize, to:usize, amount:i32)->Result<(),()>{

        let mut accounts = self.accounts.lock().unwrap();
        let from_balance = accounts.get_mut(from);
        
        match from_balance {
            Some(from_balance) => {
                if *from_balance >= amount {
                    *from_balance -= amount;

                    let to_balance = accounts.get_mut(to);
                    match to_balance {
                        Some(to_balance) => {
                            *to_balance = *to_balance + amount;
                            println!("Amount of {} transferred from account id: {} to account id: {}", amount, from, to);
                        },
                        None => {
                            return Err(())
                        },
                    }
                }
                else {
                    println!("Insufficient balance");
                }
            },
            None => {
                return Err(())
            },
        }

        Ok(())
    }


}

struct Person{
    ac_id:usize,
    buddy_id:usize,
}

impl Person{
    pub fn new(id:usize,b_id:usize)->Self{
        Person{
            ac_id:id,
            buddy_id:b_id
        }
    }
}
    
fn test_run() {
    let bank = Arc::new(Bank { accounts: Arc::new(Mutex::new(Vec::from([1, 1, 1, 1, 1, 1, 1, 1, 1, 1]))) });
    let mut people: Vec<Person> = Vec::new();
    people.push(Person::new(0, 1));

    for i in 1..10 {
        people.push(Person::new(i, 0));
    }

    let mut threads = Vec::new();

    for i in people {
        let person = Arc::new(i);
        let ba: Arc<Bank> = bank.clone();

        let handle = thread::spawn(move || {
            ba.transfer(person.ac_id, person.buddy_id, 1).unwrap();
        });

        threads.push(handle);
    }

    for i in threads {
        i.join().unwrap();
    }
}

fn main() {
    test_run();
}
