# Rusty Jenkins

>This repository is an experiment to observe and document the behavior or Jenkin's build discarder.

The default implementation of Jenkin's `BuildDiscarder` is called [`LogRotator`](https://github.com/jenkinsci/jenkins/blob/master/core/src/main/java/hudson/tasks/LogRotator.java).
The `LogRotator` accepts four arguments: `daysToKeepStr`, `numToKeepStr`, `artifactDaysToKeepStr`, `artifactNumToKeepStr`.
According to the [documentation](https://github.com/jenkinsci/jenkins/blob/22aa2e6e766074d11249893e3f35e0b99e20d3d0/core/src/main/java/hudson/tasks/LogRotator.java#L55-L77), the arguments have the following semantics.

`daysToKeepStr`: History is only kept up to this days.<br>
`numToKeepStr`: Only this number of build logs are kept.<br>
`artifactDaysToKeepStr`: Artifacts are only kept up to this days.<br>
`artifactNumToKeepStr`: Only this number of builds have their artifacts kept.

The parameter names and documentation may seem obvious but can be deceptive.
For example, when `numToKeepStr` is set to 10, every branch and pull request allows 10 builds to persist.
Therefore, given a repository with three branches:
```bash
$ git branch
  development
  documentation
* master
```
The maximum allowable number of persisted builds is `3 * 10 = 30`.

## Deep Dive

Assuming Jenkins was installed according to the ["Installing Jenkins" guide](https://jenkins.io/doc/book/installing/), Jenkins stores build logs and artifacts in the location `/var/jenkins_home/jobs/<repository>/branches`.
For each branch and pull request Jenkins creates a directory to store build logs and artifacts.
For example, given the branches `master`, `development`, `documentation`, and one pull request, Jenkins will have the following directory structure:
```bash
$ pwd
/var/jenkins_home/jobs/<repository>

$ tree branches
branches
├── PR-1
├── development
├── documentation
└── master
```

Inside one of the directories, the tree structure looks something like:
```bash
$ pwd
/var/jenkins_home/jobs/<repository>/branches

$ tree master
master
├── builds
├── config.xml
├── lastStable
├── lastSuccessful
├── name-utf8.txt
├── nextBuildNumber
├── scm-polling.log
└── scm-revision-hash.xml
```
