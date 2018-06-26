pipeline {
    agent {
        docker { image 'rust:latest' }
    }
    triggers {
        pollSCM('* * * * *')
    }
    options {
        buildDiscarder(logRotator(artifactNumToKeepStr: '1'))
    }
    stages {
        stage('Run') {
            steps {
                sh 'cargo run'
            }
        }
    }
}