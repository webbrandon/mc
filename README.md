**2.0.0-beta**   
![Master Of Ceremony](./docs/header.png)
  
**About**  
Master Of Ceremony is a script and template configuration management utility.  The purpose of this tool is to simplify management of continuous development and integration pipelines while decoupling steps from services architectures that can become bottlenecks or blockers to software deployment and testing. Configure service pipelines into a packages. Call pipelines for a service through these packages locally or in an existing continuous development and integration service.

## Usage
**Api Definitions**
Api's are called from the client or configuration file in YAML format. 

|Type|Descriptions|Default File|
|:---|:---|:---|
|mc|This is the master api. It will accept each api as a specification(ie: `specs`). When multiple api are found this api is ran.|mc.yaml|
|mc&#x2011;repo|Configure version controlled repository source. Will clone only at the moment.|mc&#x2011;repo.yaml|
|mc&#x2011;env|Load dot env files to the running shell for ENVIRONMENT values.|mc&#x2011;env.yaml|
|mc&#x2011;prompts|Run ENVIRONMENT value prompt to seed current shell.|mc&#x2011;prompts.yaml|
|mc&#x2011;flows|Configured flows to be used with the `mc-steps` api.|mc&#x2011;flows.yaml|
|mc&#x2011;steps|Configure steps that runs scripts and template handlers. Use reserved step flows for default order to process or assign `order` to each step.|mc&#x2011;steps.yaml|
|mc&#x2011;templates|Combine a template with a parameters to generate a file. |mc&#x2011;templates.yaml|  
  
**Api Configuration Files**
When running from configuration files MC will run in the directory executed as the base path.  When multiple configuration files are present they will be combined.  Combined configuration will never overwrite `mc` master configurations. Configuration api's are loaded with the defined default file. _(Note: Configuration files and also be `.yml`)_

**Commandline Options & Flags**
```bash
USAGE:
    mc [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -m, --mute         Silence output.
    -n, --no-prompt    Turn prompt off for mc.yaml steps.
    -h, --help         Prints help information

OPTIONS:
    -c, --config <file>                  Sets the "mc.yaml" file to use.
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

