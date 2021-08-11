pipeline {
    environment {
        registry = 'mengelishev/rust_project'
        registryCredential = 'dockerhub_mengelishev'
        dockerImage = ''
    }
    agent any
    stages {
        stage('Cargo build') {
            steps {
                echo 'Cargo BUILD starting'
                echo '----------Cargo Build Start----------'
                sh   'cargo build'
                echo '----------Cargo Build End------------'
                echo 'Cargo build SUCCESS'
            }
        }
        stage('Cargo test') {
            steps {
                echo 'Cargo TEST starting'
                echo '----------Cargo TEST Start-----------'
                sh   'cargo test'
                echo '----------Cargo TEST End-------------'
                echo 'Cargo TEST SUCCESS'
            }
        }
        stage('Docker Build image Dockerfile') {
            steps {
               script {
                   dockerImage = docker.build registry + ':v1'
               }
            }
        }
        stage('Docker push') {
            steps {
                script {
                    docker.withRegistry('', registryCredential) {
                        dockerImage.push()
                    }
                }
            }
        }
    }
}