# RecSys as a Service

## TODO
🔴 -> Important
🟠 -> Can wait
🟢 -> Nice to have

### Auth
* [X] Get token from header bearer for API requests 🔴
* [X] Get token from request body for script tag use 🔴

### Data
* [X] Connect to DB 🔴
* [ ] Connect to cache (redis) 🟢

#### Migrations
* [ ] Create CLI commands 🟢
* [ ] Generalise migrations so it can be used with any DB 🟢
* [ ] Create migrations based on past and current models states 🟢

#### ORM
* [ ] Improve custom ORM 🟢
* [ ] Add ORM as trait 🟢

### Recommendations
* [X] Finish the get_product_recommendations 🔴
* [X] Finish recommendation struct 🔴
* [X] Save recommendations made 🔴
* [X] Create view to redirect to correct product on click 🔴
* [X] Save metadata of clicks and redirects 🔴
* [X] Ensure that the ULIDs are truly unique (query the database) 🔴
* [ ] Finish the get_generic_recommendations 🟠
* [ ] Finish the get_user_recommendations 🟠

### Models
* [ ] Convert the correct fields/information to vectors 🟠
* [ ] Improve tables relationship to retreive correct information 🟠
* [ ] Create facades between raw model and views' model 🟠

### Views
* [X] Mapp CrudErrors with default views' responses 🟠