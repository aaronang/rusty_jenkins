pipeline {
    agent {
        docker { image 'rust:latest' }
    }
    triggers {
        pollSCM('* * * * *')
    }
    options {
        buildDiscarder(logRotator(numToKeepStr: '1', artifactNumToKeepStr: '1'))
    }
    stages {
        stage('Run') {
            steps {
                sh 'cargo run'
            }
        }
    }
}