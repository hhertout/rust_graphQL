
## Sea ORM

```bash
sea-orm-cli generate entity -o src/entity --seaography --model-extra-derives GraphQLObject
```


## Dev specs

#### Requirements:

- Without docker : Node js & npm

To run the app : 
- ```cargo run```

To watch change in your code, run 
- ```npx nodemon --watch src -e rs --exec cargo run```.