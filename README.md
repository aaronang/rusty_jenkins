# Rusty Jenkins

This repository is an experiment to observe the behavior or Jenkin's build discarder.

The default implementation of Jenkin's `BuildDiscarder` is called [`LogRotator`](https://github.com/jenkinsci/jenkins/blob/master/core/src/main/java/hudson/tasks/LogRotator.java).
The log rotator accepts four arguments: `daysToKeepStr`, `numToKeepStr`, `artifactDaysToKeepStr`, `artifactNumToKeepStr`.

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
