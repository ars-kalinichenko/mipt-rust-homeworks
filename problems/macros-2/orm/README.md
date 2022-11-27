# Object-relational mapping library

In this project, we'll write an [Object-relational mapping](https://en.wikipedia.org/wiki/Object%E2%80%93relational_mapping) library.

In practice, you'll want to use crate [diesel](https://crates.io/crates/diesel) as your ORM library.

Before continuing, install the SQLite3 driver. On Linux, you need the `libsqlite3-dev` package.

## Motivation

In practical tasks, it often necessary to store data in an external database. Such databases usually store data in the form of tables in relational data bases. However, it's inconvenient to work with tables in program code, we want to work with objects native to the programming language. The idea of an ORM library is to give application's code the ability to work with the data as with objects, abstracting from how the data will be stored in the table.

Advanced ORM libraries support many different database management systems (DBMS) as a backend. Within this project, we will limit library to supporting only one DBMS - SQLite3. However, the architecture of the library will support the ability to add other backends.

## Examples

Let's take a look at a couple of examples from the tests:

```rust
#[derive(Object)]
struct User {
    name: String,
    picture: Vec<u8>,
    visits: i64,
    balance: f64,
    is_admin: bool,
}
```

The `User` structure contains fields of all of the five types that the library will support. `#[derive(Object)]` should implement the `Object` trait from the library for `User`. The trait, as well as the derive macro, you must implement yourself.

In the ORM library, working with the DBMS is only possible within the framework of transactions that are created as follows:

```rust
// Create a connection with DBMS
let mut conn = Connection::open_sqlite_file("/path/to/file").unwrap();
// Create a new transaction
let tx = conn.new_transaction().unwrap();
```

Inside the transaction, we can create an object:

```rust
// Create an object in memory. Currently, this object isn't bounded to transaction
let user = User { /* ... */ };
// Let's create this object in the DBMS as a part of a transaction
let tx_user = tx.create(user).unwrap();
```

The `create` method returns a value of type `Tx<'a, User>`. Semantically, this is an object of type `User` that exists within a transaction. The object is bound to the transaction by the `'a` lifetime, i.e. cannot outlive its transaction.

Each object within a transaction has an identifier:

```rust
let user_id = tx_user.id();
```

In ORM library, identifiers are integers.

Another way to get an object within a transaction is to read it from the database:

```rust
let tx_user = tx.get::<User>(user_id);
```

To read or write the fields of an object within a transaction, we'll implement the `.borrow()` and `.borrow_mut()` methods:

```rust
println!("User name: {}", tx_user.borrow().name);
*tx_user.borrow_mut().visits += 1;
```

It's possible to select the same object from the database twice. In this case, `Tx<...>` objects that the transaction will return will refer to the same object in memory:

```rust
let tx_user = tx.get::<User>(user_id);
let tx_user_2 = tx.get::<User>(user_id);
*tx_user.borrow_mut().balance = 250;
assert_eq!(tx_user_2.borrow().balance, 250);
```

If you'll call `.borrow_mut()` on an object that already have active borrows, the code must panic. Similarly, a panic will occur if `.borrow()` is called on an object that has an active mutable borrow.

Also, if you have an object owned by the transaction, you can delete it:

```rust
tx_user.delete();
```

If the object has active borrows, the code must panic. Also, an attempt to call `.borrow()` or `.borrow_mut()` on an object that is deleted (for example, via `tx_user_2` in the code above) will cause a panic.

To apply all changes within a transaction, you must end it with a call to `tx.commit()`. Calling `tx.rollback()`, on the other hand, will end the transaction by rolling back all changes.

## Table and column names

By default, the table in the DBMS is named using the same name as the object type, and the columns are named using the same name as the object fields. However, table and column names can be changed with the `table_name` and `column_name` attributes on the structure, for example:

```rust
#[derive(Object)]
#[table_name("order_table")]
struct Order {
    #[column_name("IsTall")]
    is_tall: bool,
}
```

## Implementation

### Trait `Object`

The `Object` trait is declared in `src/object.rs`. You need to figure out what this trait will contain by youself. Conceptually, it should include:

- Schema: object type name, table name, list of object fields (for each field - its name, column name and type).
- A way to represent an object as a row in the table.
- A way to instantiate an object from a row in the table.

Note that:

- The `Object` trait won't be object safe almost certainly. However, you will want to use some elements from it in a dynamic context. For this declare a new object safe trait and implement it for all objects that implement `Object`.
- You'll want to downcast from `&dyn Store` to a particular object type. That is why `Object` is declared as `trait Object: Any { /* ... */ }`.

