**2.0.0-beta**   
![Master Of Ceremony](./docs/header.png)
  
**About**  
Master Of Ceremony is a script and template configuration management utility.  The purpose of this tool is to simplify management of continuous development and integration pipelines while decoupling steps from services architectures that can become bottlenecks or blockers to software deployment and testing. Configure service pipelines into a packages. Call pipelines for a service through these packages locally or in an existing continuous development and integration service.

## MC Api's
Api's are called from the client or configuration file in YAML format. When running from configuration file it run in the directory MC is executed.  Configuration files with be combined but will never overwrite a master service configuration `mc`. 

|Type|Descriptions|File|
|:---|:---|:---|
|mc|This is the master api. It will accept each api as a specification(ie: `specs`). When multiple api are found this api is ran.|mc.yaml|
|mc&#x2011;repo|Configure version controlled repository source. Will clone only at the moment.|mc&#x2011;repo.yaml|
|mc&#x2011;env|Load dot env files to the running shell for ENVIRONMENT values.|mc&#x2011;env.yaml|
|mc&#x2011;prompts|Run ENVIRONMENT value prompt to seed current shell.|mc&#x2011;prompts.yaml|
|mc&#x2011;flows|Configured flows to be used with the `mc-steps` api.|mc&#x2011;flows.yaml|
|mc&#x2011;steps|Configure steps that runs scripts and template handlers. Use reserved step flows for default order to process or assign `order` to each step.|mc&#x2011;steps.yaml|
|mc&#x2011;templates|Combine a template with a parameters to generate a file. |mc&#x2011;templates.yaml|

## Usage

#### Api Configuration Files



