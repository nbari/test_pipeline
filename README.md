# test_pipeline

Master branch

* Require status checks to pass before merging
* Require signed commits
* Require linear history

Develop branch

* Require status checks to pass before merging
* Require signed commits
* Require linear history
* Allow force pushes

Keep develop in sync with master:

    git pull origin master --rebase

to reset branch to be like master:

    git reset --hard origin/master


Creating the zip:

     zip -r add.zip codedeploy appspec.yml target/release/test_pipeline

Create random key:

    openssl rand -base64 32

Adding self-hosted runners
https://docs.github.com/en/free-pro-team@latest/actions/hosting-your-own-runners/adding-self-hosted-runners

    nohup ./run.sh &