### Interacting with SQL

All SQL work is encapsulated by the `StorageTransaction` trait declared in `src/storage.rs`. The task of `StorageTransaction` is to abstract from specific library for working with storage.

To work with SQLite3, we will use the `rusqlite` library. The `StorageTransaction` trait needs to be implemented for `rusqlite::Transaction`. `rusqlite` examples can be found in its [documentation](https://docs.rs/rusqlite).

Cheatsheet with SQL queries you may find useful:

- Check if table exists:

    ```sql
    SELECT 1 FROM sqlite_master WHERE name = "table_name"
    ```

- Create a table (using the `User` object above as an example):

    ```sql
    CREATE TABLE User(
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT,
        picture BLOB,
        visits BIGINT,
        balance REAL,
        is_admin TINYINT, -- There's no 'bool' type in SQLite3
    )
    ```

- Insert the row into table:

    ```sql
    INSERT INTO table(col1, col2) VALUES(123, 456)
    ```

- Update values in the row:

    ```sql
    UPDATE table SET col1 = 123, col2 = 456 WHERE id = 789
    ```

- Select values in the row:

    ```sql
    SELECT co1, col2 FROM table WHERE id = 123
    ```

- Delete line:

    ```sql
    DELETE FROM table WHERE id = 123
    ```

Note that the `.commit()` and `.rollback()` methods of `rusqlite::Transaction` destroy the transaction object, but the same methods of the `StorageTransaction` trait must retain it. This is due to the requirements of object safety: if `.commit()` destroy the transaction object, it would not be possible to use this trait as `&dyn StorageTranasction`. Therefore, commit and rollback directly via SQL with `COMMIT` and `ROLLBACK` commands.

### Transactions and cache

Each object instantiated within an ORM transaction (not to be confused with a `rusqlite` transaction) must be stored in that transaction's object cache. When you commit a transaction, you must walk through the object cache, check which objects have changed, and apply those changes to the underlying `StorageTransaction` (via the `.update_row()` method). Those objects that have been removed must be removed (`.remove_row()`).

An elegant way to tell if an object has changed is to check whether the `.borrow_mut()` has been called at least once.

Before working with a table of a particular object type, you should first make sure that the table exists. If it doesn't exist, create it. The table is checked solely by name; it is not proposed to check the table schema for compliance with the expected schema.

### Error handling

Errors are declared in `src/error.rs`. Within the framework of the project, we identify five types of errors:

- `NotFound` - The requested object was not found.
- `UnexpectedType` - one of the columns are of type that was not expected by the object.
- `MissingColumn` - one of the expected columns is missing in the table.
- `LockConflict` - the database is locked by a concurrent transaction (SQLite3 locks it entirely).
- `Storage` - any other underlying storage error.

The mapping from `rusqlite` errors to ORM library errors is as follows:

- The error `rusqlite::Error::QueryReturnedNoRows` is `NotFound`.
- The error `rusqlite::Error::InvalidColumnType` is `UnexpectedType`.
- `rusqlite::Error::SqliteFailure` error with code `rusqlite::ErrorCode::DatabaseBusy` is `LockConflict`.
- `rusqlite::Error::SqliteFailire` error containing the text "no such column:" or "has no column named" - is `MissingColumn`.
- Everything else is `StorageError`.

Note that most of these errors contain the context that is not contained in the `rusqlite::Error` (for instance, object of what type and with what identifier we've not found). You might want to create a function that takes a `rusqlite` error and an additional context, and creates an error from ORM library.

## Hints

- When implementing `Tx::borrow` and `Tx::borrow_mut` you may need [Ref::map](https://doc.rust-lang.org/std/cell/struct.Ref.html#method.map) and [RefMut::map](https://doc.rust-lang.org/std/cell/struct.RefMut.html#method.map).
- Begin with the test `create` (it uses functions `tx.create()`, `tx.get()` and `tx.commit()`).
- You shouldn't begin by writing derive macro. It's better to start by manually implementating trait `Object` in `tests/tests.rs`.
- When implementing derive macro, try to split meaningful parts as much as possible. For example, in order to determine the column type by the field type, you can make the trait `AsDataType` with the associated constant `DATA_TYPE` and implement it for `String`, `Vec<[u8]>`, `i64`, `f64` and `bool`. Thus, to determine the type of the column by a field, it is enough to write `<$field_type as orm::AsDataType>::DATA_TYPE`.

## Questions

- How this problem is related to `mini-frunk`? How we can use its ideas here?
