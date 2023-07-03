# disk usage tree 
This is my first rust app, It's main use is for me to get familiar with the language

CLI application which prints a file tree and information about their size. Combination of
du and tree.

```
USAGE: ./dtree [ OPTIONS ] [ PATH ]

[ OPTIONS ]

-- sort   [ "name" | "size" ]     Sorts output by given criteria.

-- depth  [ depth ]               Limits printout to specified depth
                                  Only affects the printout, the sizes are calculated
                                  to full depth.

-- percent                        displays percentage of space taken (does not work yet).

-- decimal                        displays sizes in decimal units {MB, GB..} instead
                                  of multiples of 1024 {MiB, GiB..}.

-- nocolor                        Disables text coloring.

-- quiet                          Supresses error messages.

-- verbose                        (does not work yet).
```

EXAMPLE [ --sort size --depth 2 ]
```
 844.51 MiB   .
 844.07 MiB   ├── target
 465.53 MiB   │   ├── debug
 270.32 MiB   │   ├── release
 108.21 MiB   │   ├── doc
   1.64 KiB   │   ├── .rustc_info.json
   218.00 B   │   ├── .rustdoc_fingerprint.json
   177.00 B   │   └── CACHEDIR.TAG
 413.26 KiB   ├── .git
 327.30 KiB   │   ├── objects
  26.89 KiB   │   ├── hooks
  25.36 KiB   │   ├── logs
  20.12 KiB   │   ├── refs
   4.23 KiB   │   ├── info
   4.00 KiB   │   ├── branches
   813.00 B   │   ├── index
   314.00 B   │   ├── config
   102.00 B   │   ├── FETCH_HEAD
    73.00 B   │   ├── description
    41.00 B   │   ├── ORIG_HEAD
    23.00 B   │   ├── HEAD
    18.00 B   │   └── COMMIT_EDITMSG
  14.89 KiB   ├── src
   3.48 KiB   │   ├── options.rs
   3.20 KiB   │   ├── print.rs
   2.78 KiB   │   ├── tree.rs
   891.00 B   │   ├── data_functions.rs
   573.00 B   │   └── main.rs
   9.66 KiB   ├── Cargo.lock
   960.00 B   ├── README.md
   259.00 B   ├── Cargo.toml
     8.00 B   └── .gitignore

```
