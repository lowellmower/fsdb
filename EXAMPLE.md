```rust
let db = db::new("pet_db", "/tmp")

pub struct Pet {
    name  String
    type  String
    age   u8
    color String    
}

let pet_bucket = Bucket::new(Pet, ["name", "type"])
db::add(pet_bucket)
db::commit()

let mittens = Pet {name: "mittens", type: "cat", age: 11, color: "black and white"}
let pepperoni = Pet {name: "pepperoni", type: "dog", age: 2, color: "brown"}
let cuddles = Pet {name: "cuddles", type: "gold fish", age: 3, color: "gold"}

db::add(mittens)
db::commit()
```
```bash
# list files under the pet db
ls /tmp/pet_db/
.
..
commits/
pet/

# list the files under the bucket made
ls /tmp/pet_db/pet
.
..
0:mittens:cat

# read the entry in the bucket
cat /tmp/pet_db/pet/0:mittens:cat
{
    name: "Pet",
    commit: "c1bedWcC281i",
    data: '{name: "mittens", type: "cat", age: 11, color: "black and white"}',
    time_created: "Mon Mar 25 06:21:09 EDT 2019",
    time_modified: "None",
    assoc: "None",
}
```

```rust
let pets = db::get()::all("pet")
// pets == [Pet{name: "mittens"...}]

let pet = db::get("pet")::where(("name", "mittens"))
// pet == Pet {name: "mittens"...}

db::add([pepperoni, cuddles])
db::del("pet")::where(("age",11))
db::commit()

let pets = db::get()::all("pets")
// pets == [Pet{name: "pepperoni"...}, Pet{name: "cuddles"...}]
```

```bash
# list the files in the bucket
ls /tmp/pet_db/pet
.
..
1:pepperoni:dog
2:cuddles:gold_fish

# see that there is an association between the two objects
cat /tmp/pet_db/pet/2:cuddles:gold_fish
{
    name: "Pet",
    commit: "c2incwQx98vZ",
    data: '{ name: "cuddles", type: "gold fish", age: 3, color: "gold"}',
    time_created: "Mon Mar 25 07:52:09 EDT 2019",
    time_modified: "None",
    assoc: "pepperoni:dog",
}

# see that a commits list is growing
ls -l /tmp/pet_db/commits
.
..
c3incwQx98vZ/
c1bedWcC281i/

# list latest the commit showing links to head and tail
ls -l /tmp/pet_db/commits/c3incwQx98vZ
.
..
1:pepperoni:dog
2:cuddles:gold_fish
head -> .
tail -> /tmp/pet_db/commits/c1bedWcC281i/

# list commit linked as tail to current
ls -l /tmp/pet_db/commits/c1bedWcC281i/
.
..
0:mittens:cat
head -> /tmp/pet_db/commits/c3incwQx98vZ/
tail -> .
```