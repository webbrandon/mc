**2.0.0-beta**   
![Master Of Ceremony](./docs/header.png)
  
**About**  
Master Of Ceremony is a script and template configuration management utility.  The purpose of this tool is to simplify management of continuous development and integration pipelines while decoupling steps from services architectures that can become bottlenecks or blockers to software deployment and testing. Configure service pipelines into a packages. Call pipelines for a service through these packages locally or in an existing continuous development and integration service.

## Usage
### Api Configuration Files
When running from configuration files MC will run in the directory executed as the base path.  When multiple configuration files are present they will be combined.  Combined configuration will never overwrite `mc` master configurations. Configuration api's are loaded with the defined default file. _(Note: Configuration files and also be `.yml`)_


### Commandline Options & Flags
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
---

### Api Definitions
Api's are called from the client or configuration file in YAML format. 

|Type|Descriptions|Default File|
|:---|:---|:---|
|mc|This is the master api. It will accept each api as a specification(ie: `specs`). When multiple api are found this api is ran.|mc.yaml|
|[mc&#x2011;repo](#mc-repo)|Configure version controlled repository source. Will clone only at the moment.|mc&#x2011;repo.yaml|
|mc&#x2011;env|Load dot env files to the running shell for ENVIRONMENT values.|mc&#x2011;env.yaml|
|[mc&#x2011;prompts](#mc-prompts)|Run ENVIRONMENT value prompt to seed current shell.|mc&#x2011;prompts.yaml|
|[mc&#x2011;flows](#mc-flows)|Configured flows to be used with the `mc-steps` api.|mc&#x2011;flows.yaml|
|mc&#x2011;steps|Configure steps that runs scripts and template handlers. Use reserved step flows for default order to process or assign `order` to each step.|mc&#x2011;steps.yaml|
|[mc&#x2011;templates](#mc-templates)|Combine a template with a parameters to generate a file. |mc&#x2011;templates.yaml| 

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
- **Type**: suggestion(default), option, boolean
- **Default**: The default option for the user if they only press the "Enter" key.
- **Context**: How you want the question to be asked.
- **Options**: Possible or set options for a suggestion or option type.
  
---

#### mc-repo
Set the repository your pipeline works with. This will clone and enter the repository by default.  You can also define a `path` to enter once cloned.

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
You can segment your steps into flow patterns to represent a pipelines sequence of steps.

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
    - post-script 
  - name: deploy
    flow:
    - functional-test
    - template
    - deploy-script 
    - system-test 
```
_mc-flows.yaml format for setting flow patterns._  

**Selecting a flow.**
This api needs to be used in conjunction with a `mc-steps` configuration.  You call it using the option flow option.
```
mc --flow=build
```  

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

