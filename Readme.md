# Rust ORM Example with Procedural Macro

This project demonstrates a basic implementation of a procedural macro in Rust to create a lightweight Object-Relational Mapping (ORM) system. Using the `#[Entity]` macro, you can define database entities and automatically generate methods like `table_name()` to map structs to database table names.

---

## Features

- **Custom Table Name Annotations**:
  - Use `#[EntityName = "table_name"]` to specify a custom table name.
  - Defaults to the struct name if no custom name is provided.

- **Simple and Extendable**:
  - Built using `proc-macro`, `quote`, and `syn`.
  - Designed for demonstration purposes and can be extended for more advanced ORM features like query building.

---

## File Structure

```plaintext
.
├── my_orm_macros/      # Procedural macro crate
│   ├── Cargo.toml      # Macro dependencies
│   └── src/
│       └── lib.rs      # Macro implementation
└── my_orm_demo/        # Main project using the macro
    ├── Cargo.toml      # Main project dependencies
    └── src/
        └── main.rs     # Example usage of the ORM macro
```

---

## Example Usage

### Defining an Entity

In the `my_orm_demo` project, define a struct and annotate it with `#[Entity]`:

```rust
use my_orm_macros::Entity;

#[derive(Entity)]
#[EntityName = "users"]
struct User {
    id: i32,
    name: String,
}

fn main() {
    println!("Entity name: {}", User::table_name());
}
```

### Running the Example

1. Navigate to the main project directory:
   ```bash
   cd my_orm_demo
   ```

2. Build and run the project:
   ```bash
   cargo run
   ```

3. Output:
   ```plaintext
   Entity name: users
   ```

---

## Getting Started

### Prerequisites

- Install [Rust](https://www.rust-lang.org/tools/install).

### Setting Up the Project

1. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/rust-orm-example.git
   cd rust-orm-example
   ```

2. Build and run the example:
   ```bash
   cd my_orm_demo
   cargo run
   ```

---

## Extending the Macro

You can extend the procedural macro in `my_orm_macros` to include additional features, such as:

- Query builders
- Validation annotations
- Custom field mappings

Refer to the [Rust procedural macro documentation](https://doc.rust-lang.org/book/ch19-06-macros.html) for guidance.

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

--- 

## Contributions

Contributions are welcome! Feel free to fork the repository and submit pull requests.

---

## Acknowledgments

Special thanks to the Rust community and the creators of `proc-macro`, `quote`, and `syn` crates for making macros easy to implement.
