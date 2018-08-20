# Master of Ceremony

**About**   
This is a build tool similar to other build tools like `make` and `go`'s `template` package but bundled into one.  What it does is provide modern way of configuring and running your build scripts.  Either by setting the command line options or via `Makeconfig` file you can assign your build takes.  Your build can begin with configuration script which can create the parameters needed to fill in a template (or not) and finish off with a post initialization script with the use of a single command; `mc`.

## USAGE

If no `config-file` is assigned `mc` will look in its current base path for a `Makeconfig` from which it is ran; with or without the _yaml_ mime type.  When you use option parameters they will override the existence of a `Makeconfig`.  

---

If you would like to use the config option and keep it to a single command line you can use the bellow listed options to set your build.

**Command Line**   
```bash
mc [OPTIONS]

FLAGS:
    -h, --help    Prints help information

OPTIONS:
    -c, --config-file <CONFIG>         Sets the "Makeconfig" file to use.
    -p, --params <PARAMS>              Sets a custom template parameters file.
    -S, --post-script <POST_SCRIPT>    Sets the script file to use after configuring template.
    -s, --script <SCRIPT>              Sets the script file to use for setting template parameters.
    -t, --template <TEMPLATE>          Sets a custom template file
    -o, --template-out <OUT>           Rendered template out file write location.
```
---    
To make use of a single file(`Makeconfig`) and single command(`mc`) to control you build process you can use the below bootstrap example(documentation to come).  
    
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
    file: ./sample/sample.params
  template-out:
    file: sample.out
  script:
    file: ./sample/sample.script
  post-script:
    file: ./sample/sample.post-script
```

## License  
WFYW (What Ever The <s>Fuck</s> You Want) 1.0