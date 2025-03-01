use surrealdb::engine::any::Any;
use surrealdb::{Error, Surreal};

pub async fn run_query(db: &Surreal<Any>) -> Result<(), Error> {
    let query = r#"

    
    DEFINE TABLE security_type SCHEMAFULL;
    DEFINE FIELD name ON security_type TYPE string;
    CREATE security_type:PET SET name = 'Equity';
    CREATE security_type:PLI SET name = 'Bond';
    CREATE security_type:FUT SET name = 'Future';
   

    DEFINE TABLE entity SCHEMAFULL;
    DEFINE FIELD name ON entity TYPE string;     
    INSERT INTO entity (id, name) VALUES ('SGPM', 'Soc Gen Paris');
    INSERT INTO entity (id, name) VALUES ('SGAS', 'Soc Gen America');
    INSERT INTO entity (id, name) VALUES ('SGIL', 'Soc Gen London');

    DEFINE TABLE book SCHEMAFULL;
    DEFINE FIELD entity ON book TYPE record<entity>;
    DEFINE FIELD active ON book TYPE bool;
    CREATE book:BTBSH SET entity = entity:SGPM, active = true;
    CREATE book:PBSBL SET entity = entity:SGAS, active = true;
    CREATE book:SLPHK SET entity = entity:SGIL, active = true;
    CREATE book:BTFXS SET entity = entity:SGPM, active = true;
    CREATE book:PBEQT SET entity = entity:SGAS, active = true;
    CREATE book:SLFUT SET entity = entity:SGIL, active = false;


    DEFINE TABLE security SCHEMAFULL;    
    DEFINE FIELD security_type ON security TYPE record<security_type>;
    CREATE security:TOTAL SET security_type = security_type:PET;
    CREATE security:AMUNDI SET security_type = security_type:PLI;
    CREATE security:AMD SET security_type = security_type:FUT;
    CREATE security:SOCGEN SET security_type = security_type:PET;
    CREATE security:UBISOFT SET security_type = security_type:PET;
    CREATE security:ARCELLOR SET security_type = security_type:PET;
    CREATE security:OAT SET security_type = security_type:PLI;


    DEFINE TABLE trade SCHEMAFULL;    
    DEFINE FIELD book ON trade TYPE record<book>;    
    DEFINE FIELD security ON trade TYPE record<security>;
    DEFINE FIELD quantity ON trade TYPE number;
    DEFINE FIELD price ON trade TYPE number;
    DEFINE FIELD date ON trade TYPE datetime;    
    DEFINE FIELD active ON trade TYPE bool;

    CREATE trade SET book = book:BTBSH, security = security:TOTAL, quantity = 100, price = 100, date = <datetime>"2025-02-07", active = true;
    CREATE trade SET book = book:PBSBL, security = security:AMUNDI, quantity = 200, price = 200, date = <datetime>"2025-02-07", active = true;
    CREATE trade SET book = book:SLPHK, security = security:AMD, quantity = 100, price = 300, date = <datetime>"2025-02-07", active = false;
    CREATE trade SET book = book:SLPHK, security = security:SOCGEN, quantity = 100, price = 300, date = <datetime>"2025-02-07", active = false;
    CREATE trade SET book = book:SLPHK, security = security:UBISOFT, quantity = 100, price = 300, date = <datetime>"2025-02-07", active = false;
    CREATE trade SET book = book:SLPHK, security = security:ARCELLOR, quantity = 100, price = 300, date = <datetime>"2025-02-07", active = false;
    CREATE trade SET book = book:SLPHK, security = security:OAT, quantity = 100, price = 300, date = <datetime>"2025-02-07", active = false;

    


    "#;

    let response = db.query(query).await?;
    println!("Query executed successfully. Response: {:?}", response);

    Ok(())
}
