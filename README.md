# Master of Ceremony

**About**  
The purpose of this tool is to simplify management of continuous development and integration pipelines while decoupling steps from services architectures that can become bottlenecks or blockers to software deployment and testing.  This tool helps configure CI/CD steps into a package that can be used for existing CI/CD services or to process locally with a user at the helm. 

**Features**  
- Static configuration file with mc.yaml. (Optional)
- Command line options override config files.
- Built in Handlebars template engine using JSON params file. ([learn more on templating](https://handlebarsjs.com))
- Controlled process flow. e.g. build > kubernetes template > deploy for testing > send traffic
- Script steps in any shell available. (i.e. bash, sh, zsh, node)
- No prompt for automation or prompted for local user response and control. 
- Runs on Darwin(Mac) and Debian(Linux).
- Configurable prompting for environment values with default setting and option suggestions.

## USAGE

If no file is assigned to a step the cli will look in its current base path for a **mc.yaml**.  Best practice is to create modular and stateless scripts for each step. 

![Master OF Ceremony Cli Tool](sample/cli-demo.gif)  
_When you use option parameters they will override the existing mc.yaml configuration._

**Cli Options** (Use individually or conjunction w/ mc.yaml)   
```
USAGE:
    mc [FLAGS] [OPTIONS]

FLAGS:
    -h, --help           Prints help information
    -m, --mute           Silence output.
        --no-build       Skip build step from mc.yaml.
        --no-deploy      Skip deploy step from mc.yaml.
        --no-post        Skip post build step from mc.yaml.
        --no-prompt      Turn prompt off for mc steps.
        --no-template    Skip template step from mc.yaml settings.

OPTIONS:
    -b, --build-script <BUILD_SCRIPT>      Sets the script file to use for setting building software.
    -f, --file <CONFIG>                    Sets the "mc.yaml" file to use.
    -d, --deploy-script <DEPLOY_SCRIPT>    Sets the script file to use after _build script_.
    -e, --env <ENV>                        Load from .env file.
        --param-script <PARAM_SCRIPT>      Sets a custom script to configure parameters file at render time.
    -p, --param <PARAM>                    Sets a custom template parameters file.
    -s, --post-script <POST_SCRIPT>        Sets the script file to use after configuring template.
    -t, --template <TEMPLATE>              Sets a custom template file
    -o, --template-out <OUT>               Rendered template out file write location.
```
---    
    
**mc.yaml**   
```YAML
api: mc
version: alpha/v1
metadata:
  name: "Master Of Ceremony"
  description: "A long sentence..."
specs:
  build-script:
    file: ./sample/sample.build-script
    env:
    - name: ENVIRONMENT
      default: stg
      options:
      - value: stg
      - value: prod
  template:
    file: ./sample/sample.template
    parameters: ./sample/sample.params
    script: ./sample/sample.params-script
    outfile: sample.out
    env:
    - name: HAS_THIS
      default: "true"
  deploy-script:
    file: ./sample/sample.deploy-script
  post-script:
    file: ./sample/sample.post-script
```

## Install
You can build from source or with the provided install script for pre built binaries.
```
curl https://webbrandon.github.io/mc/install.sh -sS | bash -s
```

## Build From Source
If you are building from source you will need to have the [Rust language ](https://rustup.rs/) application suite installed and download the [source code](https://webbrandon.github.io/mc).  I have built and tested for linux and OSX only.  If you try on Windows please let me know how it goes.

Stable build last compiled with with [Rust version 1.31.1](https://rustup.rs/).

```
cargo build --release
cargo install --force
```

## Future  
- Run In Docker Container  
- Jenkins Integrations (Under Planning)
- Remote Files Over HTTP

Unlicense (Public Domain)
============================

This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or
distribute this software, either in source code form or as a compiled
binary, for any purpose, commercial or non-commercial, and by any
means.

In jurisdictions that recognize copyright laws, the author or authors
of this software dedicate any and all copyright interest in the
software to the public domain. We make this dedication for the benefit
of the public at large and to the detriment of our heirs and
successors. We intend this dedication to be an overt act of
relinquishment in perpetuity of all present and future rights to this
software under copyright law.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.

For more information, please refer to &lt;<http://unlicense.org/>&gt;
