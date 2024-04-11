

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

    //let mut vector = vec![];
    let mut vector_new = vec![];
   // let mut sql2 = vec![];
    //let mut sql3 = vec![];
    let tables = [
        Table::new("drinkagelimit", &[("drinkagelimitid", "INTEGER"),("drinkagelimit","INTEGER")]),
        Table::new("orderstatus", &[("orderstatusid", "INTEGER"),("pizza","STRING"),("yoyo","STRING")]),
        // Add more tables as needed
    ];

    let mut sql_string= String::from("");
 //maybe loop 20 times here? 

 
    for table in tables{
        let mut a = 1;
        //loop 20 times here
        while a<=20{
            sql_string.push_str("insert into ");
            sql_string.push_str(format!(r"{}(",table.name).as_str());
           // println!("{}",table.name);
            //println!("{:?}", table.column);
            for item in table.column{
                for i in [item.0].iter(){
                    sql_string.push_str(i);
                    sql_string.push_str(", ");
                   // println!("{}, ",i);
                }
    
            }
            //pops off remaining ", "
            sql_string.pop();
            sql_string.pop();
            //and add a ") "
            sql_string.push_str(r") ");

            sql_string.push_str("values (");
            //add loop here to push all needed values to string
            let mut x = 1;
            //this loop is the same as third loop
            for item in table.column{
                println!("second loop");
                for i in [item.1].iter(){
                 //   println!("item 0 : {}",i);
                    //sql_string.push_str(format!(":{}, ",x).as_str());
                    match i {
                        &"INTEGER"=> {println!("Integer!{}", a); 
                         sql_string.push_str(format!("{}, ",a).as_str());
                        },
                        &"STRING"=> { println!("String!{}{}",item.0,a); sql_string.push_str(format!("'{}{}', ",item.0,a).as_str());},
                    _=>{println!("not here!"); panic!("types not valid, need to be INTEGER or STRING")},
                    }
                x+=1;
                }


            }
            sql_string.pop();
            sql_string.pop();
            //and add a ") "
            sql_string.push_str(r")");

            println!("while loop sql_string: {}", sql_string);
            vector_new.push(sql_string);
            sql_string="".to_string();
            a+=1;
        }
        /*
        let mut y = 1;
        while y<=20 {
            for item in table.column{
                println!("third loop");


                for i in [item.1].iter(){
             
    
                   // println!("item 1 : {}",i);
                    match i {
                         &"INTEGER"=> {println!("Integer!{}", y); sql2.push(format!("{}",y));},
                         &"STRING"=> { println!("String!{}{}",item.0,y); sql2.push(format!("{}{}",item.0,y));},
                         _=>println!("not here!"),
                    }

                    //println!("{:?}",sql2);
                }
            //sql2.push(i.clone());

            }
            sql3.push(sql2.clone());
            sql2 = vec![];

            //println!("yplus");
            y+=1;
            }
*/
        sql_string.pop();
        sql_string.pop();
        sql_string.push_str(r") ");
        sql_string.push_str("\n");
        println!("sql_string complete: {}",sql_string);
       // println!("sql3 complete: {:?}", sql3);
        //vector.push((sql_string,sql3.clone()));
        sql_string = "".to_string();
        //sql3 = vec![];



    }
    //println!("vector: {:?}",vector);
    //println!("---------------");

    //prints vector_new 
    //put execute statement in here
   for item in &vector_new{
        println!("{:?}",item);
       /* for i in item{
            let mut z = 0;
            while z< i.len() {
               // println!("{:?}",i.get(z).unwrap());
                z+=1;
            }
                // println!("{:?}", i);
                //println!("{:?}",i.len());
        }*/
    }

    //data built print it out
    println!("new vector: {:?}", vector_new);

    println!("------------");
    println!("Hello, world!");
}
