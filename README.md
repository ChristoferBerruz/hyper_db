# CS 257 - Database Project

The following repository is a monorepo for the database
project at SJSU.

## Organization
There are two main directories: `optim` and `io_tool`.
`io_tool` contains all the python source code to push
I/O onto the database.

`optim` contains all the [PGRX](https://github.com/pgcentralfoundation/pgrx) source
code. We are using PGRX to build a custom Postgres extension overriding
hooks and defining a new data type.

## Tooling installation

If you want to install PGRX locally, you can follow in the [install instructions](https://github.com/pgcentralfoundation/pgrx?tab=readme-ov-file#system-requirements). Note that MacOS users
require a little bit of a more complicated setup while Linux (and WSL2 users) have a better
time.

For the python side of this application, we will be using [Poetry](https://python-poetry.org/docs/#installation) which makes managing python environment and dependencies easier. Note that this makes `io_tool` a python package and all source
code is therefore located at `io_tool/io_tool/` while Python tests are located
at `io_tool/tests/`