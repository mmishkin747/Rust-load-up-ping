# Rust-load-up-ping
This is CLI program for loud up channel by ping. Usage for cisco

## Usage

```sh
rfup -h
```
>Rust load Up by Ping
>
>Usage: rlup [OPTIONS] <ADDRESS_SERVER> <ADDRESS_HOST>
>
>Arguments:
>  <ADDRESS_SERVER>  Network ipv4 address server
>  <ADDRESS_HOST>    Network ipv4 address host
>
>Options:
>  -u, --user <USER>                    User's name for connecting ups
>  -p, --password <PASSWORD>            Password for connecting ups
>      --port <PORT>                    Network port to use [default: 23]
>  -c, --count-session <COUNT_SESSION>  Count session [default: 1]
>  -t, --time-out <TIME_OUT>            Timeout for write/right, sec [default: 2]
>  -m, --mtu <MTU>                      Size MTU [default: 1500]
>  -r, --repit <REPIT>                  Count repit ping [default: 1000]
>  -h, --help                           Print help information
>  -V, --version                        Print version information
>


for exaples:

```sh
rfup 192.168.2.1 192.168.2.2 -u admin -p "passw" -c 2 -r 1000 -m 1400
```

result:

>Success rate is 100 percent (1000/1000), round-trip min/avg/max = 1/1/2 ms
>Success rate is 100 percent (1000/1000), round-trip min/avg/max = 1/1/2 ms
