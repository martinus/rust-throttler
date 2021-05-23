# rust-throttler
Limits the number of concurrent applications, e.g. gcc builds.


# What architecture?

Let's try TCP, because why not:

## Server

* Manages execution order and delays
* Provides statistics/logging


## Client

* Connects to server
* Waits until it is supposed to run
* Runs the command

