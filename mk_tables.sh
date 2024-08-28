cargo loco generate scaffold letype letype:string --kind htmx
cargo loco generate scaffold marque marque:string --kind htmx
cargo loco generate scaffold famille famille:string --kind htmx
cargo loco generate scaffold depot depot:string --kind htmx
cargo loco generate scaffold centre centre:string --kind htmx
cargo loco generate scaffold marque_oem marque_oem:string --kind htmx
cargo loco generate scaffold contact name:string phone:string email:string --kind htmx

cargo loco generate scaffold famille_mtc code:decimal designation:text pix:string \
letype:references marque:references prix_ttc:decimal famille:references depot:references \
centre:references reference:string oem:string marque_oem:references keywords:string --kind htmx
