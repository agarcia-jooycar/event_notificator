pipeline {
  agent {
    docker {
      image 'rust:latest'
    }
  }

    stages {
        stage('Build') {
            steps {
               sshagent ( ['jarvis-jooycar']) {
                 sh "cargo build --release"
               }
            }
        }
        stage('Test') {
            steps {
                sh "cargo test"
            }
        }
        stage('Clippy') {
            steps {
                sh "rustup component add clippy"
                sh "cargo clippy"
            }
        }
        /*stage('Rustfmt') {
            steps {
                // The build will fail if rustfmt thinks any changes are
                // required.
                sh "rustup component add rustfmt"
                sh "cargo fmt -- --check"
            }
        }*/
        stage('Doc') {
            steps {
                sh "cargo doc --no-deps"
                step([$class: 'JavadocArchiver',
                      javadocDir: 'target/doc',
                      keepAll: false])
            }
        }
    }
}
