# How are jobs run?
Every job is scheduled to run on a separate thread. The `TimeLoop` thread connects to the job threads using  `std::sync::mpsc::channel`s. So when the `stop` method is called, the `TimeLoop` thread sends a `()` to all the job threads and they exit.

# How does a function become a job?
`proc_macro` is used to support a custom attribute `job` that annotates a free function. The custom attribute simply modifies the function body by enclosing it with an infinite loop. It also adds a `thread::sleep` statement and uses the attribute param as a seconds value to sleep for the given duration. Also, code to check if there is any message on the channel using the `try_recv()` non-blocking call is added after the call to `thread::sleep`.

The function itself doesn't need to have an explicit param of type `Receiver<()>`. It is automatically added during the processing of the AST.

# Future improvements
Support for `async` functions.
