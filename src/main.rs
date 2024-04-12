use std::io::stdin;
use std::io::prelude::*;
use oracle::{Connection, Error};

struct Table<'a>{
    name: &'a str,
    column: &'a [(&'a str,&'a str)],
}

impl<'a> Table<'a> {
    fn new(name: &'a str, column: &'a [(&'a str, &'a str)]) -> Self {
        Table { name, column }
    }
}

fn main() {
    let conn = Connection::connect("student","student","//localhost:41521/xe").expect("connection error");
    let mut vector_new = vec![];
    let tables = [
        Table::new("drinkagelimit", &[("drinkagelimitid", "INTEGER"),("agelimit","INTEGER")]),
        Table::new("alcoholrestrictions", &[("alcoholrestrictionid", "INTEGER"),("alcoholrestriction", "STRING"),("drinkagelimitid","INTEGER")]),
        Table::new("orderstatus", &[("orderstatusid", "INTEGER"),("status","STRING")]),
        Table::new("customer", &[("phonenumber", "STRING"),("customerid","INTEGER"),("firstname","STRING"),("lastname","STRING"),("email","STRING"),("rewardpoints","INTEGER")]),
        Table::new("menuitem", &[("name","STRING"),("menuitemid", "INTEGER"), ("cost", "DOUBLE")]),
        Table::new("menu", &[("menuid", "INTEGER"),("menuitemid", "INTEGER")]),
        Table::new("salestax", &[("salestaxid", "INTEGER"), ("salestaxamount", "DOUBLE")]),
        Table::new("inventory", &[("itemlocation", "STRING"),("itemquantity","INTEGER"),("inventoryid","INTEGER")]),
        Table::new("ingredients",&[("name","STRING"),("inventoryid", "INTEGER"),("cost", "DOUBLE"),("ingredientid", "INTEGER")]),
        Table::new("gratuity", &[("amount", "DOUBLE"), ("customerid","INTEGER"),("gratuityid", "INTEGER")]),
        Table::new("diningtable", &[("tableid", "INTEGER"), ("tablecapacity", "INTEGER")]),
        Table::new("tablemap",&[("tablemapid", "INTEGER"),("tableid", "INTEGER"),("location","STRING")]),
        Table::new("tableavailability", &[("tableavailabilityid", "INTEGER"), ("tablemapid", "INTEGER"), ("reservationtime", "TIMESTAMP")]),
        Table::new("reservationqueue", &[("customerid", "INTEGER"), ("partysize", "INTEGER"), ("timestamp", "TIMESTAMP"), ("reservationid", "INTEGER"), ("tableavailabilityid", "INTEGER")]),
        Table::new("seatat", &[("tableid", "INTEGER"),("customerid", "INTEGER"),("timestamp", "TIMESTAMP"),("seatatid", "INTEGER")]),
        Table::new("eats", &[("customerid", "INTEGER"),("menuitemid", "INTEGER"),("eatsid","INTEGER")]),
        Table::new("orders", &[("orderid", "INTEGER"),("timestamp", "TIMESTAMP"),("orderstatusid","INTEGER"),("alcoholrestrictionid", "INTEGER")]),
        Table::new("payments", &[("paymentid","INTEGER"), ("paymentamount", "DOUBLE"), ("orderid", "INTEGER"),("gratuityid", "INTEGER"), ("salestaxid", "INTEGER"), ("timestamp","TIMESTAMP")]),
        Table::new("transactions", &[("transactionid", "INTEGER"), ("timestamp", "TIMESTAMP"),("paymentid", "INTEGER")]),
        Table::new("tablimit", &[("tablimitamount", "DOUBLE"), ("tablimitid", "INTEGER")]),
        Table::new("barordertab", &[("ordertotal", "DOUBLE"),("tabid","INTEGER"),("orderid","INTEGER"),("tablimitid","INTEGER"),("paymentid","INTEGER")]),
        Table::new("logs", &[("timestamp","TIMESTAMP"),("errorcode", "INTEGER"),("eventdescription","STRING"),("logid","INTEGER"), ("orderid","INTEGER"),("transactionid", "INTEGER"),("paymentid", "INTEGER")]),
        Table::new("shift", &[("shiftid", "INTEGER"),("starttime","TIMESTAMP"),("endtime","TIMESTAMP")]),
        Table::new("employee", &[("firstname","STRING"),("lastname","STRING"),("employeeid","INTEGER")]),
        Table::new("waitson", &[("timestamp","TIMESTAMP"),("customerid", "INTEGER"),("employeeid","INTEGER"),("waitsonid","INTEGER")]),
        Table::new("schedule", &[("employeeid","INTEGER"),("shiftid","INTEGER"),("scheduleid","INTEGER")]),
        Table::new("employeeidentify", &[("employeeidentifyid","INTEGER"),("employeeid", "INTEGER"),("timestamp","TIMESTAMP")]),
        Table::new("authentication", &[("authenticationid", "INTEGER"),("employeeidentifyid","INTEGER"),("password","STRING"),("username","STRING")]),
        Table::new("role", &[("role","STRING"),("roleid", "INTEGER"),("authenticationid", "INTEGER")]),
        Table::new("fulfillrole", &[("employeeid", "INTEGER"),("roleid", "INTEGER"), ("timestamp", "TIMESTAMP"), ("fulfillroleid", "INTEGER")]),
        Table::new("stock", &[("stockquantity", "INTEGER"), ("timestamp","TIMESTAMP"),("employeeid","INTEGER"),("stockid", "INTEGER"),("ingredientid", "INTEGER")]),
        Table::new("cook", &[("cookid", "INTEGER"),("ingredientid", "INTEGER"),("menuitemid","INTEGER")]),
        Table::new("submitorder", &[("submitorderid", "INTEGER"), ("timestamp", "TIMESTAMP"),("employeeid", "INTEGER"), ("orderid", "INTEGER")]),
        Table::new("serve", &[("timestamp", "TIMESTAMP"),("employeeid", "INTEGER"),("orderid", "INTEGER"),("serveid", "INTEGER")]),
        Table::new("demand", &[("orderid", "INTEGER"),("demandid", "INTEGER"),("menuitemid", "INTEGER"),("quantity", "INTEGER")]),
        Table::new("requestorder",&[("requestid", "INTEGER"),("customerid", "INTEGER"),("orderid","INTEGER")]),
        // Add more tables as needed
    ];

    let mut sql_string= String::from("");
    for table in tables{
        let mut a = 0;
        //loop 20 times here
        while a<=20{
            sql_string.push_str("insert into ");
            sql_string.push_str(format!(r"{}(",table.name).as_str());
            for item in table.column{
                    sql_string.push_str(item.0);
                    sql_string.push_str(", ");
            }
            //pops off remaining ", "
            sql_string.pop();
            sql_string.pop();
            //and add a ") "
            sql_string.push_str(r") ");
            sql_string.push_str("values (");
            for item in table.column{
                    //matches i to either INTEGER or STRING and pushes values to sql_string
                    match item.1 {
                        "INTEGER"=> {sql_string.push_str(format!("{}, ",a).as_str());},
                        "DOUBLE"=> {sql_string.push_str(format!("{:.2}, ",a as f64).as_str());},
                        "TIMESTAMP"=>{sql_string.push_str(format!("'2024-02-28T00:35:58Z20240228T003558Z', ").as_str());},
                        "STRING"=> { sql_string.push_str(format!("'{}{}', ",item.0,a).as_str());},
                    _=>{ panic!("types not valid, need to be INTEGER or STRING")},
                    }
            }
            //pops off remaining ", "
            sql_string.pop();
            sql_string.pop();
            //and add a ") "
            sql_string.push_str(r")");
            //pushes to vector to loop over later
            vector_new.push(sql_string);
            //resets string for next loop
            sql_string="".to_string();
            //increments counter to 20
            a+=1;
        }
    }
    //prints vector_new 
   for item in &vector_new{
        println!("{:?}",item);
        conn.execute(format!("{}",item).as_str(), &[]).expect("sql error");
    }
    //now we have a sweet vector, we need to iterate over it and run all the sql statements
    //then add a commit statement and get user confirmation
    println!("Would you like to commit? Y or N");
    let mut s = "".to_string();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    let binding = s.to_string();
    let trim_s = binding.trim();
    
    match trim_s.to_uppercase().as_str() {
        "Y"=>{  println!("Commiting queries");
        conn.commit();
        println!("Commited! Goodbye.");
        },
        _ => println!("Goodbye!"),
    }
}
