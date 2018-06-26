pipeline {
    agent {
        docker { image 'rust:latest' }
    }
    triggers {
        pollSCM('* * * * *')
    }
    stages {
        stage('Build') {
            steps {
                sh 'cargo build'
            }
        }
        stage('Run') {
            steps {
                sh 'cargo run'
            }
        }
    }
}