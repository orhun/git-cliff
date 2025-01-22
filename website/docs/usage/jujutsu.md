---
sidebar_position: 12
---

# Jujutsu

You can use with a repository that has been cloned using [jujutsu](https://jj-vcs.github.io/jj/latest/).

## Colocated

If the repository was cloned by `jujutsu` using the `--colocate` option, then all you need to do is make sure that
you have checked out your mainline branch using git.
If you don't, then you will likely see an error about an unborn branch.

## Non-colocated

If the repository was cloned by `jujutsu` but _not_ using the `--colocate` option,
then the Git repository, normally the `.git` directory, is located in `.jj/repo/store/git`

Create a file in the root of your repository that tells Git, and `git-cliff` where the Git repository is
and update the `HEAD` to point to your main remote branch:

e.g.:

```bash
jj git clone https://github.com/orhun/menyoki
cd menyoki
echo "gitdir: .jj/repo/store/git" > .git
echo "ref: refs/remotes/origin/master" > .jj/repo/store/git/HEAD
```

N.B.: Replace `master` in the last command with the name of your main remote branch. e.g. `main`, `trunk`, etc.
