Program Proxy
=============

The program acts as a proxy for another program or script.

When a service enables extension through an executable, but only supports executables and does not support scripts.

This program proxies stdin, stdout, stderr and allows customization of current dir, environment and arguments to another program, typically a script.

Examples
--------

Rename the program-proxy.exe to eg. 'MyApp.exe' and create a text file with the following contents as 'MyApp.cfg' next to it:

```
program=.venv\Scripts\python.exe
current_dir=
arg=script.py
```

When 'MyApp.exe' is executed it will launch a python process with arguments `script.py` followed by any command line arguments given to 'MyApp.exe'. The current directory is set to the directory of 'MyApp.exe'.

All relative paths are resolved relative to the location of the program proxy's directory.

The final program path is `$DIR\.venv\Scripts\python.exe` (where `$DIR` is the location of the proxy's directory) with arguments [`script.py`, `$ARGS`...] where `$ARGS`... are all the arguments given to the proxy program.

Configuration
-------------

The configuration file has the same name as the executable. Eg. if the executable is renamed to 'MyApp.exe' then the cfg file must be named 'MyApp.cfg' next to the executable.

The configuration is a sequence of lines with the following structure:

* Empty lines and lines starting with `#` are ignored.

* `program=$PROGRAM`

  Required. The cfg must specify the path to the program to run as its first line. If the path is relative it will be made absolute relative to this proxy's directory.

* `current_dir=$CURRENT_DIR`

  Specifies the current directory to use when running the program. If the path is relative it will be made absolute relative to this proxy's directory. Optional. If left out the current directory is untouched. Can appear at most once in the cfg.

* `arg=$ARG`

  Insert the argument before the arguments provided by the caller. Each argument line adds exactly one argument.

* `env=$KEY=$VALUE`

  Adds an environment variable `$KEY` set to `$VALUE` before running the program. Each env line adds exactly one environment variable.

* `env_remove=$KEY`

  Removes an environment variable before running the program. Each env_remove line removes exactly one environment variable.

* `env_clear`

  Clears the environment variables before running the program.

License
-------

Licensed under [MIT License](https://opensource.org/licenses/MIT), see [license.txt](license.txt).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, shall be licensed as above, without any additional terms or conditions.
