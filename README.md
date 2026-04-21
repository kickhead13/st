# shtrack

## Dependencies
`shtrack` uses `scp` for copying the files from the remote.

## Usage
1. Create a topic.
```sh
shtrack topic chores
```
2. Add a todo.
```sh
shtrack todo chores Clean up the house.
```
3. Add more todo's.
```sh
shtrack todo chores Take the trash out.
shtrack todo chores Go buy some cat food.
```
4. Add another topic and another todo.
```sh
shtrack topic dev
shtrack todo dev Fix shtrack: make it work with note taking.
```
5. Listall your to-do's:
```sh
shtrack list
chores
  [ ] Clean up the house.  
  [ ] Take the trash out.
  [ ] Go buy some cat food. 
dev
  [ ] Fix shtrack: make it work with note taking.
```
6. Set one of the items as done:
```sh
shtrack done chores 1
shtrack list
chores
  [ ] Clean up the house.  
  [X] Take the trash out.
  [ ] Go buy some cat food. 
dev
  [ ] Fix shtrack: make it work with note taking.
```
7. Add this line to your `~/.bashrc` to see all your to-do's everytime you open it:
```sh
shtrack list
```

## Installing
Currently you can only install the "app" from source. You must run this command in the root of ther repository:

```sh
cargo install --path .
```

