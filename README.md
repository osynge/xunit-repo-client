# xunit-repo-client

## Introduction

This is the command line client for the project [xunit-repo](https://github.com/osynge/xunit-repo). The client is intended to read junit/[xunit2](https://xunit.net/) style files and upload the to a server.

xunit-repo-client can be configured with enviroment variables, configuration files, or command line arguments, in order from lowest to highest presidence. It is expected to be used either on the developers desktop, or as part of a continuous intergration, continuous deployment framework such as jenkins or drone.

## Example usage

Typically `xunit-repo-client` will be used as part of script.

An example bash script using only enviroment variables for linux.
```
#!/bin/bash
# Set enviroment
export BRANCH=`git branch | grep '*' | cut -d ' ' -f 2`
export PLATFORM_ID=`lsb_release --id | sed -e 's/.*:\t//'`
export PLATFORM_RELEASE=`lsb_release --release | sed -e 's/.*:\t//'`
# Set enviroment keys to read.
export XRC_ENVIROMENT=BRANCH:PLATFORM_ID:PLATFORM_RELEASE

# Set Project Identifier
export XRC_PROJECT_IDENTIFIER=`git config --get remote.origin.url`
# Set Run Identifier
export XRC_RUN_IDENTIFIER=`git rev-parse HEAD`

# Set Service URL
export XRC_SERVICE_URL=http://127.0.0.1:8888
# Glob Xunit files from tests and xml directories.
export XRC_XUNIT=tests/xunit*.xml:xml/xunit*.xml

# Now run the client
xunit-repo-client
```

An example bash script using arguments for linux.

```
#!/bin/bash
# Set enviroment
export BRANCH=`git branch | grep '*' | cut -d ' ' -f 2`
export PLATFORM_ID=`lsb_release --id | sed -e 's/.*:\t//'`
export PLATFORM_RELEASE=`lsb_release --release | sed -e 's/.*:\t//'`

xunit-repo-client \
    --env BRANCH \
    --env PLATFORM_ID \
    --env PLATFORM_RELEASE \
    --project-identifier `git config --get remote.origin.url` \
    --run-identifier `git rev-parse HEAD` \
    --url http://127.0.0.1:8888 \
    --xunit tests/xunit*.xml \
    --xunit xml/xunit*.xml
```
## Configuring xunit-repo-client.

There are 4 type of confiuration.

* Service connection.
  * Service url
* Test run configuration.
  * Run Identifier
  * Run Key
* Test enviroment configration.
  * Enviroment
  * Environment Key
* Project configuration.
  * Project Identifier
  * Project Human Name
  * Project Key

**Service connection configuration** is required to connect the client to the server.

**Test run configuration** is specific to the test run.

**Test enviroment configration** will be the same for many test runs, and allows test runs to be compared. Every test run is in an enviroment, and may include details of the platform, such as the architecture, or code branch.

**Project configuration** will be the same for a project that will run in one or more enviroments. Typically this is related to a single version controled project.

### `Configuration file`

Is set by the enviroment varable `XRC_CONFIG` or the command line argument `--config` and has the value of the path to the configuration file to be used.


### Table of xunit-repo-client configuration.

Setting | Type | Environment variable | Configuration parameter | Command line argument
------- | ---- | -------------------- | ----------------------- | ---------------------
Configuration file | String | XRC_CONFIG ||--config
loglevel| Int | | loglevel | -v --verbose -q --quiet
xunit glob | StringList | XRC_XUNIT | xunit | xunit
Enviroment | StringList | XRC_ENVIROMENT | enviroment_keys | --env
Environment Key | String | XRC_ENVIROMENT_KEY | enviroment_sk | --environment-key
Project Key | String | XRC_PROJECT_KEY | project_sk |
Project Identifier |String | XRC_PROJECT_IDENTIFIER | project_identiifier | project-identifier
Project Human Name | String | XRC_PROJECT_NAME | project_human_name | project-name
Run Identifier | String | XRC_RUN_IDENTIFIER | |run-identifier
Run Key | String | XRC_RUN_KEY | | run-key
Service Url | String | XRC_SERVICE_URL | service_url | url