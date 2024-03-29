# RustShell

RustShell is very simple a command line program that allows you to perform various tasks using commands on your computer.

## Features

-   `AntiVM` : RustShell includes a very simple AntiVM
-   `Hide Window` : There is an "option" to hide the cmd window when you start RustShell
-   `Encryptor` : RustShell also has a simple, but powerful encryptor
-   `Cmd commands` : You can use cmd commands and arguments in RustShell (you can also use powershell commands by typing a "one liner" e.g. `powershell sl C:\`)

## Upcoming Features and updates

-   `A Listener` : With a listener you can send commands to the RustShell.exe and execute them remotely

## Commands

RustShell offers a number of commands that you can use to interact with your computer:

-   `tree` : list all files and folders in the specified directory in a tree-like way
-   `clear` : clear the command prompt
-   `find` : search for a file or folder
-   `where` : prints the current directory of the RustShell
-   `scan` : scans the C: drive and saves every path except the blacklisted in a text file in 'C:\files\files.txt'
-   `kill` : kills any process running using the PID
-   `encrypt` / `decrypt`: encrypts / decrypts the specified file
-   `info` : gives info on the computer (disks, total memory, used memory, system type, system version, etc.)
-   `help` : display a list of available commands
-   `exit` : exits the program

## Usage

To run RustShell, simply run the following command file using rust or cargo:
rust:

```
$ rustc main.rs
$ ./main
```

cargo:

```
$ cargo run
```

If you don't have rust or cargo installed on your system you can follow [this tutorial](https://doc.rust-lang.org/cargo/getting-started/installation.html)

## How to improve functions

`encrypt` : at the moment the function deletes the original file and replaces it with the encrypted file. I think it would be better to alter the data inside the original file.

`scan` : at the moment the function saves the file locations in a text file. It should be faster to save it to memory.