pipeline {
    agent {
        docker { image 'rust:latest' }
    }
    options {
        buildDiscarder(logRotator(numToKeepStr: '20'))
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