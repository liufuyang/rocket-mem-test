## To test if this program is using more and more memory:

```
cargo run
```

Open Activity Monitor to check the memory usage, it starts around 1MB.

Then visit `http://localhost:8000/`.

Afterwards keep refreshing on `http://localhost:8000/`, try wait for 10 or 20 seconds before each refreshing.

The memory usage goes from 1MB up to 4MB.

I am not sure if keep on refreshing will keep the memory continue to grow.