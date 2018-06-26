pipeline {
    agent {
        docker { image 'rust:latest' }
    }
    options {
        buildDiscarder(logRotator(numToKeepStr: '1'))
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
