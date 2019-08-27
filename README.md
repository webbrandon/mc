**2.0.0-beta**   
![Master Of Ceremony](./docs/header.png)
  
**Continuous Integration and Continuous Delivery Orchestration Framework**

The purpose of this tool is to simplify management of continuous integration and delivery pipelines.  Configure service pipelines into configuration packages.  Call pipelines for a service through these packages locally or in an existing continuous integration and delivery service.

**Features**
- Configure individual pipeline steps.
- Use shell script of any kind. (eg. bash, python, node.js, ...)
- Assign steps into pipeline flows.
- Built in templating engine.
- Customize prompting for collecting environment values.
- Git repository handling.
- Assign dotenv files to pipeline.
- Run flows in containerized environments. **beta**
- Deploy in existing CI/CD systems.

## Usage

### Command Line Options & Flags
```bash
USAGE:
    mc [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -m, --mute         Silence script and template processing output.
    -n, --no-prompt    Turn prompt off for steps.
    -h, --help         Prints help information

OPTIONS:
    -c, --config <file>                  Sets the MC api configuration file to use.
    -e, --env <env>                      Load from .env file.
    -f, --flow <flow>                    Use flow pattern from mc.yaml.
    -r, --repo <repo>                    Clone git repository.
    -s, --script <script>                Sets the script to run the the start.
    -t, --template <template>            Sets a custom template file.
    -o, --template-out <template_out>    Rendered template out file write location.
    -p, --param <param>                  Sets a custom template parameters file.
    -l, --post-script <post_script>      Sets the script to run at the end.

SUBCOMMANDS:
    completions    Completion scripts for various shells.
    create         Create api files.
```  

### Api Definitions
Api's are called from the client or configuration file in YAML format. 

