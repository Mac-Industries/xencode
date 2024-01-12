# Xencode 

Xencode is a coding support tool in Rust.

## Features
- Quickly save a coding project idea on the fly using the `xencode idea "idea description here"` command.
- Retrieve saved ideas using the `xencode ideas`

## CLI Commands:

1. Saving Ideas:
- xencode save [idea description]: Saves a new idea with a brief description.
- xencode save -f [file path]: Saves a more detailed idea from a specified file.
- xencode save -a [idea ID]: Appends additional content to an existing idea.

2. Retrieving Ideas:
- xencode list: Lists all saved ideas with their IDs and brief descriptions.
- xencode get [idea ID]: Retrieves the full content of a specific idea.
- xencode search [keyword]: Searches for ideas containing a keyword or phrase.

## Steps
1. Main Function (main.rs):
Set up Clap for command-line argument parsing.
Route the commands (save, list, get, search) to their respective handlers in the commands module.

2. Commands Module (commands/mod.rs):
Define a structure for a coding project idea.
Implement functions for each command (save, list, get, search).

3. Save Command (commands/save.rs):
Handle saving new ideas, appending to existing ideas, and saving from a file.

4. Get Command (commands/get.rs):
Implement functionality to retrieve and display specific ideas by ID.

5. Data Storage:
For simplicity, we can use a simple file-based storage system (like JSON or CSV) to store the ideas.
If needed, we can consider more advanced storage solutions later.

6. Error Handling:
Implement error handling for various scenarios (e.g., file not found, invalid input).