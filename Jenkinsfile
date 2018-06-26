pipeline {
    agent {
        docker { image 'rust:latest' }
    }
    options {
        buildDiscarder(logRotator(numToKeepStr: '5', artifactNumToKeepStr: '1'))
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
        stage('Release') {
            steps {
                sh 'cargo build --release'
            }
        }
    }

    post {
        always {
            archiveArtifacts artifacts: 'target/release/rusty_jenkins', fingerprint: true
        }
    }
}
