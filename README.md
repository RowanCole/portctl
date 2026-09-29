# portctl

Cross-platform tool for inspecting and managing port usage, supporting Linux / macOS / Windows.

- **find**: find processes occupying a given port
- **kill**: kill processes occupying a given port
- **list**: list all port usage on the system
- Supports wildcards, IPv6, and TCP/UDP protocol filtering; results are printed as a table
- Output language (Chinese / English) selectable via Cargo features

## Installation

Install from [crates.io](https://crates.io/crates/portctl):

```bash
cargo install portctl
```

Or build and install from source:

```bash
git clone https://github.com/RowanCole/portctl
cd portctl
cargo install --path .
```

The binary is installed to `~/.cargo/bin/portctl` (make sure `~/.cargo/bin` is in your PATH).

### Output language

The output language is selected at compile time via Cargo features (`zh` and `en` are mutually exclusive):

```bash
cargo install portctl                                        # Chinese (default)
cargo install portctl --no-default-features --features en    # English

# when building from source
cargo build                                                  # Chinese (default)
cargo build --no-default-features --features en              # English
```

## Usage

### Command format

```
portctl find <ip:port>[/proto]
portctl kill <ip:port>[/proto]
portctl list
```

- **Protocol**: one of `TCP` / `UDP` / `*` (case-insensitive); `*` matches both protocols
- **Omitting the protocol**: `/proto` may be omitted, or written as `/` (`*:8080/`), which is equivalent to `*`
- **Omitting the IP**: `:3000` is equivalent to `*:3000` and matches all local IPs
- **Wildcard `*`**:
  - For ip: matches all local IPs (`0.0.0.0`, `127.0.0.1`, LAN IPs, IPv6)
  - For port: matches any port in 0-65535
  - For protocol: matches both TCP and UDP
- **IPv6**: the address must be wrapped in brackets, e.g. `[::1]:8080`

### find — locate occupying processes

```bash
portctl find :3000                # all IPs' port 3000, shorthand for *:3000
portctl find *:8080               # port 8080 on all IPs, TCP+UDP together
portctl find *:8080/TCP           # TCP only
portctl find *:8080/UDP           # UDP only
portctl find 127.0.0.1:8080/TCP   # specific IP + specific protocol
portctl find '[::1]:8080/UDP'     # IPv6 address
```

Output (English build):

```
PID     PROCESS  PROTO  LOCAL          REMOTE     STATE
------  -------  ----   -------------  ---------  ------
199261  python3  TCP    0.0.0.0:18096  0.0.0.0:0  LISTEN
```

UDP has no notion of "remote address" or "state"; those columns are filled with `-`.

### kill — kill occupying processes

```bash
portctl kill *:8080               # kill all TCP+UDP occupants
portctl kill *:8080/TCP           # kill TCP occupants only
portctl kill 127.0.0.1:8080/UDP   # kill UDP occupants of a specific IP only
```

Example output:

```
Killed PID=199261 process=python3
```

When a port is occupied by multiple processes, they are killed one by one; exit code is 0 if all succeed, otherwise each failure (e.g. insufficient permissions) is reported and the command exits with a non-zero code.

### list — list all ports

```bash
portctl list
```

Prints all current TCP/UDP connections and listeners on the system, in the same format as find, sorted by protocol and local address.

### Shell wildcard caveats

A bare `*` in the shell may be expanded to file names in the current directory:

```bash
portctl find *:8080        # safe: *:8080 matches no file names, passed as-is
portctl kill *:8080        # same as above
portctl find *:8080/*      # unsafe: the trailing * may be expanded; prefer quoting
portctl find '*:8080'      # quoting is always the safest habit
```

### "Not found" message format

When no process is found, the target in the message is normalized: a wildcard IP is shown as `0.0.0.0`, and a specified protocol is appended as a suffix:

```
$ portctl find *:57336/*
No process found occupying 0.0.0.0:57336
$ portctl find *:57336/TCP
No process found occupying 0.0.0.0:57336/TCP
```

## Permissions

- Viewing the PID / process name of processes owned by other users, or killing them, may require **root / administrator** privileges; with insufficient permissions the corresponding fields show `-` or the kill fails with an error
- kill uses `SIGKILL` (Linux/macOS) / `TerminateProcess` (Windows), which cannot be intercepted by the target process

## Platform support

| Platform | Port enumeration   | Process name lookup          | Kill                |
| -------- | ------------------ | ---------------------------- | ------------------- |
| Linux    | procfs / netlink   | `/proc/{pid}/comm`           | `kill(SIGKILL)`     |
| macOS    | libproc            | `libproc::proc_pid::name`    | `kill(SIGKILL)`     |
| Windows  | iphlpapi           | `QueryFullProcessImageNameW` | `TerminateProcess`  |

Built on top of [netstat2](https://crates.io/crates/netstat2); basic port information can be viewed without administrator privileges.
