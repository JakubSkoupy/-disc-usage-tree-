# disk usage tree 
This is my first rust app, It's main use is for me to get familiar with the language

CLI application which prints a file tree and information about their size. Combination of
du and tree.

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
