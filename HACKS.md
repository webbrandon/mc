# MC Hacks!
There are lots of ways you can use MC besides CI/CD orchestration.  Below find some of the use cases others have done and how it has helped. Have a use case you want to share?

**!!!EXAMPLE FILES COMING SOON!!!**

## Templating
**Situation:**
  You have a templated document that needs to be generated ADHOC but only needs certain fields replaced.
  
**Solution:**
  Use `mc-templates` to help generate these templates and use `mc-prompts` or `mc-env` in a step to quickly parameterize the replacement fields.


## Prompting Generation
**Situation:**  
  New developer starts the job and has been handed a computer and install script with all the software already will need.  Now the only thing that user needs to do is add their permissions and credentials the software which can be picked up in an environment variable.
  
**Solution:**
  Use the `mc-prompt` api to configure question that help speed this process up and nothing is missed so the new developer has the confidence to keep pushing forward.

