# Company Management CLI

Welcome to the Company Management CLI! This Rust program helps you manage departments and employees in your company through a simple command-line interface.

## Features

- Add employees to departments
- List employees in a specific department
- List all departments and their employees

## How to Use

1. Compile and run the program using Rust's cargo system:
   ```
   cargo run
   ```

2. Once the program is running, you'll see a `>` prompt. You can enter the following commands:

   - Add an employee to a department:
     ```
     Add [name] to [department]
     ```
     Example: `Add John to Engineering`

   - List employees in a specific department:
     ```
     List [department]
     ```
     Example: `List Engineering`

   - List all departments and their employees:
     ```
     List all
     ```

3. The program will continue running until you exit it (use Ctrl+C to quit).

## Example Usage

```
> Add John to Engineering
Employee John added to Engineering

> Add Alice to Marketing
Employee Alice added to Marketing

> List Engineering
Employees in Engineering:
John

> List all
Department - Engineering
John
Department - Marketing
Alice

> List Sales
No employees in - Sales
```

## Technical Details

This program is written in Rust and uses the following key components:

- A `Company` struct to represent the company, using a `HashMap` to store departments and their employees.
- The `std::io` module for handling user input and output.
- Pattern matching to process user commands.

Feel free to explore and modify the code to suit your specific needs. Happy managing!