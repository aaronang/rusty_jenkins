# Rusty Jenkins

>This repository is an experiment to observe and document the behavior or Jenkin's build discarder.

The default implementation of Jenkin's `BuildDiscarder` is called [`LogRotator`](https://github.com/jenkinsci/jenkins/blob/master/core/src/main/java/hudson/tasks/LogRotator.java).
The log rotator accepts four arguments: `daysToKeepStr`, `numToKeepStr`, `artifactDaysToKeepStr`, `artifactNumToKeepStr`.

According to the [documentation](https://github.com/jenkinsci/jenkins/blob/22aa2e6e766074d11249893e3f35e0b99e20d3d0/core/src/main/java/hudson/tasks/LogRotator.java#L55-L77) the arguments have the following intentions.

`daysToKeepStr`: History is only kept up to this days.<br>
`numToKeepStr`: Only this number of build logs are kept.<br>
`artifactDaysToKeepStr`: Artifacts are only kept up to this days.<br>
`artifactNumToKeepStr`: Only this number of builds have their artifacts kept.



```groovy
pipeline {
    agent any
    options {
        buildDiscarder(logRotator(numToKeepStr: '10', artifactNumToKeepStr: '2'))
    }
    stages {
        stage('Run') {
            steps {
                sh 'cargo run'
            }
        }
    }
}
```
