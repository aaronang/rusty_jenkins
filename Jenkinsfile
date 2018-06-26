pipeline {
    agent {
        docker { image 'rust:latest' }
    }
    options {
        buildDiscarder(logRotator(numToKeepStr: '10', artifactNumToKeepStr: '2'))
    }
    triggers {
        pollSCM('* * * * *')
    }
    stages {
        stage('Run') {
            steps {
                sh 'cargo run'
            }
        }
    }
}
