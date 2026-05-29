# Git Basics

Git is a distributed version control system.

## Commands

### git init
Initialises a new repository in the current directory.

### git add
Stages changes for the next commit.

### git commit
Records staged changes to the repository history.

### git push
Sends local commits to a remote repository.

### git pull
Fetches and integrates changes from a remote repository.

## Troubleshooting

### rejected push
If your push is rejected with non-fast-forward, run:
git pull --rebase origin main

### merge conflict
Edit the conflicted file, remove conflict markers, then:
git add <file>
git rebase --continue
