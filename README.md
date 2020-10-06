# test_pipeline

Master branch

* Require status checks to pass before merging
* Require signed commits

Develop branch

* Require status checks to pass before merging
* Require signed commits
* Require linear history


Create PR against branch `develop`, then it can be `merged` into master, doing
this helps to keep branch "even"

Creating the zip:

     zip -r add.zip codedeploy appspec.yml target/release/test_pipeline

Create random key:

    openssl rand -base64 32
