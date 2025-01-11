# postman-github-actions-example

A very simple Rust Application to show how to use Postman within Github Actions.
Project contains all files needed

### Postman Desktop App
> Used Postman Desktop Application to export a collection.json file

### Github Actions
The workflow for Github Actions contain the following steps;
- Start the Rust API and wait 60 seconds to compile all code
- Verify that the API is running
> This step is important to determine if the API is not active, which would cause the workflow to fail
- Setup Node.js
- Install Newman ( Postman CLI )
- Run the Postman Collection ( rust-api-example/collection.json )
