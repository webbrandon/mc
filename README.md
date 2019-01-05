# Master of Ceremony

**About**  
The purpose of this tool is to simplify management of continuous development and integration pipelines while decoupling them from any one service like Jenkins.  Using modern practices of configuration management you can begin building software, parameterize templates to deploy or record activity and deploy and release software with a single command that will run cross platform (Depending how well you can script).

This is a build tool similar to other build tools.  Except it's on steroids!

**Features**  
- Static configuration file with Makeconfig.yaml. (Optional)
- Command line options override config files.
- Built in template engine using JSON params file. ([learn more on templating](https://handlebarsjs.com))
- Controlled process flow. e.g. build > kubernetes template > deploy for testing > send traffic
- Script steps in any shell available. (i.e. bash, sh, zsh, node)
- No prompt for automation or prompted for local user response and control. 
- Runs on Darwin(Mac) and Debian(Linux).
- Configurable prompting for environment values with default setting and option suggestions.

## USAGE

If no file is assigned to a step the cli will look in its current base path for a **Makeconfig.yaml**.  Best practice is to create modular and stateless scripts for each step. 

![Master OF Ceremony Cli Tool](sample/cli-demo.gif)  
_When you use option parameters they will override the existing Makeconfig.yaml configuration._

**Cli Options** (Use individually or conjunction w/ Makeconfig.yaml)   
```
USAGE:
    mc [FLAGS] [OPTIONS]

FLAGS:
    -h, --help           Prints help information
    -m, --mute           Silence output.
        --no-build       Skip build step from Makeconfig.
        --no-deploy      Skip deploy step from Makeconfig.
        --no-post        Skip post build step from Makeconfig.
        --no-prompt      Turn prompt off for Makeconfig steps.
        --no-template    Skip template step from Makeconfig settings.

OPTIONS:
    -b, --build-script <BUILD_SCRIPT>      Sets the script file to use for setting building software.
    -f, --file <CONFIG>                    Sets the "Makeconfig" file to use.
    -d, --deploy-script <DEPLOY_SCRIPT>    Sets the script file to use after _build script_.
    -e, --env <ENV>                        Load from .env file.
    -c, --param-script <PARAM_SCRIPT>      Sets a custom script to configure parameters file at render time.
    -p, --param <PARAM>                    Sets a custom template parameters file.
    -s, --post-script <POST_SCRIPT>        Sets the script file to use after configuring template.
    -t, --template <TEMPLATE>              Sets a custom template file
    -o, --template-out <OUT>               Rendered template out file write location.
```
---    
    
**Makeconfig.yaml**   
```YAML
api: makeconfig
version: alpha/v1
metadata:
  name: mc
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

## License  
WFYW (Whatever <s>Fuck</s> You Want) 1.0
