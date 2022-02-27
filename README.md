#### Samwise
Includes series of rust programs through my rust journey.
- ##### Todo
    ToDo cli app that lets us add task to a json file, and keep track of status of completion. 
    ###### Steps
    - Change directory to todo using `cd todo`
    - To add a task, we need to run `cargo run -- add <TASK NAME>`
    - To mark a task as complete, we need to run `cargo run -- complete <TASK NAME>`

-  ##### GrepLite
    GrepLite is another cli app that searches for a pattern in a file or text passed through stdin.
    ###### Steps
    - Change directory to todo using `cd grep-lite`
    - To search for a pattern, we need to run `cargo run -- <PATTERN> <FILE>`

-  ##### Diesel Demo
    DieselDemo is sample cli app to test CRUD using ORM in postgres.
    ###### Steps
    - Change directory to todo using `cd diesel_demo`
    - To search for posts, we need to run `cargo run -bin show_posts`
    - To write a new post, we need to run `cargo run -bin write_post`
    - To update a post, we need to run `cargo run -bin update_post <ID>`
    - To delete a post, we need to run `cargo run -bin delete_post <PATTERN>`