


#[derive(Debug)]

pub struct BankAccount {


    balance: f64,


}


impl BankAccount {





    // create a new accont





    pub fn new(initial_balance: f64) -> BankAccount {



        BankAccount {


            balance: initial_balance


        }


    }

    // deposit money


    pub fn deposit(&mut self, amount: f64) {


