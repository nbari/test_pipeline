# test_pipeline

Master branch

* Require status checks to pass before merging
* Require signed commits

Develop branch

* Require status checks to pass before merging
* Require signed commits
* Require linear history

Creating the zip:

     zip -r add.zip codedeploy appspec.yml target/release/test_pipeline

Create random key:

    openssl rand -base64 32
