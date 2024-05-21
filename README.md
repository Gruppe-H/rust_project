# rust_exam

### Connect to MongoDB

Run these:
```
export MONGODB_URL=mongodb://localhost:27017
export MONGODB_DATABASE=rust   
export MONGODB_COLLECTION=users
```


### To build the code

```
cargo build --release
```
You should see this:
```
 .....
 Compiling bson v1.2.4
 Compiling mongodb v1.2.5
 Compiling user-app v0.1.0 (/Users/demo/mongodb-rust)
 Finished release [optimized] target(s) in 37.84s
```
Change directory:
```
cd target/release
```

## CRUD operations
Now you can perform the CRUD operations.


### Create

When you want to create a new user, there's two ways to do so. You choose which way by the command-line flag, but must choose one of the two available options.

#### Create One
```
./rust_project create -u '{"name":"John","age":56,"email":"john@cphbusiness.dk","password":"123","username":"johnny"}'
```
Creates a single user based on the JSON data given in the command-line. The JSON must contain name, age, email, password, and username. The flag can be either long: --user or short as in the example.

#### Create Many from File
```
./rust_project create -f "C:\Users\Caroline\Documents\Software-Udvikling\Rust\rust_project\test.txt"

```
Creates multiple users based on the JSON data found in a file, which path is given in the command-line. The JSON must contain name, age, email, password, and username. The flag can be either long: --file_path or short as in the example.

### Read

#### Read All
```
./rust_project read
```
Prints out all users in the database

#### Read by Name Filter
```
./rust_project read -n "John"
```
Prints out all users in the database that match the name filter. The flag can be either long: --name or short as in the example.

### Update
```
./rust_project update --id "66499ede27684323baab7e84" -u '{"name":"John","age":56,"email":"john@example.dk","password":"Secur@Pass398","username":"johnny"}'

```
Finds the user by the '_id' (ObjectId) and then updates that user with the new JSON data provided in the command-line. The JSON must contain name, age, email, password, and username. Both flags must be present, and the id flag can anly be the long version as in the example, and the user flag can be either long: --user or short as in the example.

### Delete
```
./rust_project delete --id "663b6a67565aa05f353de166"
```
Finds the user by the '_id' (ObjectId) and then deletes that user. The flag can only be the long version as in the example.