|Type|Descriptions|Default File|
|:---|:---|:---|
|[mc](#mc)|This is the master api. It will accept each api as a specification(ie: `specs`). When multiple api are found this api is ran.|mc.yaml|
|[mc&#x2011;repo](#mc-repo)|Configure version controlled repository source. Will clone only at the moment.|mc&#x2011;repo.yaml|
|[mc&#x2011;env](#mc-env)|Load dot env files to the running shell for ENVIRONMENT values.|mc&#x2011;env.yaml|
|[mc&#x2011;prompts](#mc-prompts)|Run ENVIRONMENT value prompt to seed current shell.|mc&#x2011;prompts.yaml|
|[mc&#x2011;flows](#mc-flows)|Configured flows to be used with the `mc-steps` api.|mc&#x2011;flows.yaml|
|[mc&#x2011;steps](#mc-steps)|Configure steps that runs scripts and template handlers. Use reserved step flows for default order to process or assign `order` to each step.|mc&#x2011;steps.yaml|
|[mc&#x2011;templates](#mc-templates)|Combine a template with a parameters to generate a file. |mc&#x2011;templates.yaml| 

### Api Configuration Files
When running from configuration files MC will run in the directory executed as the base path.  When multiple configuration files are present they will be combined.  Combined configuration will never overwrite `mc` master configurations. Configuration api's are loaded with the defined default file. _(Note: Configuration files and also be `.yml`)_

```bash
  ls ~/my-pipeline/
    mc.yaml
    mc-flows.yaml
    mc-repository.yaml
```

Running `mc` from `my-pipeline` directory will merge the three configurations together.  the `mc.yaml` file cannot be overwritten with configurations in other apis.

Individual apis can be used in standalone mode and can be very use in daily operations like the `mc-templates` and `mc-prompts` api.  

```bash
  ls ~/templates/
    mc-templates.yaml
```

Running `mc` from `templates` directory will generate any template files configured.  

### Api Settings


#### mc
This api is a handler for all api configurations.  It can be configured as a single api or will generate when multiple api configuration files exist together(ie: `mc-steps.yaml` and `mc-flows.yaml`).

**Running**  
```bash
mc -c test.mc.yaml
```  
_Use configuration file outside the current execution directory or default file discovery names._

**mc-env.yaml**   
```YAML
---
api: mc
version: "2.0"
specs:
  repository:
    url: ...
  env-file:
    path: .env
  env-prompt: 
    - ...
  flows:
    - ...
  steps:
    ...
```

---

#### mc-steps 
There is a set of reserved step names that MC knows how to handle in the absence of mc-flows api or `order` field. Creating custom steps is also possible. There is no limit to the number custom configured steps. There must be at least one step configured.
    
Currently the reserved steps are presented below in the default order they are ran when present:
- pre
- unit-test
- build
- functional-test
- template
- deploy
- system-test
- post

Steps can apply the `mc-env`, `mc-prompts` and `mc-templates` api.  The `mc-templates` api will be applied as the reserved _template_ and its default flow position.

**Using scripts**  
You can use any shell available to the operating environment.  Be sure to set the shebang in the first line of a script to specify the shell you are targeting. 

Recommendation:
- Use exit codes to identify failures in a script. 
- Apply the Unix Philosophy and keep scripts modular and stateless.

**Commandline with mc-steps**  
```bash
mc -e ... -r ... -s ... -t ... -p ... -o ... -l ...
```  
_Use MC client to set a single but full featured step._

```bash
mc -m
```  
_Silence step processing output._

**mc-steps.yaml**   
```YAML
---
api: mc-steps
version: "2.0"
specs:
  pre:
    env-prompt:
      - name: INPUT_OPTION
        kind: input
        context: 'My question for you is ?.'
    script: test.sh
  build:
    script: test.sh
    templates:
    - name: deploy
      template: template.tmpl
      params: params.json
      out-file: template.bash  
    post-script: test2.sh
  my-custom-step:
    script: cleanup.sh
```

---

#### mc-container
Run your step flow configurations inside a container if you want to provide a way to guarantee scripts run consistently.  Currently docker is only supported.   **Still under beta usage only and may not contain all stated features.**

```bash
mc -i mc-slave:latest
```
_Use the no-prompt option to bypass prompting. You must account for empty values if not set with `mc-env` api._

**mc-container.yaml**   
```YAML
---
api: mc-container
version: "1.0-beta"
specs:
  docker:
    image: mc-slave:latest
    volumes:
      - "/opt/secrets"
```

---

#### mc-prompts
Instead of loading default through the `mc-env` api you may want to ask for values at run time.  Using `mc-prompts` and `mc-env` together will give default value selections to each prompt question when possible.  Control or enhance the prompt process safely.  Find more instructions below.

```bash
mc --no-prompt
```
_Use the no-prompt option to bypass prompting. You must account for empty values if not set with `mc-env` api._

**mc-prompts.yaml**   
```YAML
---
api: mc-prompts
version: "2.0"
specs:
  - name: OPTION
    kind: option
    default: "Option One"
    options:
    - value: "Option One"
    - value: "Option Two"
  - name: QUESTION
    context: "How do you like how this works?"
    kind: "input"
  - name: BOOLEAN
    context: "Are you being honest?"
    kind: boolean
```
_Prompts can be set before all pipeline steps or only when a step is ran._

Each entry has several options you can set.
- **Name**: The name of the environment value. (i.e. USER_VALUE)
- **Kind**: suggestion(default), option, boolean
- **Default**: The default option for the user if they only press the "Enter" key.
- **Context**: How you want the question to be asked.
- **Options**: Possible or set options for a suggestion or option type.
  
---

#### mc-env 
You can set ENVIRONMENT values using dot env files for the running shell using this api.

**Setting mc-env**  
```bash
mc -e .env
```  
_Use MC client to set .env instead of config files also._

**mc-env.yaml**   
```YAML
---
api: mc-env
version: "2.0"
specs:
  path: .env
```

---

#### mc-repo
Set the service repository your pipeline works with. This will clone and enter the repository by default.  You can also define a `path` to enter once cloned.

```bash
mc -r https://github.com/webbrandon/mc.git
```  
_Use the client to clone a repository instead of config files._

**mc-repo.yaml**   
```YAML
---
api: mc-repo
version: "2.0"
specs:
  url: https://github.com/webbrandon/mc.git
  path: "mc/docs"
```

---

#### mc-flows
You can segment your steps into flow patterns to represent a pipelines sequence of steps.  The `mc-env` api can be added to individual flows. When applying this `mc-env` to a flow it will take priority and overwrite any global configuration.

**Selecting a flow.**
This api needs to be used in conjunction with a `mc-steps` configuration.  You call it using the option flow option.
```bash
mc --flow=build
```  

**mc-flows.yaml**   
```YAML
---
api: mc-flows
version: "2.0"
specs:
  - name: build
    flow:
    - unit-test
    - build-script 
    - my-custom-step 
  - name: deploy
    flow:
    - functional-test
    - template
    - deploy-script 
    - system-test 
```
_mc-flows.yaml format for setting flow patterns._  

---

#### mc-templates 
We have based the template api on Handlebars.js.  Currently this api accepts JSON params body for `params` only. Learn more on how to write Handlebar templates from the [documentation](https://handlebarsjs.com)

```bash
mc -t template.tmpl -p params.json -o template.out
```  
_You could use mc only for the template engine._

**mc-templates.yaml**   
```YAML
---
api: mc-templates
version: "2.0"
specs:
  - name: deploy
    template: template.tmpl
    params: params.json
    out-file: template.bash  
```

## Build From Source
If you are building from source you will need to have the [Rust language ](https://rustup.rs/) application suite installed and download the [source code](https://webbrandon.github.io/mc).  I have built and tested for linux and OSX only.  If you try on Windows please let me know how it goes.

Stable build last compiled with with [Rust version 1.36.0](https://rustup.rs/).

```
cargo build --release
cargo install --force
```


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

