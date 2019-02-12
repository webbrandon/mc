# Master of Ceremony

**About**  
The purpose of this tool is to simplify management of continuous development and integration pipelines while decoupling steps from services architectures that can become bottlenecks or blockers to software deployment and testing.  This tool helps configure CI/CD steps into a package that can be used for existing CI/CD services or to process locally with a user at the helm. 

**Features**  
- Built in _Handlebars_ template engine using JSON params file. (_[learn more](#templating)_)
- Defined pipeline steps. (_[learn more](#pipeline-steps)_)
- Script pipeline steps in any shell available. (_[learn more](#scripts)_)
- Runs on Darwin(_Mac_) and Debian(_Linux_). (_[see releases](https://github.com/webbrandon/mc/releases)_)
- Configurable prompting for environment values. (_[learn more](#prompts)_)
- Custom flow pattens to segment steps together. (_[learn more](#flows)_)

## USAGE

If no file is assigned to a step the cli will look in its current base path for a **mc.yaml**.  Best practice is to create modular and stateless scripts for each step. 

![Master OF Ceremony Cli Tool](sample/cli-demo.gif)  
_When you use option parameters they will override the existing mc.yaml configuration._

**Cli Options** (Use individually or conjunction w/ mc.yaml)   
```
USAGE:
    mc [FLAGS] [OPTIONS]

FLAGS:
    -h, --help                  Prints help information
    -m, --mute                  Silence output.
        --no-build              Skip build step from mc.yaml.
        --no-deploy             Skip deploy step from mc.yaml.
        --no-functional-test    Skip functional testing step from mc.yaml.
        --no-post               Skip post build step from mc.yaml.
        --no-pre                Skip pre step from mc.yaml.
        --no-prompt             Turn prompt off for mc.yaml steps.
        --no-system-test        Skip system testing step from mc.yaml.
        --no-template           Skip template step from mc.yaml settings.
        --no-unit-test          Skip unit testing step from mc.yaml.

OPTIONS:
    -b, --build-script <BUILD_SCRIPT>          Sets the script file to use for setting building software.
    -f, --file <CONFIG>                        Sets the "mc.yaml" file to use.
    -d, --deploy-script <DEPLOY_SCRIPT>        Sets the script file to use after _build script_.
    -e, --env <ENV>                            Load from .env file.
        --flow <FLOW_NAME>                     Use flow pattern from mc.yaml.
    -n, --functional-test <FUNCTIONAL_TEST>    Sets the script file to use for running functional test.
        --param-script <PARAM_SCRIPT>          Sets a custom script to configure parameters file at render time.
    -p, --param <PARAM>                        Sets a custom template parameters file.
    -s, --post-script <POST_SCRIPT>            Sets the script file to use after configuring template.
    -c, --pre-script <PRE_SCRIPT>              Sets the script that run before all other scripts.
        --repo <REPO>                          Clone git repository.
    -y, --system-test <SYSTEM_TEST>            Sets the script file to use for running system test.
    -t, --template <TEMPLATE>                  Sets a custom template file
    -o, --template-out <OUT>                   Rendered template out file write location.
    -u, --unit-test <UNIT_TEST>                Sets the script file to use for running unit test.
```
---    
    
**mc.yaml**   
```YAML
api: mc
version: alpha/v2
metadata:
  name: "Master Of Ceremony"
  description: "A long sentence..."
specs:
  env:
  - name: GLOBAL
    default: global-value
  flows:
  - name: remove-deploy
    flow:
    - post-script
  steps:
    build-script:
      file: sample.build-script
      env:
      - name: SUGGESTION
        default: Two
        options:
        - value: One
        - value: Two
    template:
      file: sample.template
      parameters: sample.params
      script: sample.params-script
      outfile: sample.template-out
    deploy-script:
      file: sample.deploy-script
    post-script:
      file: sample.post-script
```

### Pipeline Steps
Steps are currently limited to a defined set.  We do plan on enhancing this down the line.  Currently we have defined what will possibly become the reserved step types that will have special feature integrations in future releases.
    
Currently the reserved steps are presented below in the default order they are ran when present:
- pre-script
- unit-test
- build-script
- functional-test
- template
- deploy-script
- system-test
- post-script

#### Scripts
Each pipeline step will run a script.  You can link to script anywhere on the host system but we recommend having placing script in a subpath to you mc.yaml file.  Be sure to set the shebang in the first line of a script to specify the shell you are targeting. 

### Templating 
We have based out built in template engine on Handlebars.js.  Currently we only accept JSON param body but have plans for other integrations. Learn more on how to write Handlebar templates from the [documentation](https://handlebarsjs.com)

```bash
mc -t template.tmpl -p params.json -o template.out
```  
_You could use mc only for the template engine._

**mc.yaml (template options)**   
```YAML
...
specs:
  steps:
    template:
      file: template.tmpl
      parameters: params.json
      script: template.bash
      outfile: template.out
    ...
```

Setting the template pipeline step is slightly different as you will target a Handlebars template with the `file` parameter.  
- **File**: Handlebars template.
- **Parameters**: JSON param body for template values.
- **Script**: Optional script file to run prior to applying the template.
- **Outfile**: The output file from template.

### Prompts
Instead of loading default values you may want to ask for values at run time, especially if not being ran through automation.  Environment values can be set using mmc's configurable prompt. Find more instructions below.

```bash
mc --no-prompt
```
_Use the no-prompt option to bypass prompting. You must account for empty values if not set from .env file._

**mc.yaml (formatting options)**   
```YAML
...
specs:
  env:
  - name: GLOBAL
    default: global-value
  steps:
    build-script:
      env:
      - name: SUGGESTION
        default: Two
        options:
        - value: One
        - value: Two
        - value: "Anything else."
      - name: OPTION
        type: option
        options:
        - value: "Option One"
        - value: "Option Two"
      - name: QUESTION
        context: "How do you like how this works?"
        default: "I like it!"
        options:
        - value: "I like it!"
        - value: "Not sure yet."
        - value: "This is wierd..."
      - name: BOOLEAN
        context: "Are you being honest?"
        type: boolean
      ...
```
_Prompts can be set before all pipeline steps or only when a step is ran._

Each entry has several options you can set.
- **Name**: The name of the environment value. (i.e. USER_VALUE)
- **Type**: suggestion(default), option, boolean
- **Default**: The default option for the user if they only press the "Enter" key.
- **Context**: How you want the question to be asked.
- **Options**: Possible or set options for a suggestion or option type.

### Repository Settings
Set the repository your pipeline works with. This will clone and enter the repository.

**mc.yaml (repository options)**   
```YAML
...
specs:
  repository:
    url: https://github.com/webbrandon/mc.git
  ...
```

Currently we only have one parameter `url` for repository remote host.  These options are expected to grow.


### Flows
You can segment your steps into flow patterns.  This is useful if you find yourself often using many options like `--no-build` and others when reading from mc.yaml file contents.


**mc.yaml (spec.flows)**   
```YAML
...
specs:
  flows:
  - name: build
    env-file: .env
    flow:
    - build-script 
    ...
```
_mc.yaml format for setting flow patterns._  

You call it using the option flow option.
```
mc --flow=build
```

## Install
You can build from source or with the provided install script for pre built binaries.
```
curl https://webbrandon.github.io/mc/install.sh -sS | bash -s
```

## Build From Source
If you are building from source you will need to have the [Rust language ](https://rustup.rs/) application suite installed and download the [source code](https://webbrandon.github.io/mc).  I have built and tested for linux and OSX only.  If you try on Windows please let me know how it goes.

Stable build last compiled with with [Rust version 1.32.0](https://rustup.rs/).

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
