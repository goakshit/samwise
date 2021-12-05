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