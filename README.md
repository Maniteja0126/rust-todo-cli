
---

## **Rust Task Manager CLI**

A simple **command-line application** built in **Rust** to manage tasks. This tool allows you to **add, update, complete, list, and delete tasks** easily. Tasks are stored in a **JSON file** for persistence.

---

### **Features**
- Add new tasks.
- Update existing tasks.
- Mark tasks as completed.
- List all tasks with their status.
- Delete tasks by index.

---

### **Prerequisites**
- **Rust**: Ensure you have **Rust and Cargo** installed.  
  You can install Rust from [here](https://www.rust-lang.org/tools/install).

---

### **Setup and Installation**

1. **Clone the repository**:
   ```bash
   git clone git@github.com:Maniteja0126/rust-todo-cli.git
   cd rust-todo-cli
   ```

2. **Build the project**:
   ```bash
   cargo build
   ```

3. **Run the project**:
   ```bash
   cargo run
   ```

---

### **Usage**

Here’s how you can use the various commands available in the Task Manager CLI:

#### **1. Add a New Task**
```bash
cargo run -- add "Go to the gym"
```
- **Description**: Adds a new task with the given description.
- **Example**:
  ```
  Task added successfully!
  ```

#### **2. Update an Existing Task**
```bash
cargo run -- update <index> "New Task Description"
```
- **Description**: Updates the task at the given index with a new description.
- **Example**:
  ```bash
  cargo run -- update 1 "Go for a run"
  Task updated successfully!
  ```

#### **3. Mark a Task as Completed**
```bash
cargo run -- complete <index>
```
- **Description**: Marks the task at the specified index as completed.
- **Example**:
  ```bash
  cargo run -- complete 1
  Task marked as completed!
  ```

#### **4. List All Tasks**
```bash
cargo run -- list
```
- **Description**: Displays all tasks with their status (Pending/Done).
- **Example Output**:
  ```
  Tasks:
  1: Go to the gym [Pending]
  2: Read a book [Done]
  ```

#### **5. Delete a Task**
```bash
cargo run -- delete <index>
```
- **Description**: Deletes the task at the given index.
- **Example**:
  ```bash
  cargo run -- delete 1
  Task deleted successfully!
  ```

---

### **Project Structure**

```
.
├── src/
│   └── main.rs         # Main source code for the CLI app
├── task.json           # JSON file to store tasks
├── Cargo.toml          # Rust package configuration
└── README.md           # Documentation for the project
```

---

### **Error Handling**

- If you try to access an **invalid index** (like a task that doesn’t exist), the CLI will print:
  ```
  Invalid task number!
  ```
- If the **task.json** file is missing or corrupted, the application will initialize with an empty task list.

---

### **Summary**

This Task Manager CLI is a lightweight and easy-to-use tool to keep track of your tasks. With commands to **add, update, complete, list, and delete tasks**, it helps you stay organized directly from the command line. Happy task managing! 🎯