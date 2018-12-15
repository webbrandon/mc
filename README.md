# Master of Ceremony

**About**   
This is a build tool similar to other build tools.  It's like `make` on steriods with features like built in dynamic templating similiar to `go`'s template package.  What it provides is a modern way of configuring and running your build scripts with a single command.  Either by setting the command line options or via `Makeconfig.yaml` file you can assign your build task.  Your build can begin with a configuration script which can create the parameters needed to fill in a template (or not) and finish off with a post initialization script in one step for the user.

## Install
You can build from source or with the provided install script for pre built binaries.
```bash
curl https://webbrandon.github.io/mc/install.sh -sS | bash -s
```

## USAGE

If no `file` is assigned `mc` will look in its current base path for a `Makeconfig.yaml` from which it is ran.  When you use option parameters they will override the existence of a `Makeconfig.yaml`.  

---

If you would like to use the config option and keep it to a single command line you can use the bellow listed options to set your build.

**Command Line**   
```bash
USAGE:
    mc [FLAGS] [OPTIONS]

FLAGS:
    -h, --help    Prints help information
    -m, --mute    Silence output.

OPTIONS:
    -b, --build-script <BUILD_SCRIPT>      Sets the script file to use for setting building software.
    -f, --file <CONFIG>                    Sets the "Makeconfig" file to use.
    -d, --deploy-script <DEPLOY_SCRIPT>    Sets the script file to use after _build script_.
    -c, --param-script <PARAM_SCRIPT>      Sets a custom script to configure parameters file at render time.
    -p, --param <PARAM>                    Sets a custom template parameters file.
    -s, --post-script <POST_SCRIPT>        Sets the script file to use after configuring template.
    -t, --template <TEMPLATE>              Sets a custom template file
    -o, --template-out <OUT>               Rendered template out file write location.
```
---    
To make use of a single file(`Makeconfig.yaml`) and single command(`mc`) to control you build process you can use the below bootstrap example(documentation to come).  
    
**Makeconfig**   
```YAML
api: makeconfig
version: beta/1
metadata:
  name: mc
  description: "A long sentence..."
specs:
  template:
    file: ./sample/sample.template
  parameters:
    type: File
    path: ./sample/sample.params
    create: ./sample/sample.params-script
  template-out:
    file: sample.out
  build-script:
    file: ./sample/sample.script
  post-script:
    file: ./sample/sample.post-script
```

## Build From Source
If you are building from source you will need to have the [Rust language ](https://rustup.rs/) application suite installed and download the [source code](https://webbrandon.github.io/mc).  I have built and tested for linux and OSX only.  If you try on Windows please let me know how it goes.

Stable build last compiled with with [Rust version 1.31.0](https://rustup.rs/).

```bash
cargo build
cargo install --force
```

## License  
WFYW (Whatever <s>Fuck</s> You Want) 1.0
